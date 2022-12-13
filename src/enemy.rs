use crate::{
    direction::Direction,
    entity::Entity,
    tile::{self, PROJECTILE_PLAYER},
    utils::Position,
    world::World,
    BOARD_SIZE, TILE_SIZE, WORLD_SIZE,
};
use std::{collections::HashMap, collections::LinkedList};

const ENEMY_HEALTH: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 2] = [tile::GRASS, tile::PROJECTILE_PLAYER];
const PERMISSIBLE_TILES_DODGING: [[f32; 4]; 1] = [tile::GRASS];

// This is basically the same as the enemy for now, but I am just testing an enemy system
pub struct Enemy {
    // This is the position in the form (x, y)
    pub pos: Position,

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
}

impl Enemy {
    pub fn new(x: usize, y: usize, speed: usize, color: [f32; 4], world_pos: Position) -> Self {
        let temp = Self {
            pos: Position::new(x, y),
            direction: Direction::North,
            speed: 1,
            color,
            attack_damage: 1,
            health: ENEMY_HEALTH,
            resistance: 1.0,
            world_pos,
        };
        temp
    }

    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        // potentially modify the damage done with the multiplier
        self.health -= damage;
    }

    pub fn update(world: &mut World) {
        for index in (0..world.enemies.len()).rev() {
            if world.enemies[index].health <= 0 {
                Enemy::kill(world, index);
            } else {
                if world.world_position == world.enemies[index].world_pos {
                    Self::move_enemy(index, world);
                }
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        let pos = world.enemies[index].pos;
        world.enemies.remove(index);
        world.entity_positions.remove(&pos);
    }

    // This just makes move along the best path for the speed, eg speed 2 = 2 moves along the best
    // path
    pub fn move_enemy(index: usize, world: &mut World) {
        // This gets the shortest path
        let mut travel_path = Self::get_best_path(index, world);
        let enemy = &world.enemies[index];
        let mut cur_pos = enemy.pos;
        for _ in 0..enemy.speed {
            if let Some(new_pos) = travel_path.pop_front() {
                if new_pos == world.player.pos {
                    world.player.damage(world.enemies[index].attack_damage);
                } else {
                    // simply updates the render queue
                    World::update_position(world, cur_pos, (new_pos, world.world_position));
                    world.enemies[index].pos = new_pos;
                    cur_pos = new_pos;
                }
            } else {
                break;
            }
        }
    }

    pub fn get_best_path(index: usize, world: &mut World) -> LinkedList<Position> {
        // Used to check if the enemy should be able to dodge around player projectiles
        let can_dodge_projectiles = match world.enemies[index].color {
            tile::BOMBER => true,
            _ => false,
        };

        let enemy = &world.enemies[index];
        // this is a visited array to save if we have visited a location on the grid
        let mut visited = [[false; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];

        // this stores every location's previous location so that we can reconstruct the best path
        // given our start and end
        let mut previous = [[Position::new(WORLD_SIZE.0 as usize, WORLD_SIZE.1 as usize);
            WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];

        let mut queue = LinkedList::new();
        queue.push_back(enemy.pos);

        visited[enemy.pos.y][enemy.pos.x] = true;
        // visited[enemy.pos.y - (world.world_position.y * WORLD_SIZE.1 as usize)][enemy.pos.x - (world.world_position.x * WORLD_SIZE.0 as usize)] = true;
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                if node == world.player.pos {
                    // reached the goal location, break and reconstruct path
                    break;
                }

                // standard bfs stuff, for each neighbor, if it hasn't been visited, put it into
                // the queue
                let neighbors = Self::get_neighbors(
                    world,
                    node,
                    can_dodge_projectiles,
                    index,
                    Entity::Enemy(index),
                );
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
        let enemy_pos = world.enemies[index].pos;
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
            let (new_pos, _) =
                World::new_position(position, direction, world, 1, entity_type.clone());
            // if the new position is valid(correct tiles & within bounds) add it to the potential
            // neighbors
            if new_pos != world.enemies[index].pos
                && Self::can_travel_to(
                    (new_pos, world.enemies[index].world_pos),
                    &world.entity_map,
                    &world.terrain_map,
                    can_dodge_projectiles,
                )
                && world.enemies[index].world_pos == world.world_position
            {
                moves.push(new_pos);
            }
        }
        return moves;
    }

    pub fn can_travel_to(
        // this is the (position_in_world, position_of_world)
        position_info: (Position, Position),
        entity_map: &[[HashMap<Position, ([f32; 4], Entity)>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
             (BOARD_SIZE.0 / WORLD_SIZE.0) as usize],
        terrain_map: &[[HashMap<Position, [f32; 4]>; (BOARD_SIZE.1 / WORLD_SIZE.1) as usize];
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
        } else if terrain_map[position_info.1.y][position_info.1.x].contains_key(&position_info.0) {
            let info = entity_map[position_info.1.y][position_info.1.x].get(&position_info.0);
            if let Some(info_under) = info {
                if can_dodge_projectiles {
                    if PERMISSIBLE_TILES_DODGING.contains(&info_under.0) {
                        return true;
                    }
                } else {
                    if PERMISSIBLE_TILES.contains(&info_under.0) {
                        return true;
                    }
                }
            }
            return false;
        }
        // if entity_map[position_info.1.y][position_info.1.x].contains_key(&position_info.0) {
        // || terrain_map[position_info.1.y][position_info.1.x].contains_key(&position_info.0) {
        // let info = entity_map[position_info.1.y][position_info.1.x].get(&position_info.0);
        // let info2 = terrain_map[position_info.1.y][position_info.1.x].get(&position_info.0);
        // if can_dodge_projectiles {
        //     if let Some(info) = info {
        //         if PERMISSIBLE_TILES_DODGING.contains(&info.0) {
        //             return true;
        //         }
        //     }
        //     if let Some(info) = info2 {
        //         if PERMISSIBLE_TILES_DODGING.contains(&info) {
        //             return true;
        //         }
        //     }
        // } else {
        //     if let Some(info) = info {
        //         if PERMISSIBLE_TILES.contains(&info.0) {
        //             return true;
        //         }
        //     }
        //     if let Some(info) = info2 {
        //         if PERMISSIBLE_TILES.contains(&info) {
        //             return true;
        //         }
        //     }
        // }
        // return false;
        // }
        true
    }
}
