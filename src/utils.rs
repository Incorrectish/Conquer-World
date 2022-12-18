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
const STUN_WELL_COOLDOWN: usize = 10;
const STUN_WELL_LINGER_VALUE: usize = 50;
const LASER_DAMAGE: usize = 5;
const LASER_AMOUNT: usize = 7;
const ASTEROID_DAMAGE: usize = 10;
const VULNERABLE_TIME_BASE: usize = 10;
const BOSS_3_RUSH_COOLDOWN: usize = 20;
const BOSS_3_MOVE_DELAY: usize = 2;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Boss {
    pub position: Position,
    pub color: [f32; 4],
    pub world_position: Position,
    pub health: usize,
    pub stun_well_cooldown: usize,
    pub asteroid_cooldown: usize,
    pub is_major: bool,
    pub offset: usize,
    pub boss_can_attack: bool,
    pub rush_info: (bool, Option<Direction>, [f32; 4], usize),
    pub vulnerable_time: usize,
    pub chase_rush_cooldown: usize,
    pub speed_delay: usize,
}

impl Boss {
    pub fn new(x: usize, y: usize, color: [f32; 4], world_position: Position,
    terrain_loc: &mut HashMap<Position, [f32; 4]>,
    ) -> Self {
        let mut offset: usize = 4;
        let is_major: bool = color == tile::MAJOR_BOSS;
        if is_major { offset = 5; }
        Boss {
            position: Position::new(x, y),
            color,
            world_position,
            health: BOSS_HEALTH,
            asteroid_cooldown: ASTEROID_COOLDOWN,
            stun_well_cooldown: STUN_WELL_COOLDOWN,
            is_major,
            offset,
            rush_info: (false, None, tile::BOSS_LASER_STAGE_1, 0),
            boss_can_attack: true,
            vulnerable_time: 0,
            chase_rush_cooldown: BOSS_3_RUSH_COOLDOWN,
            speed_delay: BOSS_3_MOVE_DELAY, 
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

    pub fn draw_boss(world: &mut World, canvas: &mut graphics::Canvas, index: usize) {
        let boss_size = (world.bosses[index].offset - 1) as i32;
        for i in -boss_size..=boss_size {
            for j in -boss_size..=boss_size {
                let mut color = world.bosses[index].color;
                if i == -boss_size || j == -boss_size || i == boss_size || j == boss_size { 
                    if world.bosses[index].vulnerable_time > 2 {
                        color = tile::BOSS_VULNERABLE;
                    } else if world.bosses[index].vulnerable_time == 2 {
                        color = tile::BOSS_RECOVERY_ONE;
                    } else if world.bosses[index].vulnerable_time == 1 {
                        color = tile::BOSS_RECOVERY_TWO;
                    } else {
                        color = tile::BOSS_SURROUNDINGS;
                    }
                }
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (world.bosses[index].position.x as i32 + i as i32) * TILE_SIZE.0 as i32,
                            (world.bosses[index].position.y as i32 + j as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color(color),
                );
            }
        }
    }

    pub fn draw_boss_stuff(world: &mut World, canvas: &mut graphics::Canvas, index: usize) {
        // for lasers in &mut world.boss_lasers {
        //     let mut special_case = true;
        //     if lasers.0 == Position::new(0,0) {
        //         if Boss::coin_flip(&mut world.rng) {
        //             special_case = false;
        //         }
        //     }

        //     if lasers.0.x == 0 && special_case {
        //         for i in 0..WORLD_SIZE.0 {
        //             canvas.draw(
        //                 &graphics::Quad,
        //                 graphics::DrawParam::new()
        //                     .dest_rect(graphics::Rect::new_i32(
        //                         (i) as i32 * TILE_SIZE.0 as i32,
        //                         ((lasers.0.y) as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
        //                         TILE_SIZE.0 as i32,
        //                         TILE_SIZE.1 as i32,
        //                     ))
        //                     .color(lasers.1),
        //             )
        //         }
        //     } else {
        //         for i in 0..WORLD_SIZE.0 {
        //             canvas.draw(
        //                 &graphics::Quad,
        //                 graphics::DrawParam::new()
        //                     .dest_rect(graphics::Rect::new_i32(
        //                         (lasers.0.x) as i32 * TILE_SIZE.0 as i32,
        //                        ((i) as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
        //                         TILE_SIZE.0 as i32,
        //                         TILE_SIZE.1 as i32,
        //                     ))
        //                     .color(lasers.1),
        //             )
        //         }
        //     }
        // }   

        // for asteroids in &mut world.boss_asteroids {
        //     for i in 0..=2 {
        //         for j in 0..=2 {
        //             canvas.draw(
        //                 &graphics::Quad,
        //                 graphics::DrawParam::new()
        //                     .dest_rect(graphics::Rect::new_i32(
        //                         (max(0, asteroids.0.x as i32 - 1) + i) as i32 * TILE_SIZE.0 as i32,
        //                         ((max(0, asteroids.0.y as i32 - 1) + j) as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
        //                         TILE_SIZE.0 as i32,
        //                         TILE_SIZE.1 as i32,
        //                     ))
        //                     .color(asteroids.1),
        //             )
        //         }
        //     }
            
        // }
        // if world.bosses[index].rush_info.0 {
        //     Self::draw_rush(world, index, world.bosses[index].rush_info.3, canvas);
        // }
        // Self::draw_stun_wells(world, canvas);

    }

    
    pub fn coin_flip(rng: &mut ChaCha8Rng) -> bool {
        random::rand_range(rng, 0, 2) > 0
    }

    pub fn attack(world: &mut World, num_laser: usize, index: usize) {
        // let lasers = &mut world.boss_lasers;
        // for index in (0..lasers.len()).rev() {
        //     match lasers[index].1 {
        //         tile::BOSS_LASER_STAGE_1 => {
        //             lasers[index].1 = tile::BOSS_LASER_STAGE_2;
        //         },

        //         tile::BOSS_LASER_STAGE_2 => {
        //             lasers[index].1 = tile::BOSS_LASER_REAL;
        //         },

        //         _ => {
        //             if lasers[index].2 == 0 {
        //                 lasers.remove(index);
        //             } else {
        //                 lasers[index].2 -= 1;
        //             }
        //         }
        //     }
        // }
        // Boss::generate_laser(world, num_laser);

        // let asteroids = &mut world.boss_asteroids;
        // for index in (0..asteroids.len()).rev() {
        //     match asteroids[index].1 {
        //         tile::BOSS_ASTEROID_STAGE_1 => {
        //             asteroids[index].1 = tile::BOSS_ASTEROID_STAGE_2;
        //         },

        //         tile::BOSS_ASTEROID_STAGE_2 => {
        //             asteroids[index].1 = tile::BOSS_ASTEROID_STAGE_3;
        //         },

        //         tile::BOSS_ASTEROID_STAGE_3 => {
        //             asteroids[index].1 = tile::BOSS_ASTEROID_REAL;
        //         },

        //         _ => {
        //             if asteroids[index].2 == 0 {
        //                 asteroids.remove(index);
        //             } else {
        //                 asteroids[index].2 -= 1;
        //             }
        //         }
        //     }
            
        //}
        
        // if world.bosses[index].color == tile::MAJOR_BOSS {
        //     if world.player.is_visible() && Boss::generate_asteroid(world, world.bosses[index].asteroid_cooldown) {
        //         world.bosses[index].asteroid_cooldown = ASTEROID_COOLDOWN;
        //     } else {
        //         world.bosses[index].asteroid_cooldown -= 1;
        //     }
        // }

        // if BOSS_ROOMS.contains(&world.world_position) {
        //     for laser in &world.boss_lasers {
        //         if (world.player.pos.x == laser.0.x 
        //         || world.player.pos.y == laser.0.y) 
        //         && laser.1 == tile::BOSS_LASER_REAL 
        //         && world.player.pos.y != 0
        //         && world.player.pos.x != 0
        //         && world.player.pos.y != WORLD_SIZE.1 as usize - 1
        //         && world.player.pos.x != WORLD_SIZE.0 as usize - 1 {
        //             world.player.damage(LASER_DAMAGE);
        //         }
        //     }

        //     for asteroid in &world.boss_asteroids {
        //         if (world.player.pos.x <= asteroid.0.x + 1 
        //         && world.player.pos.x >= asteroid.0.x - 1
        //         && world.player.pos.y <= asteroid.0.y + 1
        //         && world.player.pos.y >= asteroid.0.y - 1) 
        //         && asteroid.1 == tile::BOSS_ASTEROID_REAL {
        //             world.player.damage(ASTEROID_DAMAGE);
        //         } 
        //     }
        // }

        // Boss::generate_column_laser(world, index);
        // Boss::chase_player(world, index);
        // Boss::generate_stun_well(world, index);
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
        if cooldown == 0 {
            world.boss_asteroids.push((world.player.pos, tile::BOSS_ASTEROID_STAGE_1, ASTEROID_LINGER_VALUE));
            return true;
        }
        return false;
    }

    pub fn draw_laser_column(world: &mut World, index: usize, canvas: &mut Canvas) {
        let boss_pos = world.bosses[index].position;
        let mut laser_width: i32 = -2;
        if world.bosses[index].is_major { laser_width = -3; }
        if let Some(laser_pos) = world.boss_column_laser {
            for j in laser_width..=laser_width.abs() {
                for i in 0..WORLD_SIZE.1 / 2 {
                    let mut x: i32 = laser_pos.0.x as i32;
                    let mut y: i32 = laser_pos.0.y as i32;
                    if laser_pos.0.x > boss_pos.x {
                        y = y + j;
                        x = x + i as i32;
                    } else if laser_pos.0.x < boss_pos.x {
                        y = y + j;
                        x = x - i as i32;
                    } else if laser_pos.0.y > boss_pos.y {
                        y = y + i as i32;
                        x = x + j;
                    } else if laser_pos.0.y < boss_pos.y {
                        y = y - i as i32;
                        x = x + j;
                    }

                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                x  * TILE_SIZE.0 as i32,
                                (y  + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(tile::BOSS_LASER_REAL),
                    )
                }
            }
        }
    }   

    pub fn generate_column_laser(world: &mut World, index: usize) {
        let mut offset = world.bosses[index].offset;
        let boss_pos = world.bosses[index].position;
        let can_attack = &mut world.bosses[index].boss_can_attack;
        if let Some(curr_pos) = &mut world.boss_column_laser {
            let pos = curr_pos.0;
            match curr_pos.1 {
                Direction::North => {
                    if boss_pos.y - offset == 0 {
                        world.boss_column_laser = None;
                    } else {
                        world.bosses[index].position.y = boss_pos.y - 1;
                        curr_pos.0.y = pos.y - 1;    
                    }
                },

                Direction::South => {
                    if boss_pos.y + offset == WORLD_SIZE.1 as usize - 1 {
                        world.boss_column_laser = None;
                    } else {
                        world.bosses[index].position.y = boss_pos.y + 1;
                        curr_pos.0.y = pos.y + 1;    
                    }
                },

                Direction::East => {
                    if boss_pos.x + offset == WORLD_SIZE.0 as usize - 1 {
                        world.boss_column_laser = None;
                    } else {
                        world.bosses[index].position.x = boss_pos.x + 1;
                        curr_pos.0.x = pos.x + 1;    
                    }
                },

                Direction::West => {
                    if boss_pos.x - offset == 0 {
                        world.boss_column_laser = None;
                    } else {
                        world.bosses[index].position.x = boss_pos.x - 1;
                        curr_pos.0.x = pos.x - 1;    
                    }
                },
            }
        } else if *can_attack {
            let boss_delta = (boss_pos.x as i32 - world.player.pos.x as i32, boss_pos.y as i32 - world.player.pos.y as i32);
            let mut new_position: (Position, Direction);
            if boss_delta.0.abs() > boss_delta.1.abs() {
                if boss_delta.0 > 0 {
                    new_position = (Position::new(boss_pos.x, boss_pos.y - offset), Direction::West);
                } else {
                    new_position = (Position::new(boss_pos.x, boss_pos.y - offset), Direction::East);
                }

                if boss_delta.1 < 0 {
                    new_position.0.y = boss_pos.y + offset;
                }
            } else {
                if boss_delta.1 < 0 {
                    new_position = (Position::new(boss_pos.x - offset, boss_pos.y), Direction::South);
                } else {
                    new_position = (Position::new(boss_pos.x - offset, boss_pos.y), Direction::North);
                }

                if boss_delta.0 < 0 {
                    new_position.0.x = boss_pos.x + offset;
                }
            }
            *can_attack = false;
            world.boss_column_laser = Some(new_position);
        } else {
            Self::return_boss_to_center(world, index);
        }
    }

    pub fn return_boss_to_center(world: &mut World, index: usize) {
        let boss_pos = world.bosses[index].position;
        if boss_pos.x < WORLD_SIZE.0 as usize / 2  {
            world.bosses[index].position.x += 1;
        } else if boss_pos.x > WORLD_SIZE.0 as usize / 2 {
            world.bosses[index].position.x -= 1;
        } else if boss_pos.y > WORLD_SIZE.1 as usize / 2 {
            world.bosses[index].position.y -= 1;
        } else if boss_pos.y < WORLD_SIZE.1 as usize / 2 {
            world.bosses[index].position.y += 1;
        } else {
            if world.bosses[index].vulnerable_time == 0 {
                world.bosses[index].boss_can_attack = true;
            } else {
                world.bosses[index].vulnerable_time -= 1;
            }
        }
    }   
    
    pub fn generate_stun_well(world: &mut World, index: usize) {
        if world.bosses[index].stun_well_cooldown == 0 { 
            world.bosses[index].stun_well_cooldown = STUN_WELL_COOLDOWN;
            let mut well_size = 3;
            if Self::coin_flip(&mut world.rng) {
                well_size = 5;
            }
            let x = random::rand_range(&mut world.rng, 5, WORLD_SIZE.0) as usize;
            let y = random::rand_range(&mut world.rng, 5, WORLD_SIZE.1) as usize;
            world.stun_wells.push((Position::new(x,y), tile::STUN_WELL_INDICATOR, well_size, STUN_WELL_LINGER_VALUE));
        } else {
            for index in (0..world.stun_wells.len()).rev() {
                let well = world.stun_wells[index];
                if well.1 == tile::STUN_WELL_INDICATOR {
                    world.stun_wells[index].1 = tile::STUN_WELL_REAL;
                } else {
                    if well.3 == 0 {
                        world.stun_wells.remove(index);
                        
                    } else {
                        world.stun_wells[index].3 -= 1;
                    }
                }
            }
            world.bosses[index].stun_well_cooldown -= 1;
        }
    }
    
    pub fn draw_stun_wells(world: &mut World, canvas: &mut Canvas) {
        for well in &world.stun_wells {
            let len = well.2 as i32 - 1;
            let pos = well.0;
            for i in -len..=len {
                for j in -len..=len {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (pos.x as i32 + i) * TILE_SIZE.0 as i32,
                                ((pos.y as i32 + j) + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(well.1),
                    )
                }
            }
        }
    }

    pub fn generate_safe_spot(world: &mut World, index: usize) {

    }
    
    pub fn chase_player(world: &mut World, index: usize) {
        let boss_pos = world.bosses[index].position;
        let boss_delta = (boss_pos.x as i32 - world.player.pos.x as i32, boss_pos.y as i32 - world.player.pos.y as i32);
        if (world.bosses[index].rush_info.0 || 
        (boss_delta.0.abs() <= 3 || boss_delta.1.abs() <= 3))
        && world.bosses[index].chase_rush_cooldown == 0 {
            Boss::rush_player(world, index, boss_delta.0, boss_delta.1);
        } else if world.bosses[index].boss_can_attack && world.bosses[index].speed_delay == 0 {
            if boss_delta.0.abs() > boss_delta.1.abs() {
                if boss_delta.0 < 0 {
                    world.bosses[index].position.x += 1;
                } else {
                    world.bosses[index].position.x -= 1;
                }

            } else {
                if boss_delta.1 < 0 {
                    world.bosses[index].position.y += 1;
                } else {
                    world.bosses[index].position.y -= 1;

                }
            }
            world.bosses[index].speed_delay = BOSS_3_MOVE_DELAY;        
            if (world.bosses[index].chase_rush_cooldown != 0) {world.bosses[index].chase_rush_cooldown -= 1;}
    
        } else {
            if world.bosses[index].vulnerable_time != 0 {
                world.bosses[index].vulnerable_time -= 1;
                if (world.bosses[index].vulnerable_time == 0) {
                    world.bosses[index].boss_can_attack = true;
                }
            } else {
                world.bosses[index].speed_delay -= 1;
                if (world.bosses[index].chase_rush_cooldown != 0) {world.bosses[index].chase_rush_cooldown -= 1;}
            }
        }
    }

    pub fn rush_player(world: &mut World, index: usize, x_delta: i32, y_delta: i32) {
        if !world.bosses[index].rush_info.0 {
            if x_delta.abs() > y_delta.abs() {
                if x_delta < 0 {
                    world.bosses[index].rush_info.1 = Some(Direction::East);
                    world.bosses[index].rush_info.3 = WORLD_SIZE.0 as usize - 1 - world.bosses[index].position.x;
                } else {
                    world.bosses[index].rush_info.1 = Some(Direction::West);
                    world.bosses[index].rush_info.3 =  world.bosses[index].position.x;

                }
            } else {
                if y_delta < 0 {
                    world.bosses[index].rush_info.1 = Some(Direction::South);
                    world.bosses[index].rush_info.3 = WORLD_SIZE.1 as usize - 1 - world.bosses[index].position.y;

                } else {
                    world.bosses[index].rush_info.1 = Some(Direction::North);
                    world.bosses[index].rush_info.3 = world.bosses[index].position.y;
                }

            }
            world.bosses[index].rush_info.0 = true;
        } else {
            let stage = world.bosses[index].rush_info.2;
            let offset = world.bosses[index].offset;
            let mut length: usize;
            match stage {
                tile::BOSS_LASER_STAGE_1 => {
                    world.bosses[index].rush_info.2 = tile::BOSS_LASER_STAGE_2;
                },

                tile::BOSS_LASER_STAGE_2 => {
                    world.bosses[index].rush_info.2 = tile::BOSS_LASER_REAL;
                },

                _ => {
                    if let Some(direction) = world.bosses[index].rush_info.1 {
                        match direction {
                            Direction::North => {
                                world.bosses[index].position.y = 0 + offset;
                            },

                            Direction::South => {
                                world.bosses[index].position.y = WORLD_SIZE.1 as usize - offset - 1;

                            },

                            Direction::West => {
                                world.bosses[index].position.x = 0 + offset;
                            },

                            Direction::East => {
                                world.bosses[index].position.x = WORLD_SIZE.0 as usize - 1 - offset;
                            }
                        }   
                    }
                    world.bosses[index].chase_rush_cooldown = BOSS_3_RUSH_COOLDOWN;
                    world.bosses[index].rush_info.2 = tile::BOSS_LASER_STAGE_1;
                    world.bosses[index].vulnerable_time = VULNERABLE_TIME_BASE;
                    world.bosses[index].boss_can_attack = false;
                }
            }
        }
    }
    
    pub fn draw_rush(world: &mut World, index: usize, distance: usize, canvas: &mut Canvas) {
        if let Some(direction) = world.bosses[index].rush_info.1 {
            let mut rush_dist = distance;
            let after_rush = !world.bosses[index].boss_can_attack;
            let mut width: i32 = -3;
            let mut color = world.bosses[index].rush_info.2;
            if world.bosses[index].offset == 5 {
                width = -3;
            }
            if after_rush {
                world.bosses[index].rush_info.0 = false;
                // rush_dist = WORLD_SIZE.0 as usize - world.bosses[index].offset;
                color = tile::FIRE_TERTIARY;
            }
            
            for i in 0..rush_dist as i32 {
                for j in width..=(-width) {
                    let mut x = world.bosses[index].position.x as i32;
                    let mut y = world.bosses[index].position.y as i32;
                    match direction {
                        Direction::North => {
                            x = x + j;
                            if after_rush {
                                y = y + i;
                            } else {
                                y = y - i;
                            }
                        },
        
                        Direction::South => {
                            x = x + j;
                            if after_rush {
                                y = y - i;
                            } else {
                                y = y + i;
                            }
                        },
        
                        Direction::West => {
                            if after_rush {
                                x = x + i;
                            } else {
                                x = x - i;
                            }
                            y = y + j;
                        },
        
                        Direction::East => {
                            if after_rush {
                                x = x - i;
                            } else {
                                x = x + i;
                            }
                            y = y + j;
                        }
                    } 
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                x * TILE_SIZE.0 as i32,
                                (y + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(color),
                    )
                }
            }
              
        }
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
        world.bosses.remove(index);
        // when kill is implemented this should reopen doors
        world.boss_defeated[world.world_position.y][world.world_position.x] = true;
        World::toggle_doors(&mut world.terrain_map, world.world_position,
            world.player.pos, world.boss_defeated);
    }
}


#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]

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

