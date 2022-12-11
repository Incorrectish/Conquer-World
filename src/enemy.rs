use crate::{direction::Direction, tile, world::World, WORLD_SIZE, utils::Position, entity::Entity, TILE_SIZE};
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
    collections::{VecDeque, LinkedList}, num,

};
const ENEMY_HEALTH: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 3] = [tile::GRASS, tile::PLAYER, tile::ENEMY];


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

    // Stores enemy health: for enemy death and such
    health: usize,

    resistance: f32,
}

impl Enemy {
    pub fn new(
        world: &mut [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
        x: usize,
        y: usize,
    ) -> Self {
        let temp = Self {
            pos: Position::new(x, y),
            direction: Direction::North,
            speed: 1,
            color: tile::ENEMY,
            health: ENEMY_HEALTH,
            resistance: 1.0,
        };
        world[y][x] = temp.color;
        temp
    }

    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        // potentially modify the damage done with the multiplier
        self.health -= damage;
    }

    // TODO: rewrite to make the travel function the same as player travel
    // pub fn travel(
    //     &mut self,
    //     world: &mut World,
    // ) {
    //     world.world[self.pos.1][self.pos.0] = self.covered_tile;
    //     match self.direction {
    //         Direction::North => {
    //             if self.pos.1 > 0 {
    //                 self.pos.1 -= 1
    //             }
    //         }
    //         Direction::South => {
    //             if self.pos.1 < (WORLD_SIZE.1 - 1) as usize {
    //                 self.pos.1 += 1
    //             }
    //         }
    //         Direction::East => {
    //             if self.pos.0 < (WORLD_SIZE.0 - 1) as usize {
    //                 self.pos.0 += 1
    //             }
    //         }
    //         Direction::West => {
    //             if self.pos.0 > 0 {
    //                 self.pos.0 -= 1
    //             }
    //         }
    //     }
    //     self.covered_tile = world.world[self.pos.1][self.pos.0];
    //     world.world[self.pos.1][self.pos.0] = self.color;
    // }

    // pub fn find_path(world: &mut World) -> VecDeQueue

    pub fn update(world: &mut World) {
        // thinking of using a hack to remove all the enemies at the position instead because two
        // enemies cannot be on the same tile, would avoid the f32 lack of equality
        for index in (0..world.enemies.len()).rev() {
            if world.enemies[index].health <= 0 {
                Enemy::kill(world, index);
            }
            if World::coordinates_are_within_world(world, world.enemies[index].pos) {
                Self::move_enemy(index, world);
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        // TODO: refactor next two lines using terrain implementation
        // world.world[world.enemies[index].pos.y][world.enemies[index].pos.x] =
        //    world.board[world.enemies[index].pos.y][world.enemies[index].pos.x];
        world.enemies.remove(index);
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
                // simply updates the render queue
                World::update_position(world, cur_pos, new_pos);
                world.enemies[index].pos = new_pos;
                cur_pos = new_pos;
            } else {
                break;
            }
        }
     }        

    pub fn get_best_path(index: usize, world: &mut World) -> LinkedList<Position> {
        let enemy = &world.enemies[index];
        // this is a visited array to save if we have visited a location on the grid
        let mut visited = [[false; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];

        // this stores every location's previous location so that we can reconstruct the best path
        // given our start and end
        let mut previous = [[Position::new(WORLD_SIZE.0 as usize, WORLD_SIZE.1 as usize); WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize];

        let mut queue = LinkedList::new();
        queue.push_back(enemy.pos);

        
        visited[enemy.pos.y - world.y_offset][enemy.pos.x - world.x_offset] = true;
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {

                // reached the goal location, break and reconstruct path
                if node == world.player.pos {
                    break;
                }

                // standard bfs stuff, for each neighbor, if it hasn't been visited, put it into
                // the queue
                let neighbors = Self::get_neighbors(world, node);
                for next in neighbors {
                    if !visited[next.y - world.y_offset][next.x - world.x_offset] {
                        queue.push_back(next);
                        visited[next.y - world.y_offset][next.x - world.x_offset] = true;

                        // mark the previous of the neighbor as the node to reconstruct the path
                        previous[next.y - world.y_offset][next.x - world.x_offset] = node;
                    }
                }
            }
        }

        // This uses the previous 2 dimensional array to reconstruct the best path
        let mut path = LinkedList::new();
        let mut position = world.player.pos;
        let enemy_pos = world.enemies[index].pos;
        while (position != enemy_pos) {
            path.push_front(position);

            // if the position's or y is greater than the world size, that means that a path wasn't
            // found, as it means the previous position did not have a previous, so we break out
            if (position.x - world.x_offset) as i16 >= WORLD_SIZE.0 {
                break;
            }
            position = previous[position.y - world.y_offset][position.x - world.x_offset];
        }
        path
    }

    pub fn get_neighbors(world:&mut World, position: Position) -> Vec<Position> {
        let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
        let mut moves = Vec::new();

        // loop through all the directions
        for direction in directions {
            let pos = World::new_position(position, direction, world, 1);

            // if the new position is valid(correct tiles & within bounds) add it to the potential
            // neighbors
            if Self::can_travel_to(
                pos, &world.entity_positions, 
                &world.terrain_positions) 
                && World::coordinates_are_within_world(world, pos) 
                {
                    moves.push(pos);
                }
        }
        return moves;
    }

    pub fn can_travel_to(
        position: Position,
        entity_positions: &HashMap<Position, ([f32; 4], Entity)>,
        terrain_positions: &HashMap<Position, [f32;4]>
    ) -> bool {

        // check if there are any static or dynamic entities in the position
        if entity_positions.contains_key(&position) || terrain_positions.contains_key(&position) {
            let info = entity_positions.get(&position);
            let info2 = terrain_positions.get(&position);
            if let Some(info) = info {
                if PERMISSIBLE_TILES.contains(&info.0) {
                    return true;
                }
            }
            if let Some(info) = info2 {
                if PERMISSIBLE_TILES.contains(&info) {
                    return true;
                }
            }
            return false;
        }
        true
    }
}
