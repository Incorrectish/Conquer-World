use crate::{
    direction::Direction,
    entity::Entity,
    projectile::Projectile,
    tile::{self, PROJECTILE_PLAYER},
    utils::Position,
    utils::Boss,
    world::World,
    BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE,
};
use ggez::graphics::{self, Canvas};
use std::{cmp::max, collections::HashMap, collections::LinkedList};

const CHASING_ENEMY_HEALTH: usize = 50;
const BOMBER_ENEMY_HEALTH: usize = 25;
const KNIGHT_ENEMY_HEALTH: usize = 100;
const SHOOTER_ENEMY_HEALTH: usize = 25;
const MAJOR_ENEMY_HEALTH: usize = 200;
const MINOR_BOSS_HEALTH: usize = 1000;
const MAJOR_BOSS_HEALTH: usize = 2000;

const PERMISSIBLE_TILES: [[f32; 4]; 4] = [
    tile::GRASS,
    tile::PROJECTILE_PLAYER,
    tile::LIGHTNING_SECONDARY,
    tile::LIGHTNING_INITIAL,
];
const PERMISSIBLE_TILES_DODGING: [[f32; 4]; 3] = [
    tile::GRASS,
    tile::LIGHTNING_INITIAL,
    tile::LIGHTNING_SECONDARY,
];
const PERMISSIBLE_TILES_BOSS: [[f32; 4]; 0] = [];

const CHASING_ENEMY_SPEED: usize = 1;
const BOMBER_ENEMY_SPEED: usize = 1;
const KNIGHT_ENEMY_SPEED: usize = 1;
const SHOOTER_ENEMY_SPEED: usize = 1;
const MAJOR_ENEMY_SPEED: usize = 1;
const MINOR_BOSS_SPEED: usize = 1;
const MAJOR_BOSS_SPEED: usize = 1;

const CHASING_ENEMY_ENERGY_RETURN: usize = 3;
const BOMBER_ENEMY_ENERGY_RETURN: usize = 5;
const KNIGHT_ENEMY_ENERGY_RETURN: usize = 12;
const SHOOTER_ENEMY_ENERGY_RETURN: usize = 7;
const MAJOR_ENEMY_ENERGY_RETURN: usize = 25;
const MINOR_BOSS_ENERGY_RETURN: usize = 100;
const MAJOR_BOSS_ENERGY_RETURN: usize = 100;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
// This is basically the same as the enemy for now, but I am just testing an enemy system
pub struct Enemy {
    // This is the position in the form (x, y)
    pub pos: Vec<Position>,

    // The direction that the enemy is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    pub direction: Direction,

    // Just like in player controls the amount of tiles an enemy moves in one "turn"
    pub speed: usize,

    // This is the enemy color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the enemy is represented as a solid color
    pub color: [f32; 4],

    //Enemy attack damage
    pub attack_damage: usize,

    // Stores enemy health: for enemy death and such
    health: usize,

    pub world_pos: Position,

    resistance: f32,

    can_dodge_projectiles: bool,

    is_boss: bool,

    pub movement_cooldown: bool,
}

impl Enemy {
    fn new(
        pos: Vec<Position>,
        speed: usize,
        color: [f32; 4],
        world_pos: Position,
        health: usize,
        can_dodge_projectiles: bool,
        boss: bool,
    ) -> Self {
        let temp = Self {
            pos,
            direction: Direction::North,
            speed: 1,
            color,
            attack_damage: 1,
            health,
            resistance: 1.0,
            world_pos,
            can_dodge_projectiles,
            is_boss: boss,
            movement_cooldown: false,
        };
        temp
    }

    pub fn bomber(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        pos.push(Position::new(x, y));
        Enemy::new(
            pos,
            BOMBER_ENEMY_SPEED,
            tile::BOMBER_ENEMY,
            world_pos,
            BOMBER_ENEMY_HEALTH,
            true,
            false,
        )
    }

