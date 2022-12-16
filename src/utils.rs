use crate::{
    enemy::Enemy, 
    tile, 
    direction::Direction, 
    world::World,
    BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE,
    entity::Entity,
    random,
    world::BOSS_ROOMS,
};
use std::{collections::HashMap, cmp::max};
use rand::rngs::ThreadRng;
use rand::rngs;
use ggez::graphics::{self, Canvas};
use rand_chacha::ChaCha8Rng;

const BOSS_HEALTH: usize = 100;
const LASER_LINGER_VALUE: usize = 3;
const ASTEROID_LINGER_VALUE: usize = 5;
const ASTEROID_COOLDOWN: usize = 20;
const LASER_DAMAGE: usize = 5;
const LASER_AMOUNT: usize = 7;
const ASTEROID_DAMAGE: usize = 10;

pub struct Boss {
    pub position: Position,
    pub color: [f32; 4],
    pub surrounding: Vec<Position>,
    pub world_position: Position,
    pub health: usize,
    pub asteroid_cooldown: usize,
}

impl Boss {
    pub fn new(x: usize, y: usize, color: [f32; 4], world_position: Position,
    terrain_loc: &mut HashMap<Position, [f32; 4]>,
    ) -> Self {

        let mut surrounding: Vec<Position> = Vec::new();
        
        if color == tile::MAJOR_BOSS {
            for i in 0..=6 {
                surrounding.push(Position::new(x+i, y));
                terrain_loc.insert(Position::new(x+i, y), tile::BOSS_SURROUNDINGS);
                surrounding.push(Position::new(x, y+i));
                terrain_loc.insert(Position::new(x, y+i), tile::BOSS_SURROUNDINGS);
                surrounding.push(Position::new(x+6, y+i));
                terrain_loc.insert(Position::new(x+6, y+i), tile::BOSS_SURROUNDINGS);
                surrounding.push(Position::new(x+i, y+6));
                terrain_loc.insert(Position::new(x+i, y+6), tile::BOSS_SURROUNDINGS);
            }

        } else {
            for i in 0..=5 {
                surrounding.push(Position::new(x+i, y));
                terrain_loc.insert(Position::new(x+i, y), tile::BOSS_SURROUNDINGS);
                surrounding.push(Position::new(x, y+i));
                terrain_loc.insert(Position::new(x, y+i), tile::BOSS_SURROUNDINGS);
                surrounding.push(Position::new(x+5, y+i));
                terrain_loc.insert(Position::new(x+5, y+i), tile::BOSS_SURROUNDINGS);
                surrounding.push(Position::new(x+i, y+5));
                terrain_loc.insert(Position::new(x+i, y+5), tile::BOSS_SURROUNDINGS);
            }
        }
        Boss {
            position: Position::new(x, y),
            color,
            surrounding,
            world_position,
            health: BOSS_HEALTH,
            asteroid_cooldown: 0,
        }
    }
    
    pub fn update(world: &mut World) {
        for index in (0..world.bosses.len()).rev() {
            if world.bosses[index].world_position == world.world_position {
                Boss::attack(world, LASER_AMOUNT, index);
            }
            if world.bosses[index].health <= 0 {
                Boss::kill(world, index);
            } 
        }
    }

