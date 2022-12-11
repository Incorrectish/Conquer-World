use crate::{direction::Direction, tile, world::World, WORLD_SIZE, utils::Position, entity::Entity, TILE_SIZE};
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
};
const ENEMY_HEALTH: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 1] = [tile::GRASS];

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

    pub fn draw(&self, canvas: &mut graphics::Canvas, world: &World) {
        let color = tile::ENEMY;
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(graphics::Rect::new_i32(
                    (self.pos.x as i32 - world.x_offset as i32) * TILE_SIZE.0 as i32,
                    (self.pos.y as i32 - world.y_offset as i32) * TILE_SIZE.1 as i32,
                    TILE_SIZE.0 as i32,
                    TILE_SIZE.1 as i32
                ))
                .color(color),
        )
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
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        // for now all it does is remove the tile on the world "board"
        world.world[world.enemies[index].pos.y][world.enemies[index].pos.x] =
            world.board[world.enemies[index].pos.y][world.enemies[index].pos.x];
        world.enemies.remove(index);
    }

    pub fn move_enemy(world: &mut World, index: usize, pos: &mut Position) {
        let delta_pos = (world.player.pos.x - world.enemies[index].pos.x, world.player.pos.y - world.enemies[index].pos.y);
        
    }

    pub fn can_travel_to(
        tile: [f32; 4], 
        position: Position,
        entity_positions: &HashMap<Position, ([f32; 4], Option<Entity>)>,
        terrain_positions: &HashMap<Position, [f32;4]>
    ) -> bool {
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
