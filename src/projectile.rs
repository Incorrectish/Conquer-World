use crate::{
    direction::Direction, entity::Entity, player::Player, tile, utils::Position, world::World,
    BOARD_SIZE, TILE_SIZE, WORLD_SIZE,
};
use ggez::graphics::{self, Canvas};
use std::{
    cmp::{max, min},
    collections::{HashMap, LinkedList},
};

const TRACKING_PROJECTILE_DAMAGE: usize = 100;
const TRACKING_PROJECTILE_SPEED: usize = 3;
const LIGHTNING_DAMAGE: usize = 80;
const LIGHTNING_SPEED: usize = 0;
const LIGHTNING_SIZE: i16 = 2;
const PLAYER_PROJECTILE_DAMAGE: usize = 20;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const FIRE_DAMAGE_INITIAL: usize = 60;
const FIRE_DAMAGE_SECONDARY: usize = 45;
const FIRE_DAMAGE_TERTIARY: usize = 30;
const FIRE_DAMAGE_FINAL: usize = 15;
const FIRE_SPEED: usize = 1;

const PERMISSIBLE_TILES: [[f32; 4]; 10] = [
    tile::WATER,
    tile::GRASS,
    tile::PLAYER,
    // tile::PROJECTILE_PLAYER,
    tile::CHASING_ENEMY,
    tile::BOMBER_ENEMY,
    tile::MAJOR_ENEMY,
    tile::KNIGHT_ENEMY,
    tile::SHOOTER_ENEMY,
    tile::MINOR_BOSS,
    tile::MAJOR_BOSS,
];

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct Projectile {
    pub pos: Position,
    pub speed: usize,
    pub direction: Direction,
    pub color: [f32; 4],
    pub damage: usize,
    pub world_pos: Position,
    // maybe add an alignment so projectiles from enemies cannot damage themselves and projectiles
    // from players cannot damage themselves
}

