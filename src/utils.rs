use crate::{
    enemy::Enemy, 
    tile, 
    direction::Direction, 
    world::World,
    BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE,
    entity::Entity,
    random,
};
use std::collections::HashMap;
use rand::rngs::ThreadRng;
use rand::rngs;
use ggez::graphics::{self, Canvas};

const BOSS_HEALTH: usize = 100;

pub struct Boss {
    pub position: Position,
    pub color: [f32; 4],
    pub surrounding: Vec<Option<Enemy>>,
    pub world_position: Position,
    pub health: usize,
}

impl Boss {
    pub fn new(x: usize, y: usize, color: [f32; 4], world_position: Position,
    entity_loc: &mut HashMap<Position, ([f32; 4], Entity)>,
    ) -> Self {

        let mut surrounding: Vec<Option<Enemy>> = Vec::new();
        let mut index = 0;
        if color == tile::MAJOR_BOSS {
            for i in 0..=6 {
                for j in 0..=6 {
                    if i == 0 || j == 0 || i == 6 || j == 6 {
                        surrounding.push(Some(Enemy::minor_boss(x+i, y+j, world_position)));
                        entity_loc.insert(Position::new(x+i, y+j), 
                        (tile::BOSS_SURROUNDINGS, Entity::Enemy)
                    );
                    index += 1;
                    } else { 
                        entity_loc.insert(Position::new(x+i, y+j), 
                        (tile::MAJOR_BOSS, Entity::Enemy));
                    }
                }
            }
        } else {

        }
        Boss {
            position: Position::new(x, y),
            color,
            surrounding,
            world_position,
            health: BOSS_HEALTH,
        }
    }
    
    pub fn update(world: &mut World) {
        // for index in (0..world.bosses.len()).rev() {
        //     if world.bosses[index].health <= 0 {
        //         Enemy::kill(world, index);
        //     } else {
        //         if world.world_position == world.bosses[index].world_position {
        //             Self::move_boss(index, world);
        //         }
        //     }
        // }
    }

    pub fn draw_lasers(world: &mut World, canvas: &mut graphics::Canvas) {
        for lasers in &mut world.boss_lasers {
            
        }   
    }

    pub fn coin_flip(rng: &mut ThreadRng) -> bool {
        random::rand_range(rng, 0, 2) > 0
    }

    pub fn grid_attack(stage: usize, world: &mut World, num_laser: usize) {
        Boss::generate_laser(world, num_laser);
        let lasers = &mut world.boss_lasers;
        for index in (0..lasers.len()).rev() {
            match lasers[index].1 {
                tile::BOSS_LASER_STAGE_1 => {
                    lasers[index].1 = tile::BOSS_LASER_STAGE_2;
                },

                tile::BOSS_LASER_STAGE_2 => {
                    lasers[index].1 = tile::BOSS_LASER_STAGE_3;
                },

                tile::BOSS_LASER_STAGE_3 => {
                    lasers[index].1 = tile::BOSS_LASER_REAL;
                },

                _ => {
                    lasers.remove(index);
                }
            }
        }
    }

    pub fn generate_laser(world: &mut World, num_lasers: usize) {
        let rng = &mut world.rng;
        for _ in 0..num_lasers {
            let coord: Position;
            if Boss::coin_flip(rng) {
                coord = Position::new(0,random::rand_range(rng, 0, BOARD_SIZE.1) as usize);
            } else {
                coord = Position::new(random::rand_range(rng, 0, BOARD_SIZE.0) as usize, 0);
            }
            world.boss_lasers.push((coord, tile::BOSS_LASER_STAGE_1));
        }
    }
    pub fn kill(world: &mut World, index: usize) {
        let pos = world.bosses[index].position;
        let world_pos = world.bosses[index].world_position;
        let curr_world = &mut world.entity_map[world_pos.y][world_pos.x];
        for enemy_type in &world.bosses[index].surrounding {
            if let Some(enemy) = enemy_type {
                curr_world.remove(&enemy.pos);
            }
        }
        curr_world.remove(&pos);
        world.bosses.remove(index);
        // when kill is implemented this should reopen doors
        world.boss_defeated[world.world_position.y][world.world_position.x] = true;
        World::toggle_doors(&mut world.terrain_map, world.world_position,
            world.player.pos, world.boss_defeated);
    }
}


#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]

pub struct Position {
    pub x: usize, 
    pub y: usize,
}
impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Position {
            x,
            y,
        }
    }
}