    pub fn chasing(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        pos.push(Position::new(x, y));
        Enemy::new(
            pos,
            CHASING_ENEMY_SPEED,
            tile::CHASING_ENEMY,
            world_pos,
            CHASING_ENEMY_HEALTH,
            true,
            false,
        )
    }

    pub fn major_enemy(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                pos.push(Position::new(x+i, y+j));
            }
        }
        Enemy::new(
            pos,
            MAJOR_ENEMY_SPEED,
            tile::MAJOR_ENEMY,
            world_pos,
            MAJOR_ENEMY_HEALTH,
            true,
            false,
        )
    }

    pub fn shooting_enemy(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        pos.push(Position::new(x, y));
        Enemy::new(
            pos,
            SHOOTER_ENEMY_SPEED,
            tile::SHOOTER_ENEMY,
            world_pos,
            SHOOTER_ENEMY_HEALTH,
            true,
            false,
        )
    }

    pub fn knight(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        pos.push(Position::new(x, y));
        Enemy::new(
            pos,
            KNIGHT_ENEMY_SPEED,
            tile::KNIGHT_ENEMY,
            world_pos,
            KNIGHT_ENEMY_HEALTH,
            true,
            false,
        )
    }

    pub fn major_boss(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        pos.push(Position::new(x, y));
        Enemy::new(
            pos,
            MAJOR_BOSS_SPEED,
            tile::MAJOR_BOSS,
            world_pos,
            MAJOR_BOSS_HEALTH,
            true,
            false,
        )
    }

    pub fn minor_boss(x: usize, y: usize, world_pos: Position) -> Self {
        let mut pos = Vec::new();
        pos.push(Position::new(x, y));
        Enemy::new(
            pos,
            MINOR_BOSS_SPEED,
            tile::MINOR_BOSS,
            world_pos,
            MINOR_BOSS_HEALTH,
            true,
            false,
        )
    }

    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        // potentially modify the damage done with the multiplier
        self.health = max(0, self.health as i32 - damage as i32) as usize;
    }

    pub fn update(world: &mut World) {
        for index in
            (0..world.enemies_map[world.world_position.y][world.world_position.x].len()).rev()
        {
            // if world.enemies_map[world.world_position.y][world.world_position.x][index].pos.x >= WORLD_SIZE.0 as usize || world.enemies_map[world.world_position.y][world.world_position.x][index].pos.y >= WORLD_SIZE.1 as usize {
            //     panic!("Enemy out of bounds with position: {:?}", world.enemies_map[world.world_position.y][world.world_position.x][index].pos);
            // }
            if world.enemies_map[world.world_position.y][world.world_position.x][index].health <= 0
            {
                Enemy::kill(world, index);
            } else {
                if world.player.is_visible() {
                    Self::move_enemy(index, world);
                }
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        let delta =
            match world.enemies_map[world.world_position.y][world.world_position.x][index].color {
                tile::CHASING_ENEMY => CHASING_ENEMY_ENERGY_RETURN,
                tile::BOMBER_ENEMY => BOMBER_ENEMY_ENERGY_RETURN,
                tile::BOMBER_ENEMY_ACTIVATED => BOMBER_ENEMY_ENERGY_RETURN,
                tile::BOMBER_ENEMY_DEACTIVATED => BOMBER_ENEMY_ENERGY_RETURN,
                tile::MAJOR_ENEMY => MAJOR_ENEMY_ENERGY_RETURN,
                tile::SHOOTER_ENEMY => SHOOTER_ENEMY_ENERGY_RETURN,
                tile::KNIGHT_ENEMY => KNIGHT_ENEMY_ENERGY_RETURN,
                tile::MINOR_BOSS => MINOR_BOSS_ENERGY_RETURN,
                tile::MAJOR_BOSS => MAJOR_BOSS_ENERGY_RETURN,
                tile::BOMBER_ENEMY_ACTIVATED => 0,
                _ => unreachable!("Cannot be anything other than the enemy tiles"),
            } as i32;
        world.player.change_energy(delta);
        let enemy = &mut world.enemies_map[world.world_position.y][world.world_position.x][index]; 
        let pos = &mut enemy.pos;
        for tile in pos {
            world.entity_map[enemy.world_pos.y][enemy.world_pos.x].remove(&tile);
        }
        world.enemies_map[world.world_position.y][world.world_position.x].remove(index);
    }

    pub fn move_enemy_with_deltas(index: usize, world: &mut World) {
        let (delta_x, delta_y) = (
            world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0].x as i32 - world.player.pos.x as i32,
            world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0].y as i32 - world.player.pos.y as i32,
        );
        let direction = if delta_x.abs() > delta_y.abs() {
            // delta x will never be 0
            if delta_x > 0 {
                // move to the left
                Direction::West
            } else {
                // move to the right
                Direction::East
            }
        } else {
            // delta y will never be 0
            if delta_y > 0 {
                // move up
                Direction::North
            } else {
                // move down
                Direction::South
            }
        };

        let mut new_pos = Vec::new();
        for i in 0..world.enemies_map[world.world_position.y][world.world_position.x][index].pos.len() {
            let (a_new_pos, _) = World::new_position(
                world.enemies_map[world.world_position.y][world.world_position.x][index].pos[i],
                direction,
                world,
                world.enemies_map[world.world_position.y][world.world_position.x][index].speed,
                Entity::Enemy,
                Some(index),
            );
            new_pos.push(a_new_pos);
        }

        let world_pos = world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos;
        // TODO
        
        for tile in &new_pos {
            if (world.terrain_map[world_pos.y][world_pos.x].contains_key(&tile)) {
                if (!PERMISSIBLE_TILES.contains(
                    world.terrain_map[world_pos.y][world_pos.x]
                        .get(&tile)
                        .unwrap(),
                )) {
                    return;
                }
            }
            if (world.atmosphere_map[world_pos.y][world_pos.x].contains_key(&tile)) {
                if (!PERMISSIBLE_TILES.contains(
                    world.atmosphere_map[world_pos.y][world_pos.x]
                        .get(&tile)
                        .unwrap(),
                )) {
                    return;
                }
            }
            if (world.entity_map[world_pos.y][world_pos.x].contains_key(&tile)) {
                if (!PERMISSIBLE_TILES.contains(
                    &world.entity_map[world_pos.y][world_pos.x]
                        .get(&tile)
                        .unwrap()
                    .0,
                )) {
                    return;
                }
            }
        }
        if new_pos.contains(&world.player.pos) {
            world.player.damage(world.enemies_map[world.world_position.y][world.world_position.x][index].attack_damage);
        } else {
            let mut index_proj: i32 = 0;
            for _ in 0..world.projectiles.len() {
                if world.projectiles[index_proj as usize].color == tile::PROJECTILE_PLAYER {
                    if new_pos.contains(&world.projectiles[index_proj as usize].pos)
                        && world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos
                            == world.projectiles[index_proj as usize].world_pos
                    {
                        world.enemies_map[world.world_position.y][world.world_position.x][index].damage(world.projectiles[index_proj as usize].damage);
                        Projectile::kill(index_proj as usize, world);
                        index_proj -= 1;
                    }
                    index_proj += 1
                }
            }
            // simply updates the render queue
            for i in 0..new_pos.len() {
                World::update_position(
                    world,
                    world.enemies_map[world.world_position.y][world.world_position.x][index].pos[i],
                    (new_pos[i].clone(), world.world_position),
                    );
                world.enemies_map[world.world_position.y][world.world_position.x][index].pos = new_pos.clone();
            }
        }
    }

    // This just makes move along the best path for the speed, eg speed 2 = 2 moves along the best
    // path
    // returns if the enemy dies
    pub fn move_enemy(index: usize, world: &mut World) {
        // This gets the shortest path
        let can_dodge_projectiles = match world.enemies_map[world.world_position.y][world.world_position.x][index].color {
            tile::BOMBER_ENEMY => true,
            _ => false,
        };
        let mut travel_path = Self::get_best_path(index, world, can_dodge_projectiles);
        let enemy = &world.enemies_map[world.world_position.y][world.world_position.x][index];
        let mut cur_pos = enemy.pos[0];
        for _ in 0..enemy.speed {
            if let Some(new_pos) = travel_path.pop_front() {
                if Self::match_color(&world.enemies_map[world.world_position.y][world.world_position.x][index].color, &tile::CHASING_ENEMY) {
                    if new_pos.x >= WORLD_SIZE.0 as usize || new_pos.y >= WORLD_SIZE.1 as usize {
                        Self::move_enemy_with_deltas(index, world);
                        return;
                    }
                    if new_pos == world.player.pos {
                        world.player.damage(world.enemies_map[world.world_position.y][world.world_position.x][index].attack_damage);
                    } else {
                        let mut index_proj: i32 = 0;
                        for _ in 0..world.projectiles.len() {
                            if world.projectiles[index_proj as usize].color
                                == tile::PROJECTILE_PLAYER
                                || world.projectiles[index_proj as usize].color
                                    == tile::TRACKING_PROJECTILE
                            {
                                if new_pos == world.projectiles[index_proj as usize].pos
                                    && world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos
                                        == world.projectiles[index_proj as usize].world_pos
                                {
                                    world.enemies_map[world.world_position.y][world.world_position.x][index]
                                        .damage(world.projectiles[index_proj as usize].damage);
                                    Projectile::kill(index_proj as usize, world);
                                    index_proj -= 1;
                                }
                                index_proj += 1
                            }
                        }
                        // simply updates the render queue
                        World::update_position(world, cur_pos, (new_pos, world.world_position));
                        world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0] = new_pos;
                        cur_pos = new_pos;
                    }
                } else if Self::match_color(&world.enemies_map[world.world_position.y][world.world_position.x][index].color, &tile::BOMBER_ENEMY) {
                    // activate bomber if within range (no movement)
                    if Self::player_within_spaces(&cur_pos, &world, 2) {
                        world.enemies_map[world.world_position.y][world.world_position.x][index].color = tile::BOMBER_ENEMY_ACTIVATED;
                        let curr_world =
                            &mut world.entity_map[world.world_position.y][world.world_position.x];
                        curr_world.insert(cur_pos, (world.enemies_map[world.world_position.y][world.world_position.x][index].color, Entity::Enemy));
                        return;
                    }

                    // otherwise move as normal chaser enemy
                    if new_pos.x >= WORLD_SIZE.0 as usize || new_pos.y >= WORLD_SIZE.1 as usize {
                        Self::move_enemy_with_deltas(index, world);
                        return;
                    }
                    if new_pos == world.player.pos {
                        world.player.damage(world.enemies_map[world.world_position.y][world.world_position.x][index].attack_damage);
                    } else {
                        let mut index_proj: i32 = 0;
                        for _ in 0..world.projectiles.len() {
                            if world.projectiles[index_proj as usize].color
                                == tile::PROJECTILE_PLAYER
                            {
                                if new_pos == world.projectiles[index_proj as usize].pos
                                    && world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos
                                        == world.projectiles[index_proj as usize].world_pos
                                {
                                    world.enemies_map[world.world_position.y][world.world_position.x][index]
                                        .damage(world.projectiles[index_proj as usize].damage);
                                    Projectile::kill(index_proj as usize, world);
                                    index_proj -= 1;
                                }
                                index_proj += 1
                            }
                        }
                        // simply updates the render queue
                        World::update_position(world, cur_pos, (new_pos, world.world_position));
                        world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0] = new_pos;
                        cur_pos = new_pos;
                    }
                } else if Self::match_color(
                    &world.enemies_map[world.world_position.y][world.world_position.x][index].color,
                    &tile::BOMBER_ENEMY_ACTIVATED,
                ) {
                    world.enemies_map[world.world_position.y][world.world_position.x][index].color = tile::BOMBER_ENEMY_DEACTIVATED;
                    let curr_world =
                        &mut world.entity_map[world.world_position.y][world.world_position.x];
                    curr_world.insert(cur_pos, (world.enemies_map[world.world_position.y][world.world_position.x][index].color, Entity::Enemy));
                    if Self::player_within_spaces(&cur_pos, &world, 2) {
                        world.player.damage(world.enemies_map[world.world_position.y][world.world_position.x][index].attack_damage);
                    }
                    Self::create_bomber_explosion(index, world);
                } else if Self::match_color(
                    &world.enemies_map[world.world_position.y][world.world_position.x][index].color,
                    &tile::BOMBER_ENEMY_DEACTIVATED,
                ) {
                    let pos = &world.enemies_map[world.world_position.y][world.world_position.x][index].pos;
                    world.entity_map[world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos.y]
                        [world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos.x]
                        .remove(&pos[0]);
                    world.enemies_map[world.world_position.y][world.world_position.x].remove(index);
                } else if Self::match_color(
                    &world.enemies_map[world.world_position.y][world.world_position.x][index].color,
                    &tile::MAJOR_ENEMY
                ) {
                    let del_x = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[4].x as i16 - world.player.pos.x as i16;
                    let del_y = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[4].y as i16 - world.player.pos.y as i16;

                    let (move_x, move_y): (i16, i16) = {
                        if del_x.abs() > del_y.abs() {
                            if del_x > 0 {
                                (-1, 0)
                            } else {
                                (1, 0)
                            }
                        } else {
                            if del_y > 0 {
                                (0, -1)
                            } else {
                                (0, 1)
                            }
                        }
                    };

                    let positions = world.enemies_map[world.world_position.y][world.world_position.x][index].pos.clone();
                    for tile in &positions {
                        let new_x = tile.x as i16 + move_x;
                        let new_y = tile.y as i16 + move_y;
                        let new_pos = Position::new(new_x as usize, new_y as usize);
                        if new_x < 0 || new_x >= WORLD_SIZE.0
                            || new_y < 0 || new_y >= WORLD_SIZE.1 {
                            return;
                        } else if new_pos == world.player.pos {
                            world.player.damage(world.enemies_map[world.world_position.y][world.world_position.x][index].attack_damage);
                            return;
                        }

                        let mut index_proj: i32 = 0;
                        for _ in 0..world.projectiles.len() {
                            if world.projectiles[index_proj as usize].color
                                == tile::PROJECTILE_PLAYER
                            {
                                if new_pos == world.projectiles[index_proj as usize].pos
                                    && world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos
                                        == world.projectiles[index_proj as usize].world_pos
                                {
                                    world.enemies_map[world.world_position.y][world.world_position.x][index]
                                        .damage(world.projectiles[index_proj as usize].damage);
                                    Projectile::kill(index_proj as usize, world);
                                    index_proj -= 1;
                                }
                                index_proj += 1
                            }
                        }

                        for i in 0..world.enemies_map[world.world_position.y][world.world_position.x].len() {
                            if i != index && world.enemies_map[world.world_position.y][world.world_position.x][i].pos.contains(&new_pos) {
                                return;
                            }
                        }
                    }

                    if move_x == -1 || move_y == -1 {
                        for i in 0..positions.len() {
                            let new_x = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[i].x as i16 + move_x;
                            let new_y = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[i].y as i16 + move_y;
                            let new_pos = Position::new(new_x as usize, new_y as usize);
                            World::update_position(world, world.enemies_map[world.world_position.y][world.world_position.x][index].pos[i], (new_pos, world.world_position));
                            world.enemies_map[world.world_position.y][world.world_position.x][index].pos[i] = new_pos;
                        }
                    } else {
                        for i in 0..positions.len() {
                            let new_x = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[positions.len()-i-1].x as i16 + move_x;
                            let new_y = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[positions.len()-i-1].y as i16 + move_y;
                            let new_pos = Position::new(new_x as usize, new_y as usize);
                            World::update_position(world, world.enemies_map[world.world_position.y][world.world_position.x][index].pos[positions.len()-i-1], (new_pos, world.world_position));
                            world.enemies_map[world.world_position.y][world.world_position.x][index].pos[positions.len()-i-1] = new_pos;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn get_best_path(
        index: usize,
        world: &mut World,
        can_dodge_projectiles: bool,
    ) -> LinkedList<Position> {
        // Used to check if the enemy should be able to dodge around player projectiles

        let enemy = &world.enemies_map[world.world_position.y][world.world_position.x][index];
        // this is a visited array to save if we have visited a location on the grid
        let mut visited = [[false; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];

        // this stores every location's previous location so that we can reconstruct the best path
        // given our start and end
        let mut previous = [[Position::new(WORLD_SIZE.0 as usize + 1, WORLD_SIZE.1 as usize + 1);
            WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];
        let mut queue = LinkedList::new();
        queue.push_back(enemy.pos[0]);

        visited[enemy.pos[0].y][enemy.pos[0].x] = true;
        // visited[enemy.pos.y - (world.world_position.y * WORLD_SIZE.1 as usize)][enemy.pos.x - (world.world_position.x * WORLD_SIZE.0 as usize)] = true;
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                if node == world.player.pos {
                    // reached the goal location, break and reconstruct path
                    break;
                }

                // standard bfs stuff, for each neighbor, if it hasn't been visited, put it into
                // the queue
                let neighbors =
                    Self::get_neighbors(world, node, can_dodge_projectiles, index, Entity::Enemy);
                for next in neighbors {
                    if !visited[next.y][next.x] {
                        queue.push_back(next);
                        visited[next.y][next.x] = true;

                        // mark the previous of the neighbor as the node to reconstruct the path
                        previous[next.y][next.x] = node;
                    }
                }
            }
        }

        // This uses the previous 2 dimensional array to reconstruct the best path
        let mut path = LinkedList::new();
        let mut position = world.player.pos;
        let enemy_pos = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0];
        while position != enemy_pos {
            path.push_front(position);

            // if the position's or y is greater than the world size, that means that a path wasn't
            // found, as it means the previous position did not have a previous, so we break out
            if position.x as i16 >= WORLD_SIZE.0 {
                break;
            }
            position = previous[position.y][position.x];
        }
        path
    }

    pub fn get_neighbors(
        world: &mut World,
        position: Position,
        can_dodge_projectiles: bool,
        index: usize,
        entity_type: Entity,
    ) -> Vec<Position> {
        let directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        let mut moves = Vec::new();

        // loop through all the directions
        for direction in directions {
            let (new_pos, _) = World::new_position(
                position,
                direction,
                world,
                1,
                entity_type.clone(),
                Some(index),
            );
            // if the new position is valid(correct tiles & within bounds) add it to the potential
            // neighbors
            if new_pos != world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0]
                && Self::can_travel_to(
                    world,
                    (new_pos, world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos),
                    &world.entity_map,
                    &world.terrain_map,
                    &world.atmosphere_map,
                    can_dodge_projectiles,
                )
                && world.enemies_map[world.world_position.y][world.world_position.x][index].world_pos == world.world_position
            {
                moves.push(new_pos);
            }
        }
        return moves;
    }

    pub fn can_travel_to(
        // this is the (position_in_world, position_of_world)
        world: &World,
        position_info: (Position, Position),
        entity_map: &[[HashMap<Position, ([f32; 4], Entity)>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
             (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
        terrain_map: &[[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
             (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
        atmosphere_map: &[[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
             (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
        can_dodge_projectiles: bool,
    ) -> bool {
        // check if there are any static or dynamic entities in the position
        if terrain_map[position_info.1.y][position_info.1.x].contains_key(&position_info.0) {
            let info = terrain_map[position_info.1.y][position_info.1.x].get(&position_info.0);
            if let Some(info_under) = info {
                if can_dodge_projectiles {
                    if PERMISSIBLE_TILES_DODGING.contains(&info_under) {
                        return true;
                    }
                } else {
                    if PERMISSIBLE_TILES.contains(&info_under) {
                        return true;
                    }
                }
            }
            return false;
        } else if entity_map[position_info.1.y][position_info.1.x].contains_key(&position_info.0) {
            let info = entity_map[position_info.1.y][position_info.1.x].get(&position_info.0);
            if let Some(info_under) = info {
                if can_dodge_projectiles {
                    if PERMISSIBLE_TILES_DODGING.contains(&info_under.0)
                        || info_under.1 == Entity::Player
                    {
                        return true;
                    }
                } else {
                    if PERMISSIBLE_TILES.contains(&info_under.0) || info_under.1 == Entity::Player {
                        return true;
                    }
                }
            }
            return false;
        }
        if atmosphere_map[position_info.1.y][position_info.1.x].contains_key(&position_info.0) {
            let info = atmosphere_map[position_info.1.y][position_info.1.x].get(&position_info.0);
            if let Some(info_under) = info {
                if can_dodge_projectiles {
                    if PERMISSIBLE_TILES_DODGING.contains(&info_under) {
                        return true;
                    }
                } else {
                    if PERMISSIBLE_TILES.contains(&info_under) {
                        return true;
                    }
                }
            }
            return false;
        }

        if Boss::pos_inside_boss(world, position_info.0, position_info.1) {
            return false;
        }

        true
    }

    pub fn can_dodge_projectiles(&self) -> bool {
        self.can_dodge_projectiles
    }

    pub fn is_boss(&self) -> bool {
        self.is_boss
    }

    pub fn match_color(a: &[f32; 4], b: &[f32; 4]) -> bool {
        const EPSILON: f32 = 0.000001;
        return (a[0] - b[0]).abs() < EPSILON
            && (a[1] - b[1]).abs() < EPSILON
            && (a[2] - b[2]).abs() < EPSILON;
    }

    pub fn player_within_spaces(pos: &Position, world: &World, spaces: i16) -> bool {
        (world.player.pos.x as i16 - pos.x as i16).abs() as usize
            + (world.player.pos.y as i16 - pos.y as i16).abs() as usize
            <= spaces as usize
    }

    pub fn draw_bomber_explosion(world: &mut World, canvas: &mut graphics::Canvas) {
        let curr_world =
            &mut world.bomber_explosions[world.world_position.y][world.world_position.x];
        for tile in curr_world {
            let x = tile.0.x;
            let y = tile.0.y;
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(graphics::Rect::new_i32(
                        x as i32 * TILE_SIZE.0 as i32,
                        (y as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                        TILE_SIZE.0 as i32,
                        TILE_SIZE.1 as i32,
                    ))
                    .color(tile.1),
            )
        }
        world.bomber_explosions[world.world_position.y][world.world_position.x].clear();
    }

    pub fn create_bomber_explosion(index: usize, world: &mut World) {
        let pos = world.enemies_map[world.world_position.y][world.world_position.x][index].pos[0];
        for i in -2..=2_i16 {
            for j in -(2 - i.abs())..=(2 - i.abs()) {
                let x = pos.x as i16 + i;
                let y = pos.y as i16 + j;
                if x >= 0 && x < WORLD_SIZE.0 && y >= 0 && y < WORLD_SIZE.1 {
                    world.bomber_explosions[world.world_position.y][world.world_position.x].push((
                        Position::new(x as usize, y as usize),
                        tile::BOMBER_EXPLOSION[((i.abs() + j.abs()) / 2) as usize],
                    ));
                }
            }
        }
    }
}