impl Projectile {
    pub fn tracking_projectile(x: usize, y: usize, world_pos: Position) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: 2,
            direction: Direction::North,
            color: tile::TRACKING_PROJECTILE,
            damage: TRACKING_PROJECTILE_DAMAGE,
            world_pos,
        }
    }

    pub fn player_projectile(
        x: usize,
        y: usize,
        direction: Direction,
        world_pos: Position,
    ) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: PLAYER_PROJECTILE_SPEED,
            direction,
            color: tile::PROJECTILE_PLAYER,
            damage: PLAYER_PROJECTILE_DAMAGE,
            world_pos,
        }
    }

    pub fn lightning(x: usize, y: usize, world_pos: Position) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: LIGHTNING_SPEED,
            direction: Direction::North,
            color: tile::LIGHTNING_PLACEHOLDER,
            damage: LIGHTNING_DAMAGE,
            world_pos,
        }
    }

    pub fn player_fire(x: usize, y: usize, direction: Direction, world_pos: Position) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed: FIRE_SPEED,
            direction,
            color: tile::FIRE_PLACEHOLDER,
            damage: FIRE_DAMAGE_INITIAL,
            world_pos,
        }
    }

    fn new(
        x: usize,
        y: usize,
        speed: usize,
        damage: usize,
        direction: Direction,
        color: [f32; 4],
        player_pos: Position,
    ) -> Self {
        Projectile {
            pos: Position::new(x, y),
            speed,
            damage,
            direction,
            color,
            world_pos: player_pos,
        }
    }

    pub fn update(world: &mut World) {
        let mut index: i32 = 0;
        for _ in 0..world.projectiles.len() {
            match world.projectiles[index as usize].color {
                tile::LIGHTNING_PLACEHOLDER => {
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::LIGHTNING_INITIAL;
                    world.atmosphere_map[world_pos.y][world_pos.x]
                        .insert(pos, tile::LIGHTNING_INITIAL);
                }
                tile::LIGHTNING_INITIAL => {
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::LIGHTNING_SECONDARY;
                    world.atmosphere_map[world_pos.y][world_pos.x]
                        .insert(pos, tile::LIGHTNING_SECONDARY);
                }
                tile::LIGHTNING_SECONDARY => {
                    const deltas: [i16; 3] = [0, 1, -1];
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::LIGHTNING_FINAL;
                    // basically checks the 8 around and including the projectile and turns
                    // them to their original state
                    for i in 1..=LIGHTNING_SIZE {
                        for x_delta in deltas {
                            for y_delta in deltas {
                                if pos.x < (WORLD_SIZE.0 - x_delta * i) as usize
                                    && pos.y < (WORLD_SIZE.1 - y_delta * i) as usize
                                    && pos.x as i16 >= -(x_delta * i)
                                    && pos.y as i16 >= -(y_delta * i)
                                {
                                    let new_position = Position::new(
                                        (pos.x as i16 + (x_delta * i)) as usize,
                                        (pos.y as i16 + (y_delta * i)) as usize,
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .insert(new_position, tile::LIGHTNING_FINAL);
                                    for enemy in &mut world.enemies {
                                        if enemy.pos.contains(&new_position) {
                                            enemy.damage(LIGHTNING_DAMAGE);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                tile::LIGHTNING_FINAL => {
                    const deltas: [i16; 3] = [0, 1, -1];
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles.remove(index as usize);
                    index -= 1;
                    // basically checks the 8 around and including the projectile and turns
                    // them to their original state
                    for i in 1..=LIGHTNING_SIZE {
                        for x_delta in deltas {
                            for y_delta in deltas {
                                if pos.x < (WORLD_SIZE.0 - x_delta * i) as usize
                                    && pos.y < (WORLD_SIZE.1 - y_delta * i) as usize
                                    && pos.x as i16 >= -(x_delta * i)
                                    && pos.y as i16 >= -(y_delta * i)
                                {
                                    let new_position = Position::new(
                                        (pos.x as i16 + (x_delta * i)) as usize,
                                        (pos.y as i16 + (y_delta * i)) as usize,
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .remove(&new_position);
                                }
                            }
                        }
                    }
                }
                tile::FIRE_PLACEHOLDER => {
                    let pos = world.projectiles[index as usize].pos;
                    let world_pos = world.projectiles[index as usize].world_pos;
                    world.projectiles[index as usize].color = tile::FIRE_INITIAL;
                    world.atmosphere_map[world_pos.y][world_pos.x].insert(pos, tile::FIRE_INITIAL);
                    for enemy in &mut world.enemies {
                        if enemy.pos.contains(&pos) {
                            enemy.damage(FIRE_DAMAGE_INITIAL);
                        }
                    }
                }
                tile::FIRE_INITIAL => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let old_pos = world.projectiles[index as usize].pos;
                    world.projectiles[index as usize].color = tile::FIRE_SECONDARY;
                    world.atmosphere_map[world_pos.y][world_pos.x].remove(&old_pos);
                    let (new_pos, new_world_pos) = World::new_position(
                        old_pos,
                        world.projectiles[index as usize].direction,
                        world,
                        world.projectiles[index as usize].speed,
                        Entity::Projectile,
                        Some(index as usize),
                    );
                    world.projectiles[index as usize].pos = new_pos;
                    let mut new_positions = Vec::new();
                    const INITIAL_DELTA: usize = 1;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) =
                            match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => (
                                    Position::new(new_pos.x + i, new_pos.y),
                                    Position::new(
                                        max(0, new_pos.x as i32 - i as i32) as usize,
                                        new_pos.y,
                                    ),
                                ),
                                Direction::East | Direction::West => (
                                    Position::new(new_pos.x, new_pos.y + i),
                                    Position::new(
                                        new_pos.x,
                                        max(0, new_pos.y as i32 - i as i32) as usize,
                                    ),
                                ),
                            };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for enemy in &mut world.enemies {
                        for new_pos in &new_positions {
                            if enemy.pos.contains(&new_pos) {
                                enemy.damage(FIRE_DAMAGE_SECONDARY);
                            }
                        }
                    }
                    for new_position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x]
                            .insert(*new_position, tile::FIRE_SECONDARY);
                    }
                }
                // TODO: Copy paste code
                tile::FIRE_SECONDARY => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let old_pos = world.projectiles[index as usize].pos;
                    world.projectiles[index as usize].color = tile::FIRE_TERTIARY;
                    world.atmosphere_map[world_pos.y][world_pos.x].remove(&old_pos);
                    let (new_pos, new_world_pos) = World::new_position(
                        old_pos,
                        world.projectiles[index as usize].direction,
                        world,
                        world.projectiles[index as usize].speed,
                        Entity::Projectile,
                        Some(index as usize),
                    );
                    world.projectiles[index as usize].pos = new_pos;
                    let mut new_positions = Vec::new();
                    const INITIAL_DELTA: usize = 2;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) =
                            match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => {
                                    world.atmosphere_map[world_pos.y][world_pos.x].remove(
                                        &Position::new(
                                            max(0, old_pos.x as i32 - i as i32) as usize,
                                            old_pos.y,
                                        ),
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .remove(&Position::new(old_pos.x + i, old_pos.y));
                                    (
                                        Position::new(new_pos.x + i, new_pos.y),
                                        Position::new(
                                            max(0, new_pos.x as i32 - i as i32) as usize,
                                            new_pos.y,
                                        ),
                                    )
                                }
                                Direction::East | Direction::West => {
                                    world.atmosphere_map[world_pos.y][world_pos.x].remove(
                                        &Position::new(
                                            old_pos.x,
                                            max(0, old_pos.y as i32 - i as i32) as usize,
                                        ),
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .remove(&Position::new(old_pos.x, old_pos.y + i));
                                    (
                                        Position::new(new_pos.x, new_pos.y + i),
                                        Position::new(
                                            new_pos.x,
                                            max(0, new_pos.y as i32 - i as i32) as usize,
                                        ),
                                    )
                                }
                            };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for enemy in &mut world.enemies {
                        for new_pos in &new_positions {
                            if enemy.pos.contains(&*new_pos) {
                                enemy.damage(FIRE_DAMAGE_TERTIARY);
                            }
                        }
                    }
                    for new_position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x]
                            .insert(*new_position, tile::FIRE_TERTIARY);
                    }
                }
                tile::FIRE_TERTIARY => {
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let old_pos = world.projectiles[index as usize].pos;
                    world.projectiles[index as usize].color = tile::FIRE_FINAL;
                    let (new_pos, new_world_pos) = World::new_position(
                        old_pos,
                        world.projectiles[index as usize].direction,
                        world,
                        world.projectiles[index as usize].speed,
                        Entity::Projectile,
                        Some(index as usize),
                    );
                    world.projectiles[index as usize].pos = new_pos;
                    let mut new_positions = Vec::new();
                    const INITIAL_DELTA: usize = 3;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) =
                            match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => {
                                    world.atmosphere_map[world_pos.y][world_pos.x].remove(
                                        &Position::new(
                                            max(0, old_pos.x as i32 - i as i32) as usize,
                                            old_pos.y,
                                        ),
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .remove(&Position::new(old_pos.x + i, old_pos.y));
                                    (
                                        Position::new(new_pos.x + i, new_pos.y),
                                        Position::new(
                                            max(0, new_pos.x as i32 - i as i32) as usize,
                                            new_pos.y,
                                        ),
                                    )
                                }
                                Direction::East | Direction::West => {
                                    world.atmosphere_map[world_pos.y][world_pos.x].remove(
                                        &Position::new(
                                            old_pos.x,
                                            max(0, old_pos.y as i32 - i as i32) as usize,
                                        ),
                                    );
                                    world.atmosphere_map[world_pos.y][world_pos.x]
                                        .remove(&Position::new(old_pos.x, old_pos.y + i));
                                    (
                                        Position::new(new_pos.x, new_pos.y + i),
                                        Position::new(
                                            new_pos.x,
                                            max(0, new_pos.y as i32 - i as i32) as usize,
                                        ),
                                    )
                                }
                            };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for enemy in &mut world.enemies {
                        for new_pos in &new_positions {
                            if enemy.pos.contains(&new_pos) {
                                enemy.damage(FIRE_DAMAGE_FINAL);
                            }
                        }
                    }
                    for new_position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x]
                            .insert(*new_position, tile::FIRE_FINAL);
                    }
                }
                tile::FIRE_FINAL => {
                    // TODO: get this dissapearing the thing properly
                    let world_pos = world.projectiles[index as usize].world_pos;
                    let mut new_positions = Vec::new();
                    let new_pos = world.projectiles[index as usize].pos;
                    const INITIAL_DELTA: usize = 3;
                    for i in (0..=INITIAL_DELTA) {
                        let (positive_new_pos, negative_new_pos) =
                            match world.projectiles[index as usize].direction {
                                Direction::South | Direction::North => (
                                    Position::new(new_pos.x + i, new_pos.y),
                                    Position::new(
                                        max(0, new_pos.x as i32 - i as i32) as usize,
                                        new_pos.y,
                                    ),
                                ),
                                Direction::East | Direction::West => (
                                    Position::new(new_pos.x, new_pos.y + i),
                                    Position::new(
                                        new_pos.x,
                                        max(0, new_pos.y as i32 - i as i32) as usize,
                                    ),
                                ),
                            };
                        if !new_positions.contains(&negative_new_pos) {
                            new_positions.push(negative_new_pos);
                        }
                        if !new_positions.contains(&positive_new_pos) {
                            new_positions.push(positive_new_pos);
                        }
                    }
                    for position in &new_positions {
                        world.atmosphere_map[world_pos.y][world_pos.x].remove(position);
                    }
                    Projectile::kill(index as usize, world);
                    index -= 1;
                }
                tile::TRACKING_PROJECTILE => {
                    // move_tracking projectile(index, world);
                    let (found_path, collided) =
                        Self::move_tracking_projectile(index as usize, world);
                    if (!found_path || collided) {
                        Projectile::kill(index as usize, world);
                        index -= 1;
                    }
                }
                _ => {
                    if !World::travel(world, Entity::Projectile, Some(index as usize)) {
                        Projectile::kill(index as usize, world);
                        //When projectile dies, whole array shifts back one,
                        //so need to account for this in order to check the next projectile  in array
                        index -= 1;
                    }
                }
            }
            index += 1;
            // case for impact with player

            // case for impact with enemy

            // general projectile movement
        }
    }

    // returns (found_a_path, collided_with_enemy)
    // ASSUMES THAT THIS IS JUST FOR PLAYERS
    pub fn move_tracking_projectile(index: usize, world: &mut World) -> (bool, bool) {
        // This gets the shortest path
        let mut travel_path = Self::get_best_path(index, world);
        let projectile = &world.projectiles[index];
        let mut cur_pos = projectile.pos;
        for _ in 0..projectile.speed {
            if let Some(new_pos) = travel_path.pop_front() {
                // no paths found
                if new_pos.x >= WORLD_SIZE.0 as usize || new_pos.y >= WORLD_SIZE.1 as usize {
                    return (false, false);
                } else {
                    let mut index_enemy: i32 = 0;
                    for _ in 0..world.enemies.len() {
                        if (world.enemies[index_enemy as usize].pos.contains(&new_pos)) {
                            world.enemies[index_enemy as usize].damage(TRACKING_PROJECTILE_DAMAGE);
                            return (true, true);
                        }
                        index_enemy += 1;
                    }
                    // simply updates the render queue
                    World::update_atmosphere_position(
                        world,
                        cur_pos,
                        (new_pos, world.world_position),
                    );
                    world.projectiles[index].pos = new_pos;
                    cur_pos = new_pos;
                }
            }
        }
        (true, false)
    }

    pub fn get_best_path(index: usize, world: &mut World) -> LinkedList<Position> {
        // Used to check if the enemy should be able to dodge around player projectiles

        let projectile = &world.projectiles[index];
        let world_pos = world.world_position;
        let mut target_pos =
            Position::new((WORLD_SIZE.0 + 1) as usize, (WORLD_SIZE.1 + 1) as usize);
        // this is a visited array to save if we have visited a location on the grid
        let mut visited = [[false; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];

        // this stores every location's previous location so that we can reconstruct the best path
        // given our start and end
        let mut previous = [[Position::new(WORLD_SIZE.0 as usize + 1, WORLD_SIZE.1 as usize + 1);
            WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];
        let mut queue = LinkedList::new();
        queue.push_back(projectile.pos);

        visited[projectile.pos.y][projectile.pos.x] = true;
        // visited[enemy.pos.y - (world.world_position.y * WORLD_SIZE.1 as usize)][enemy.pos.x - (world.world_position.x * WORLD_SIZE.0 as usize)] = true;
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                // if there is an entity at the new position and it's an enemy, we know we have
                // found the shortest path to an enemy, so end immediately to reconstruct best path
                if world.entity_map[world_pos.y][world_pos.x].contains_key(&node)
                    && world.entity_map[world_pos.y][world_pos.x][&node].1 == Entity::Enemy
                {
                    // reached the goal location, break and reconstruct path
                    target_pos = node;
                    break;
                }

                // standard bfs stuff, for each neighbor, if it hasn't been visited, put it into
                // the queue
                let neighbors = Self::get_neighbors(world, node, index, Entity::Projectile);
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
        let projectile_pos = world.projectiles[index].pos;
        while target_pos != projectile_pos {
            path.push_front(target_pos);

            // if the position's or y is greater than the world size, that means that a path wasn't
            // found, as it means the previous position did not have a previous, so we break out
            if target_pos.x as i16 >= WORLD_SIZE.0 {
                break;
            }
            target_pos = previous[target_pos.y][target_pos.x];
        }
        path
    }

    pub fn get_neighbors(
        world: &mut World,
        position: Position,
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
            if new_pos != world.projectiles[index].pos
                && Self::can_travel_to(world, (new_pos, world.projectiles[index].world_pos))
                && world.projectiles[index].world_pos == world.world_position
            {
                moves.push(new_pos);
            }
        }
        return moves;
    }

    pub fn kill(index: usize, world: &mut World) {
        match world.projectiles[index].color {
            tile::TRACKING_PROJECTILE
            | tile::FIRE_INITIAL
            | tile::FIRE_SECONDARY
            | tile::FIRE_TERTIARY
            | tile::FIRE_PLACEHOLDER
            | tile::LIGHTNING_FINAL
            | tile::LIGHTNING_INITIAL
            | tile::LIGHTNING_SECONDARY
            | tile::LIGHTNING_PLACEHOLDER => {
                world.atmosphere_map[world.projectiles[index].world_pos.y]
                    [world.projectiles[index].world_pos.x]
                    .remove(&world.projectiles[index].pos);
            }
            _ => {
                world.entity_map[world.projectiles[index].world_pos.y]
                    [world.projectiles[index].world_pos.x]
                    .remove(&world.projectiles[index].pos);
            }
        }
        world.projectiles.remove(index);
    }

    pub fn can_travel_to(
        world: &mut World,
        position_info: (Position, Position), //Where .0 is the position, and .1 is the world_position
    ) -> bool {
        //Get the map on which the position is on
        let terrain_map = &world.terrain_map;
        let entity_map = &world.entity_map;
        let curr_terrain_map = &terrain_map[position_info.1.y][position_info.1.x];
        let curr_entity_map = &entity_map[position_info.1.y][position_info.1.x];
        if curr_entity_map.contains_key(&position_info.0)
            || curr_terrain_map.contains_key(&position_info.0)
        {
            if let Some(info) = curr_entity_map.get(&position_info.0) {
                if PERMISSIBLE_TILES.contains(&info.0) {
                    return true;
                }
            }
            if let Some(info) = curr_terrain_map.get(&position_info.0) {
                if PERMISSIBLE_TILES.contains(&info) {
                    return true;
                }
            }
            return false;
        }
        true
    }
}