    pub fn draw_boss_stuff(world: &mut World, canvas: &mut graphics::Canvas, index: usize) {
        for lasers in &mut world.boss_lasers {
            let mut special_case = true;
            if lasers.0 == Position::new(0,0) {
                if Boss::coin_flip(&mut world.rng) {
                    special_case = false;
                }
            }

            if lasers.0.x == 0 && special_case {
                for i in 0..WORLD_SIZE.0 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (i) as i32 * TILE_SIZE.0 as i32,
                                ((lasers.0.y) as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(lasers.1),
                    )
                }
            } else {
                for i in 0..WORLD_SIZE.0 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (lasers.0.x) as i32 * TILE_SIZE.0 as i32,
                               ((i) as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(lasers.1),
                    )
                }
            }
        }   

        for asteroids in &mut world.boss_asteroids {
            for i in 0..=2 {
                for j in 0..=2 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (max(0, asteroids.0.x as i32 - 1) + i) as i32 * TILE_SIZE.0 as i32,
                                ((max(0, asteroids.0.y as i32 - 1) + j) as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(asteroids.1),
                    )
                }
            }
            
        }
        
        if world.bosses[index].world_position == Position::new(3,3) {
            for i in 1..=5 {
                for j in 1..=5 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (world.bosses[index].position.x as i32 + i as i32) * TILE_SIZE.0 as i32,
                                (world.bosses[index].position.y as i32 + j as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(tile::MAJOR_BOSS),
                        )
                }
            }
        } else {
            for i in 1..=4 {
                for j in 1..=4 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (world.bosses[index].position.x as i32 + i as i32) * TILE_SIZE.0 as i32,
                                (world.bosses[index].position.y as i32 + j as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(tile::MINOR_BOSS),
                        )
                }
            }
        }

    }

    
    pub fn coin_flip(rng: &mut ChaCha8Rng) -> bool {
        random::rand_range(rng, 0, 2) > 0
    }

    pub fn attack(world: &mut World, num_laser: usize, index: usize) {
        let lasers = &mut world.boss_lasers;
        for index in (0..lasers.len()).rev() {
            match lasers[index].1 {
                tile::BOSS_LASER_STAGE_1 => {
                    lasers[index].1 = tile::BOSS_LASER_STAGE_2;
                },

                tile::BOSS_LASER_STAGE_2 => {
                    lasers[index].1 = tile::BOSS_LASER_REAL;
                },

                _ => {
                    if lasers[index].2 == 0 {
                        lasers.remove(index);
                    } else {
                        lasers[index].2 -= 1;
                    }
                }
            }
        }
        Boss::generate_laser(world, num_laser);

        let asteroids = &mut world.boss_asteroids;
        for index in (0..asteroids.len()).rev() {
            match asteroids[index].1 {
                tile::BOSS_ASTEROID_STAGE_1 => {
                    asteroids[index].1 = tile::BOSS_ASTEROID_STAGE_2;
                },

                tile::BOSS_ASTEROID_STAGE_2 => {
                    asteroids[index].1 = tile::BOSS_ASTEROID_STAGE_3;
                },

                tile::BOSS_ASTEROID_STAGE_3 => {
                    asteroids[index].1 = tile::BOSS_ASTEROID_REAL;
                },

                _ => {
                    if asteroids[index].2 == 0 {
                        asteroids.remove(index);
                    } else {
                        asteroids[index].2 -= 1;
                    }
                }
            }
        }
        
        if world.bosses[index].color == tile::MAJOR_BOSS {
            if world.player.is_visible() && Boss::generate_asteroid(world, world.bosses[index].asteroid_cooldown) {
                world.bosses[index].asteroid_cooldown = 0;
            } else {
                world.bosses[index].asteroid_cooldown += 1;
            }
        }

        if BOSS_ROOMS.contains(&world.world_position) {
            for laser in &world.boss_lasers {
                if (world.player.pos.x == laser.0.x 
                || world.player.pos.y == laser.0.y) 
                && laser.1 == tile::BOSS_LASER_REAL 
                && world.player.pos.y != 0
                && world.player.pos.x != 0
                && world.player.pos.y != WORLD_SIZE.1 as usize - 1
                && world.player.pos.x != WORLD_SIZE.0 as usize - 1 {
                    world.player.damage(LASER_DAMAGE);
                }
            }

            for asteroid in &world.boss_asteroids {
                if (world.player.pos.x <= asteroid.0.x + 1 
                && world.player.pos.x >= asteroid.0.x - 1
                && world.player.pos.y <= asteroid.0.y + 1
                && world.player.pos.y >= asteroid.0.y - 1) 
                && asteroid.1 == tile::BOSS_ASTEROID_REAL {
                    world.player.damage(ASTEROID_DAMAGE);
                } 
            }
        }
    }

    pub fn generate_laser(world: &mut World, num_lasers: usize) {
        let rng = &mut world.rng;
        for _ in 0..num_lasers {
            let coord: Position = if Boss::coin_flip(rng) {
                Position::new(0,random::rand_range(rng, 0, BOARD_SIZE.1) as usize)
            } else {
                Position::new(random::rand_range(rng, 0, BOARD_SIZE.0) as usize, 0)
            };
            world.boss_lasers.push((coord, tile::BOSS_LASER_STAGE_1, LASER_LINGER_VALUE));
        }
    }
    
    pub fn generate_asteroid(world: &mut World, cooldown: usize) -> bool {
        if cooldown == ASTEROID_COOLDOWN {
            world.boss_asteroids.push((world.player.pos, tile::BOSS_ASTEROID_STAGE_1, ASTEROID_LINGER_VALUE));
            return true;
        }
        return false;
    }

    pub fn damage(world: &mut World, damage: usize, world_pos: Position) {
        for index in 0..world.bosses.len() {
            let boss = &mut world.bosses[index];
            if boss.world_position == world_pos {
                boss.health = max(0, boss.health - damage);
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        let pos = world.bosses[index].position;
        let world_pos = world.bosses[index].world_position;
        let curr_world = &mut world.terrain_map[world_pos.y][world_pos.x];
        for enemy_type in &world.bosses[index].surrounding {
            curr_world.remove(&enemy_type);
        }
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

