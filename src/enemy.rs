use crate::{direction::Direction, tile, world::World, WORLD_SIZE, utils::Position, entity::Entity, TILE_SIZE};
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
    collections::VecDeque, num,

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
            Self::move_enemy(index, world);
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        world.world[world.enemies[index].pos.y][world.enemies[index].pos.x] =
            world.board[world.enemies[index].pos.y][world.enemies[index].pos.x];
        world.enemies.remove(index);
    }

    pub fn move_enemy(index: usize, world: &mut World) {
        let enemy = &world.enemies[index];
        let mut x = enemy.pos.x as i32;
        let mut y = enemy.pos.y  as i32;
        let delta = (x - world.player.pos.x as i32, y - world.player.pos.y as i32);
        
        if i32::abs(delta.0) > i32::abs(delta.1) {
            if delta.0 < 0 {
                x += 1;
            } else {
                x -= 1;
            }
        } else {
            if delta.1 < 0 {
                y += 1;
            } else {
                y -= 1;
            }
        }
        let new_pos = Position::new(x as usize, y as usize);
        World::update_position(world, enemy.pos, new_pos);
        world.enemies[index].pos = new_pos;
    }

    //BELOW -----------> Really Slow BFS method, doesn't even work half the time because the path is really long.
    // pub fn move_enemy(index: usize, world: &mut World) {
    //     let mut travel_path = Self::get_best_path(index, world);
    //     let enemy = &world.enemies[index];
    //     let mut cur_pos = enemy.pos;
    //     for num_move in 0..enemy.speed {
    //         if let Some(new_pos) = travel_path.pop_front() {
    //             World::update_position(world, cur_pos, new_pos);
    //             world.enemies[index].pos = new_pos;
    //             cur_pos = new_pos;
    //         }
    //     }

    //  }        

    // pub fn get_best_path(index: usize, world: &mut World) -> VecDeque<Position> {
    //     let enemy = &world.enemies[index];
    //     let mut visited: HashMap<Position, bool> = HashMap::new();
    //     let mut possible_paths: VecDeque<VecDeque<Position>> = VecDeque::new();
    //     let mut actual_path: VecDeque<Position> = VecDeque::new();
    //     actual_path.push_back(enemy.pos);
    //     possible_paths.push_back(actual_path.clone());
    //     while !possible_paths.is_empty() {
    //         if let Some(curr_path) = possible_paths.pop_back() {
    //             if let Some(curr_move) = curr_path.get(curr_path.len()-1) {
    //                 if *curr_move == world.player.pos {
    //                     actual_path = curr_path;
    //                     break;
    //                 }
    //                 let pos_moves = Self::add_moves(world, *curr_move);
    //                 for moves in pos_moves {
    //                     if !visited.contains_key(&moves) {
    //                         visited.insert(moves, true);
    //                         let mut possible_path_copy = curr_path.clone();
    //                         possible_path_copy.push_back(moves);
    //                         possible_paths.push_back(possible_path_copy);
    //                     } 
    //                 }
    //             }
    //         }
    //     }
    //     actual_path.pop_front();
    //     return actual_path;
    // }

    // pub fn add_moves(world:&mut World, position: Position) -> Vec<Position> {
    //     let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
    //     let mut moves = Vec::new();
    //     for direction in directions {
    //         let pos = World::new_position(position, direction, world, 1);
    //         if Self::can_travel_to(
    //             pos, &world.entity_positions, 
    //             &world.terrain_positions) 
    //             && World::coordinates_are_within_board(world, pos) 
    //             {
    //                 moves.push(pos);
    //             }
    //     }
    //     return moves;
    // }

    // pub fn can_travel_to(
    //     position: Position,
    //     entity_positions: &HashMap<Position, ([f32; 4], Entity)>,
    //     terrain_positions: &HashMap<Position, [f32;4]>
    // ) -> bool {
    //     if entity_positions.contains_key(&position) || terrain_positions.contains_key(&position) {
    //         let info = entity_positions.get(&position);
    //         let info2 = terrain_positions.get(&position);
    //         if let Some(info) = info {
    //             if PERMISSIBLE_TILES.contains(&info.0) {
    //                 return true;
    //             }
    //         }

    //         if let Some(info) = info2 {
    //             if PERMISSIBLE_TILES.contains(&info) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    //     true
    // }
}
