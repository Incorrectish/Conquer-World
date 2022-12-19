use crate::{
    direction::Direction, enemy::Enemy, entity::Entity, random, tile, world::World,
    world::BOSS_ROOMS, BOARD_SIZE, TILE_SIZE, UNIVERSAL_OFFSET, WORLD_SIZE, 
    player::MAX_PLAYER_HEALTH,
};
use ggez::{graphics::{self, Canvas}, mint::Point2, glam::*};
use rand::rngs;
use rand::rngs::ThreadRng;
use rand_chacha::ChaCha8Rng;
use std::{cmp::max, collections::HashMap};
use ggez::glam::*;

pub const BOSS_HEALTH: usize = 1000;
pub const MAJOR_BOSS_HEALTH: usize = 5000;
const LASER_LINGER_VALUE: usize = 3;
const ASTEROID_LINGER_VALUE: usize = 5;
const ASTEROID_COOLDOWN: usize = 20;
const STUN_WELL_COOLDOWN: usize = 10;
const STUN_WELL_LINGER_VALUE: usize = 50;
const LASER_DAMAGE: usize = 5;
const COLUMN_LASER_DAMAGE: usize = 20;
const LASER_AMOUNT: usize = 7;
const ASTEROID_DAMAGE: usize = 10;
const VULNERABLE_TIME_BASE: usize = 15;
const BOSS_3_RUSH_COOLDOWN: usize = 20;
const BOSS_3_MOVE_DELAY: usize = 2;
const SAFE_SPOT_ATTACK_COOLDOWN: usize = 20;
const SAFE_SPOT_TIME: usize = 10;
const BOSS_COLLISION_DAMAGE: usize = 10;
const STUN_WELL_STUN_TIME: usize = 2;
const SHIELD_HITS_NEEDED: usize = 3;
const ENEMY_SPAWN_COOLDOWN: usize = 10;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Boss {
    pub position: Position,
    pub color: [f32; 4],
    pub world_position: Position,
    pub health: usize,
    pub laser_amount: usize,
    pub stun_well_cooldown: usize,
    pub asteroid_cooldown: usize,
    pub safe_spot_cooldown: usize,
    pub enemy_spawn_cooldown: usize,
    pub is_major: bool,
    pub offset: usize,
    pub boss_can_attack: bool,
    pub rush_info: (bool, Option<Direction>, [f32; 4], usize), //is rushing, direction, color of rush indicator, length of indicator
    pub vulnerable_time: usize,
    pub shield_health: usize,
    pub chase_rush_cooldown: usize,
    pub speed_delay: usize,
    pub first_enter: bool,
}

impl Boss {
    pub fn new(
        x: usize,
        y: usize,
        color: [f32; 4],
        world_position: Position,
        terrain_loc: &mut HashMap<Position, [f32; 4]>,
    ) -> Self {
        let mut offset: usize = 4;
        let is_major: bool = color == tile::MAJOR_BOSS;
        let mut health = BOSS_HEALTH;
        if is_major {
            offset = 5;
        }
        if color == tile::MAJOR_BOSS {
            health = MAJOR_BOSS_HEALTH;
        }
        Boss {
            position: Position::new(x, y),
            color,
            world_position,
            health,
            laser_amount: LASER_AMOUNT,
            safe_spot_cooldown: SAFE_SPOT_ATTACK_COOLDOWN,
            asteroid_cooldown: ASTEROID_COOLDOWN,
            stun_well_cooldown: STUN_WELL_COOLDOWN,
            enemy_spawn_cooldown: 0,
            is_major,
            offset,
            first_enter: true,
            rush_info: (false, None, tile::BOSS_LASER_STAGE_1, 0),
            boss_can_attack: true,
            vulnerable_time: 0,
            shield_health: SHIELD_HITS_NEEDED,
            chase_rush_cooldown: BOSS_3_RUSH_COOLDOWN,
            speed_delay: BOSS_3_MOVE_DELAY,
        }
    }

    pub fn spawn_enemies(world: &mut World, rng: &mut ChaCha8Rng, index: usize) {
        let curr_entity_map = &world.enemies_map[world.world_position.y as usize][world.world_position.x as usize]; 
        if world.world_position == BOSS_ROOMS[0] {
            if curr_entity_map.is_empty() {
                if world.bosses[index].vulnerable_time == 0 && world.bosses[index].boss_can_attack && !world.bosses[index].first_enter{
                    world.bosses[index].vulnerable_time = VULNERABLE_TIME_BASE + 10;
                    world.bosses[index].boss_can_attack = false;
                } else if world.bosses[index].vulnerable_time != 0 {
                    world.bosses[index].vulnerable_time -= 1;
                } else {
                    world.bosses[index].first_enter = false;
                    world.bosses[index].boss_can_attack = true;
                    for i in 0..13 {
                        let mut pos = world.bosses[index].position;
                        let size = world.bosses[index].offset - 1;
                        while Self::pos_inside_boss(world, Position::new(pos.x + size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x - size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y + size), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y - size), world.world_position) {
                                let x = random::rand_range(rng, 3, WORLD_SIZE.0 - 5) as usize;
                                let y = random::rand_range(rng, 3, WORLD_SIZE.1-  5) as usize;
                                pos = Position::new(x,y);
                        }
                        if i <= 3 {
                            world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::major_enemy(
                                pos.x as usize,
                                pos.y as usize,
                                world.world_position),);
                            for h in 0..3 {
                                for j in 0..3 {
                                    world.entity_map[world.world_position.y][world.world_position.x].insert(
                                        Position::new(pos.x as usize + h, pos.y as usize + j), (tile::MAJOR_ENEMY, Entity::Enemy));
                                }
                            }

                        } else {
                            world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::bomber(
                                pos.x as usize,
                                pos.y as usize,
                                world.world_position),);
                            world.entity_map[world.world_position.y][world.world_position.x].insert(pos, (tile::BOMBER_ENEMY, Entity::Enemy));
                        }
                    }
                }
            } 
        } else if world.world_position == BOSS_ROOMS[1] {
            if curr_entity_map.is_empty() && world.bosses[index].enemy_spawn_cooldown == 0 {
                for i in 0..9 {
                    let mut pos = world.bosses[index].position;
                    let size = world.bosses[index].offset - 1;
                        while Self::pos_inside_boss(world, Position::new(pos.x + size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x - size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y + size), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y - size), world.world_position) {
                            let x = random::rand_range(rng, 3, WORLD_SIZE.0 - 5) as usize;
                            let y = random::rand_range(rng, 3, WORLD_SIZE.1 - 5) as usize;
                            pos = Position::new(x,y);
                        }
                    if i <= 2 {
                        world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::major_enemy(
                            pos.x as usize,
                            pos.y as usize,
                            world.world_position),);
                        for h in 0..3 {
                            for j in 0..3 {
                                world.entity_map[world.world_position.y][world.world_position.x].insert(
                                    Position::new(pos.x as usize + h, pos.y as usize + j), (tile::MAJOR_ENEMY, Entity::Enemy));
                            }
                        }

                    } else {
                        world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::chasing(
                            pos.x as usize,
                            pos.y as usize,
                            world.world_position),);
                        world.entity_map[world.world_position.y][world.world_position.x].insert(pos, (tile::CHASING_ENEMY, Entity::Enemy));
                    }
                }
                world.bosses[index].enemy_spawn_cooldown = ENEMY_SPAWN_COOLDOWN;
            } else {
                if world.bosses[index].enemy_spawn_cooldown != 0 {
                    world.bosses[index].enemy_spawn_cooldown -= 1;
                }
            }
        } else if world.world_position == BOSS_ROOMS[4] {
            if curr_entity_map.is_empty() && world.bosses[index].enemy_spawn_cooldown == 0 {
                for i in 0..10 {
                    let mut pos = world.bosses[index].position;
                    let size = world.bosses[index].offset - 1;
                        while Self::pos_inside_boss(world, Position::new(pos.x + size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x - size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y + size), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y - size), world.world_position) {
                            let x = random::rand_range(rng, 3, WORLD_SIZE.0) as usize;
                            let y = random::rand_range(rng, 3, WORLD_SIZE.1) as usize;
                            pos = Position::new(x,y);
                        }
                    world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::chasing(
                        pos.x as usize,
                        pos.y as usize,
                        world.world_position),);
                    world.entity_map[world.world_position.y][world.world_position.x].insert(pos, (tile::CHASING_ENEMY, Entity::Enemy));
                }
                world.bosses[index].enemy_spawn_cooldown = ENEMY_SPAWN_COOLDOWN;
            } else {
                if world.bosses[index].enemy_spawn_cooldown != 0 {
                    world.bosses[index].enemy_spawn_cooldown -= 1;
                }
            }
        } else if world.world_position == BOSS_ROOMS[2] {
            if curr_entity_map.is_empty() && world.bosses[index].enemy_spawn_cooldown == 0 {
                for i in 0..33 {
                    let mut pos = world.bosses[index].position;
                    let size = world.bosses[index].offset - 2;
                        while Self::pos_inside_boss(world, Position::new(pos.x + size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x - size, pos.y), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y + size), world.world_position) ||
                            Self::pos_inside_boss(world, Position::new(pos.x, pos.y - size), world.world_position) {
                            let x = random::rand_range(rng, 3, WORLD_SIZE.0 - 5) as usize;
                            let y = random::rand_range(rng, 3, WORLD_SIZE.1 - 5) as usize;
                            pos = Position::new(x,y);
                        }
                    if i <= 3 {
                        world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::major_enemy(
                            pos.x as usize,
                            pos.y as usize,
                            world.world_position),);
                        for h in 0..3 {
                            for j in 0..3 {
                                world.entity_map[world.world_position.y][world.world_position.x].insert(
                                    Position::new(pos.x as usize + h, pos.y as usize + j), (tile::MAJOR_ENEMY, Entity::Enemy));
                            }
                        }

                    } else if i <= 18{
                        world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::chasing(
                            pos.x as usize,
                            pos.y as usize,
                            world.world_position),);
                        world.entity_map[world.world_position.y][world.world_position.x].insert(pos, (tile::CHASING_ENEMY, Entity::Enemy));

                    } else {
                        world.enemies_map[world.world_position.y as usize][world.world_position.x as usize].push(Enemy::bomber(
                            pos.x as usize,
                            pos.y as usize,
                            world.world_position),);
                        world.entity_map[world.world_position.y][world.world_position.x].insert(pos, (tile::BOMBER_ENEMY, Entity::Enemy));

                    }
                } 
                world.bosses[index].enemy_spawn_cooldown = ENEMY_SPAWN_COOLDOWN;
            } else {
                if world.bosses[index].enemy_spawn_cooldown != 0 {
                    world.bosses[index].enemy_spawn_cooldown -= 1;
                }
            }
        }  
    }

    pub fn update(world: &mut World, rng: &mut ChaCha8Rng) {
        for index in (0..world.bosses.len()).rev() {
            if world.bosses[index].world_position == world.world_position {
                Self::attack(world, index, rng);
            }
            if world.bosses[index].health <= 0 {
                Self::kill(world, index);
            }
        }
    }

    pub fn draw_boss(world: &mut World, canvas: &mut graphics::Canvas, index: usize) {
        let boss_size = (world.bosses[index].offset - 1) as i32;
        for i in -boss_size..=boss_size {
            for j in -boss_size..=boss_size {
                let pos = world.bosses[index].position;
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
                if let Some(spot) = world.boss_vulnerable_spot {
                    if Position::new((pos.x as i32 + i) as usize, (pos.y as i32+ j) as usize) == spot {
                        color = tile::BOSS_VULNERABLE
                    }
                }

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            (pos.x as i32 + i as i32) * TILE_SIZE.0 as i32,
                            (pos.y as i32
                                + j as i32
                                + UNIVERSAL_OFFSET as i32)
                                * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color(color),
                );
            }
        }
    }

    pub fn draw_boss_stuff(world: &mut World, canvas: &mut graphics::Canvas, index: usize, rng: &mut ChaCha8Rng) {
        if world.world_position == BOSS_ROOMS[0] {
            Self::draw_lasers(world, canvas, rng);
        } else if world.world_position == BOSS_ROOMS[1] {
            Self::draw_laser_column(world, index, canvas);
            Self::draw_asteroids(world, index, canvas);
        } else if world.world_position == BOSS_ROOMS[3] {
            if world.bosses[index].rush_info.0 {
                Self::draw_rush(world, index, world.bosses[index].rush_info.3, canvas);
            }
            Self::draw_stun_wells(world, canvas);
        } else if world.world_position == BOSS_ROOMS[4] {
            Self::draw_safe_spot(world, canvas);
        } else if world.world_position == BOSS_ROOMS[2] {
            Self::draw_stun_wells(world, canvas);
            Self::draw_laser_column(world, index, canvas);
            Self::draw_asteroids(world, index, canvas);
            Self::draw_lasers(world, canvas, rng);
        }  
    }

    pub fn coin_flip(rng: &mut ChaCha8Rng) -> bool {
        random::rand_range(rng, 0, 2) > 0
    }

    pub fn attack(world: &mut World, index: usize, rng: &mut ChaCha8Rng) {
        if world.world_position == BOSS_ROOMS[0] {
            Self::generate_lasers(world, world.bosses[index].laser_amount, index, rng);
            Self::check_laser_damage(world);
            Self::spawn_enemies(world, rng, index);
        } else if world.world_position == BOSS_ROOMS[1] {
            Self::generate_asteroid(world, index);
            Self::check_asteroid_damage(world);
            Self::generate_column_laser(world, index);
            Self::check_laser_column_damage(world, index);
            Self::spawn_enemies(world, rng, index);
        } else if world.world_position == BOSS_ROOMS[3] {
            Self::chase_player(world, index);
            Self::generate_stun_well(world, index, rng);
            Self::check_stun_well_stun(world);
            Self::spawn_enemies(world, rng, index);
        } else if world.world_position == BOSS_ROOMS[4] {
            Self::generate_safe_spot(world, index, rng);
            Self::check_survive_black_out(world);
            Self::generate_vulnerable_spot(world, index, rng);
            Self::spawn_enemies(world, rng, index);
        } else if world.world_position == BOSS_ROOMS[2] {
            Self::spawn_enemies(world, rng, index);
            Self::generate_asteroid(world, index);
            Self::check_asteroid_damage(world);
            Self::generate_lasers(world, world.bosses[index].laser_amount, index, rng);
            Self::check_laser_damage(world);
            Self::generate_column_laser(world, index);
            Self::check_laser_column_damage(world, index);
            Self::generate_stun_well(world, index, rng);
            Self::check_stun_well_stun(world);
        }  
    }

    pub fn pos_inside_boss(world: &World, pos: Position, world_pos: Position) -> bool {
        for boss_info in &world.bosses {
            if boss_info.world_position == world_pos {
                let boss_pos = boss_info.position;
                let width = boss_info.offset - 1 ;
                return pos.x >= (boss_pos.x - width) && pos.x <= (boss_pos.x + width) &&
                pos.y >= (boss_pos.y - width) && pos.y <= (boss_pos.y + width);
            }
        }
        return false;
    }

    pub fn generate_lasers(world: &mut World, num_lasers: usize, index: usize, rng: &mut ChaCha8Rng) {
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

        for _ in 0..num_lasers {
            let coord: Position = if Boss::coin_flip(rng) {
                Position::new(0, random::rand_range(rng, 0, BOARD_SIZE.1) as usize)
            } else {
                Position::new(random::rand_range(rng, 0, BOARD_SIZE.0) as usize, 0)
            };
            world
                .boss_lasers
                .push((coord, tile::BOSS_LASER_STAGE_1, LASER_LINGER_VALUE));
        }
    }

    pub fn draw_lasers(world: &mut World, canvas: &mut Canvas, rng: &mut ChaCha8Rng) {
         for lasers in &mut world.boss_lasers {
            let mut special_case = true;
            if lasers.0 == Position::new(0,0) {
                if Boss::coin_flip(rng) {
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
    }
    
    pub fn check_laser_damage(world: &mut World) {
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
    }
    
    pub fn draw_asteroids(world: &mut World, index: usize, canvas: &mut Canvas) {
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
    }

    pub fn generate_asteroid(world: &mut World, index: usize) {
        let asteroids = &mut world.boss_asteroids;
        let cooldown = world.bosses[index].asteroid_cooldown;
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
        
        if cooldown == 0 {
            world.boss_asteroids.push((
                world.player.pos,
                tile::BOSS_ASTEROID_STAGE_1,
                ASTEROID_LINGER_VALUE,
            ));
            world.bosses[index].asteroid_cooldown = ASTEROID_COOLDOWN;
        } else {
            world.bosses[index].asteroid_cooldown -= 1;
        }
    }

    pub fn check_asteroid_damage(world: &mut World) {
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
    
    pub fn draw_laser_column(world: &mut World, index: usize, canvas: &mut Canvas) {
        let boss_pos = world.bosses[index].position;
        let mut laser_width: i32 = -2;
        if world.bosses[index].is_major {
            laser_width = -3;
        }
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
                                x * TILE_SIZE.0 as i32,
                                (y + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
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
        let time_vulnerable = world.bosses[index].vulnerable_time;
        let offset = world.bosses[index].offset;
        let mut boss_pos = world.bosses[index].position;
        let can_attack = &mut world.bosses[index].boss_can_attack;
        if let Some(curr_pos) = &mut world.boss_column_laser {
            let pos = curr_pos.0;
            match curr_pos.1 {
                Direction::North => {
                    if boss_pos.y - offset == 0 {
                        world.boss_column_laser = None;
                    } else {
                        boss_pos.y -= 1;
                        curr_pos.0.y = pos.y - 1;
                        Self::move_boss(world, index, boss_pos, Direction::North);
                    }
                }

                Direction::South => {
                    if boss_pos.y + offset == WORLD_SIZE.1 as usize - 1 {
                        world.boss_column_laser = None;
                    } else {
                        boss_pos.y += 1;
                        curr_pos.0.y = pos.y + 1;
                        Self::move_boss(world, index, boss_pos, Direction::South);
                    }
                }

                Direction::East => {
                    if boss_pos.x + offset == WORLD_SIZE.0 as usize - 1 {
                        world.boss_column_laser = None;
                    } else {
                        boss_pos.x += 1;
                        curr_pos.0.x = pos.x + 1;
                        Self::move_boss(world, index, boss_pos, Direction::East);
                    }
                }

                Direction::West => {
                    if boss_pos.x - offset == 0 {
                        world.boss_column_laser = None;
                    } else {
                        boss_pos.x -= 1;
                        curr_pos.0.x = pos.x - 1;
                        Self::move_boss(world, index, boss_pos, Direction::West);
                    }
                }
            }
        } else if *can_attack && time_vulnerable == 0 {
            let boss_delta = (
                boss_pos.x as i32 - world.player.pos.x as i32,
                boss_pos.y as i32 - world.player.pos.y as i32,
            );
            let mut new_position: (Position, Direction);
            if boss_delta.0.abs() > boss_delta.1.abs() {
                if boss_delta.0 > 0 {
                    new_position = (
                        Position::new(boss_pos.x, boss_pos.y - offset),
                        Direction::West,
                    );
                } else {
                    new_position = (
                        Position::new(boss_pos.x, boss_pos.y - offset),
                        Direction::East,
                    );
                }

                if boss_delta.1 < 0 {
                    new_position.0.y = boss_pos.y + offset;
                }
            } else {
                if boss_delta.1 < 0 {
                    new_position = (
                        Position::new(boss_pos.x - offset, boss_pos.y),
                        Direction::South,
                    );
                } else {
                    new_position = (
                        Position::new(boss_pos.x - offset, boss_pos.y),
                        Direction::North,
                    );
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

    pub fn check_laser_column_damage(world: &mut World, index: usize) {
        if let Some(laser) = world.boss_column_laser {
            let boss_pos = world.bosses[index].position;
            let mut player_in_laser: bool = false;
            let mut laser_width: usize = 2;
            let player_pos = world.player.pos;
            if world.bosses[index].is_major {
                laser_width = 3;
            }
            if laser.0.x > boss_pos.x {
                player_in_laser = (player_pos.y >= laser.0.y - laser_width && player_pos.y <= laser.0.y + laser_width
                && player_pos.x >= laser.0.x);
            } else if laser.0.x < boss_pos.x {
                player_in_laser = (player_pos.y >= laser.0.y - laser_width && player_pos.y <= laser.0.y + laser_width
                && player_pos.x <= laser.0.x);
            } else if laser.0.y > boss_pos.y {
                player_in_laser = (player_pos.x >= laser.0.x - laser_width && player_pos.x <= laser.0.x + laser_width
                && player_pos.y >= laser.0.y);

            } else if laser.0.y < boss_pos.y {
                player_in_laser = (player_pos.x >= laser.0.x - laser_width && player_pos.x <= laser.0.x + laser_width
                && player_pos.y <= laser.0.y);
            }

            if player_in_laser {
                world.player.damage(COLUMN_LASER_DAMAGE);
            }
        }
    }
    
    pub fn return_boss_to_center(world: &mut World, index: usize) {
        let mut boss_pos = world.bosses[index].position;
        if boss_pos.x < WORLD_SIZE.0 as usize / 2 {
            boss_pos.x += 1;
            Self::move_boss(world, index, boss_pos, Direction::East);
        } else if boss_pos.x > WORLD_SIZE.0 as usize / 2 {
            boss_pos.x -= 1;
            Self::move_boss(world, index, boss_pos, Direction::West);
        } else if boss_pos.y > WORLD_SIZE.1 as usize / 2 {
            boss_pos.y -= 1;
            Self::move_boss(world, index, boss_pos, Direction::North);
        } else if boss_pos.y < WORLD_SIZE.1 as usize / 2 {
            boss_pos.y += 1;
            Self::move_boss(world, index, boss_pos, Direction::South);
        } else {
            if world.bosses[index].boss_can_attack && world.bosses[index].vulnerable_time == 0 {
                world.bosses[index].boss_can_attack = true;
            } else {
                if !world.bosses[index].boss_can_attack {
                    world.bosses[index].boss_can_attack = true;
                    world.bosses[index].vulnerable_time = VULNERABLE_TIME_BASE;
                }
                world.bosses[index].vulnerable_time -= 1;
            }
        }
    }

    pub fn generate_stun_well(world: &mut World, index: usize, rng: &mut ChaCha8Rng) {
        if world.bosses[index].stun_well_cooldown == 0 {
            world.bosses[index].stun_well_cooldown = STUN_WELL_COOLDOWN;
            let mut well_size = 2;
            if Self::coin_flip(rng) {
                well_size = 3;
            }
            let mut pos = world.bosses[index].position;
            while Self::pos_inside_boss(world, Position::new(pos.x + well_size, pos.y), world.world_position) ||
                Self::pos_inside_boss(world, Position::new(pos.x - well_size, pos.y), world.world_position) ||
                Self::pos_inside_boss(world, Position::new(pos.x, pos.y + well_size), world.world_position) ||
                Self::pos_inside_boss(world, Position::new(pos.x, pos.y - well_size), world.world_position) {
                let x = random::rand_range(rng, 5, WORLD_SIZE.0) as usize;
                let y = random::rand_range(rng, 5, WORLD_SIZE.1) as usize;
                pos = Position::new(x,y);
            }
            world.stun_wells.push((
                pos,
                tile::STUN_WELL_INDICATOR,
                well_size,
                STUN_WELL_LINGER_VALUE,
                false,
            ));
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
            let len = well.2 as i32;
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

    pub fn check_stun_well_stun(world: &mut World) {
        let pos = world.player.pos;
        for index in 0..world.stun_wells.len() {
            let size = world.stun_wells[index].2;
            let well_pos = world.stun_wells[index].0;
            if pos.x >= (well_pos.x - size) && pos.x <= (well_pos.x + size) &&
            pos.y >= (well_pos.y - size) && pos.y <= (well_pos.y + size) {
                if !world.stun_wells[index].4 {
                    world.player.stun_timer = STUN_WELL_STUN_TIME;
                    world.stun_wells[index].4 = true;
                }
            } else {
                world.stun_wells[index].4 = false;
            }
        }
    }
    
    pub fn draw_safe_spot(world: &mut World, canvas: &mut Canvas) {
        if let Some(spot) = world.boss_safe_spot {
            let size = spot.1 as i32;
            let pos = spot.0;
            let text_spot = Vec2::new((pos.x as f32 + 0.25) * TILE_SIZE.0 as f32,  (pos.y as f32 + UNIVERSAL_OFFSET as f32) * TILE_SIZE.1 as f32);
            // (pos.y as f32 + UNIVERSAL_OFFSET as f32) * TILE_SIZE.1 as f32);
            // bevy::math::Vec2::new((pos.x as f32 + 0.25) * TILE_SIZE.0 as f32, 
            // (pos.y as f32 + UNIVERSAL_OFFSET as f32) * TILE_SIZE.1 as f32);
            let duration_left = format!("{}", spot.2);

            if world.in_blackout {
                for i in 0..WORLD_SIZE.0 {
                    for j in 0..WORLD_SIZE.1 {
                        canvas.draw(
                            &graphics::Quad,
                            graphics::DrawParam::new()
                                .dest_rect(graphics::Rect::new_i32(
                                    i as i32 * TILE_SIZE.0 as i32,
                                    (j as i32 + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                    TILE_SIZE.0 as i32,
                                    TILE_SIZE.1 as i32,
                                ))
                                .color(graphics::Color::BLACK),
                        )
                    }
                }
            }

            for i in -size..=size {
                for j in -size..=size {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                (pos.x as i32 + i) * TILE_SIZE.0 as i32,
                                ((pos.y as i32 + j) + UNIVERSAL_OFFSET as i32) * TILE_SIZE.1 as i32,
                                TILE_SIZE.0 as i32,
                                TILE_SIZE.1 as i32,
                            ))
                            .color(tile::SAFE_SPOT_INDICATOR),
                    )
                }
            }
            if !world.in_blackout {
                canvas.draw(
                    &graphics::Text::new(duration_left),
                    graphics::DrawParam::from(text_spot).color(graphics::Color::WHITE),
                );
            }
        }
    }
    
    pub fn generate_safe_spot(world: &mut World, index: usize, rng: &mut ChaCha8Rng) {
        if let Some(spot) = &mut world.boss_safe_spot {
            if spot.2 == 0 {
                world.boss_safe_spot = None;
                world.bosses[index].safe_spot_cooldown = SAFE_SPOT_ATTACK_COOLDOWN;
                world.in_blackout = false;
            } else {
                spot.2 -= 1;
                if spot.2 == 0 {
                    world.in_blackout = true;
                }
            }
        } else if world.bosses[index].safe_spot_cooldown == 0 {
            let mut size: usize = 1; //3x3
            if Self::coin_flip(rng) {
                size = 2; //5x5
            }
            let player_x = world.player.pos.x as i16;
            let player_y = world.player.pos.y as i16;

            let mut pos = world.bosses[index].position;
            while Self::pos_inside_boss(world, Position::new(pos.x + size, pos.y), world.world_position) ||
                Self::pos_inside_boss(world, Position::new(pos.x - size, pos.y), world.world_position) ||
                Self::pos_inside_boss(world, Position::new(pos.x, pos.y + size), world.world_position) ||
                Self::pos_inside_boss(world, Position::new(pos.x, pos.y - size), world.world_position){
                let x = random::rand_range(rng, max(0, player_x - 5), player_x + 5) as usize;
                let y = random::rand_range(rng, max(0, player_y - 5), player_y + 5) as usize;
                pos = Position::new(x,y);
            }
        
            world.boss_safe_spot = Some((pos, size, SAFE_SPOT_TIME));
        } else {
            world.bosses[index].safe_spot_cooldown -= 1;
        }


    }

    pub fn check_survive_black_out(world: &mut World) {
        if world.in_blackout {
            if let Some(spot) = world.boss_safe_spot {
                let spot_pos = spot.0;
                let pos = world.player.pos;
                let size = spot.1;
                if !(pos.x >= (spot_pos.x - size) && pos.x <= (spot_pos.x + size) &&
                pos.y >= (spot_pos.y - size) && pos.y <= (spot_pos.y + size)) {
                    world.player.damage(MAX_PLAYER_HEALTH);
                }
            }
        }
    }
    
    pub fn generate_vulnerable_spot(world: &mut World, index: usize, rng: &mut ChaCha8Rng) {
        if let Some(spot) = world.boss_vulnerable_spot {
            return;
        } else {
            if world.bosses[index].vulnerable_time == 0 {
                let offset = world.bosses[index].offset - 1;
                let on_vert = Self::coin_flip(rng); // On horizontal?
                let pos_side = Self::coin_flip(rng); //On top or left?
                let spot = random::rand_range(rng, 0, offset as i16 * 2 - 1) as usize;
                let mut vulnerable_spot = Position::new(0,0);
                if on_vert {
                    vulnerable_spot.y = world.bosses[index].position.y - offset + spot;
                    if pos_side {
                        vulnerable_spot.x = world.bosses[index].position.x - offset;
                    } else {
                        vulnerable_spot.x = world.bosses[index].position.x + offset;

                    }
                } else {
                    vulnerable_spot.x = world.bosses[index].position.x - offset + spot;
                    if pos_side {
                        vulnerable_spot.y = world.bosses[index].position.y - offset;
                    } else {
                        vulnerable_spot.y = world.bosses[index].position.y + offset;
                    }
                }
                world.boss_vulnerable_spot = Some(vulnerable_spot);

                if world.bosses[index].shield_health == 0 {
                    world.bosses[index].vulnerable_time = VULNERABLE_TIME_BASE;
                    world.bosses[index].shield_health = SHIELD_HITS_NEEDED;
                    world.boss_vulnerable_spot = None
                } 
                
            } else {
                world.bosses[index].vulnerable_time -= 1;
            }
        }
    }
    
    pub fn chase_player(world: &mut World, index: usize) {
        let boss_pos = world.bosses[index].position;
        let boss_delta = (
            boss_pos.x as i32 - world.player.pos.x as i32,
            boss_pos.y as i32 - world.player.pos.y as i32,
        );
        if (world.bosses[index].rush_info.0 || (boss_delta.0.abs() <= 3 || boss_delta.1.abs() <= 3))
            && world.bosses[index].chase_rush_cooldown == 0
        {
            Self::rush_player(world, index, boss_delta.0, boss_delta.1);
        } else if world.bosses[index].boss_can_attack && world.bosses[index].speed_delay == 0 {
            let mut boss_pos = world.bosses[index].position;
            if boss_delta.0.abs() > boss_delta.1.abs() {
                if boss_delta.0 < 0 {
                    boss_pos.x += 1;
                    Self::move_boss(world, index, boss_pos, Direction::East);
                } else {
                    boss_pos.x -= 1;
                    Self::move_boss(world, index, boss_pos, Direction::West);
                }
            } else {
                if boss_delta.1 < 0 {
                    boss_pos.y += 1;
                    Self::move_boss(world, index, boss_pos, Direction::South);
                } else {
                    boss_pos.y -= 1;
                    Self::move_boss(world, index, boss_pos, Direction::North);
                }
            }
            world.bosses[index].speed_delay = BOSS_3_MOVE_DELAY;
            if (world.bosses[index].chase_rush_cooldown != 0) {
                world.bosses[index].chase_rush_cooldown -= 1;
            }
        } else {
            if world.bosses[index].vulnerable_time != 0 {
                world.bosses[index].vulnerable_time -= 1;
                if (world.bosses[index].vulnerable_time == 0) {
                    world.bosses[index].boss_can_attack = true;
                }
            } else {
                world.bosses[index].speed_delay -= 1;
                if (world.bosses[index].chase_rush_cooldown != 0) {
                    world.bosses[index].chase_rush_cooldown -= 1;
                }
            }
        }
    }

    pub fn rush_player(world: &mut World, index: usize, x_delta: i32, y_delta: i32) {
        if !world.bosses[index].rush_info.0 {
            if x_delta.abs() > y_delta.abs() {
                if x_delta < 0 {
                    world.bosses[index].rush_info.1 = Some(Direction::East);
                    world.bosses[index].rush_info.3 =
                        WORLD_SIZE.0 as usize - 1 - world.bosses[index].position.x;
                } else {
                    world.bosses[index].rush_info.1 = Some(Direction::West);
                    world.bosses[index].rush_info.3 = world.bosses[index].position.x;
                }
            } else {
                if y_delta < 0 {
                    world.bosses[index].rush_info.1 = Some(Direction::South);
                    world.bosses[index].rush_info.3 =
                        WORLD_SIZE.1 as usize - 1 - world.bosses[index].position.y;
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
                }

                tile::BOSS_LASER_STAGE_2 => {
                    world.bosses[index].rush_info.2 = tile::BOSS_LASER_REAL;
                }

                _ => {
                    if let Some(direction) = world.bosses[index].rush_info.1 {
                        let player_pos = world.player.pos;
                        let boss_pos = world.bosses[index].position;
                        let len = world.bosses[index].offset - 1;
                        match direction {
                            Direction::North => {
                                if player_pos.x <= boss_pos.x + len && player_pos.x >= boss_pos.x - len &&
                                player_pos.y <= boss_pos.y 
                                {
                                    world.player.damage(MAX_PLAYER_HEALTH);
                                } else {
                                    world.bosses[index].position.y = 0 + offset;
                                }
                            }

                            Direction::South => {
                                if player_pos.x <= boss_pos.x + len && player_pos.x >= boss_pos.x - len &&
                                player_pos.y >= boss_pos.y 
                                {
                                    world.player.damage(MAX_PLAYER_HEALTH);
                                } else {
                                    world.bosses[index].position.y = WORLD_SIZE.1 as usize - offset - 1;
                                }
                            }

                            Direction::West => {
                                if player_pos.y <= boss_pos.y + len && player_pos.y >= boss_pos.y - len &&
                                player_pos.x <= boss_pos.x
                                {
                                    world.player.damage(MAX_PLAYER_HEALTH);
                                } else {
                                    world.bosses[index].position.x = 0 + offset;
                                }
                            }

                            Direction::East => {
                                if player_pos.y <= boss_pos.y + len && player_pos.y >= boss_pos.y - len &&
                                player_pos.x >= boss_pos.x
                                {
                                    world.player.damage(MAX_PLAYER_HEALTH);
                                } else {
                                    world.bosses[index].position.x = WORLD_SIZE.0 as usize - 1 - offset;
                                }
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
                        }

                        Direction::South => {
                            x = x + j;
                            if after_rush {
                                y = y - i;
                            } else {
                                y = y + i;
                            }
                        }

                        Direction::West => {
                            if after_rush {
                                x = x + i;
                            } else {
                                x = x - i;
                            }
                            y = y + j;
                        }

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

    pub fn move_boss(world: &mut World, index: usize, new_pos: Position, direction: Direction) {
        world.bosses[index].position = new_pos;
        let world_map = &world.terrain_map[world.world_position.y][world.world_position.x];
        if  Self::pos_inside_boss(world, world.player.pos, world.world_position) {
            world.player.damage(BOSS_COLLISION_DAMAGE);
            match direction {
                Direction::North => {
                    if world_map.contains_key(&Position::new(world.player.pos.x, world.player.pos.y - 1)) {
                        world.player.damage(MAX_PLAYER_HEALTH);
                    } else {
                        World::update_position(world, world.player.pos, 
                            (Position::new(world.player.pos.x, world.player.pos.y - 1), 
                            world.world_position));
                        world.player.pos.y = world.player.pos.y - 1;
                    }
                },
                Direction::South => {
                    if world_map.contains_key(&Position::new(world.player.pos.x, world.player.pos.y + 1)) {
                        world.player.damage(MAX_PLAYER_HEALTH);
                    } else {
                        World::update_position(world, world.player.pos, 
                            (Position::new(world.player.pos.x, world.player.pos.y + 1), 
                            world.world_position));
                        world.player.pos.y = world.player.pos.y + 1;
                    }
                },
                Direction::East => {
                    if world_map.contains_key(&Position::new(world.player.pos.x + 1, world.player.pos.y)) {
                        world.player.damage(MAX_PLAYER_HEALTH);
                    } else {
                        World::update_position(world, world.player.pos, 
                            (Position::new(world.player.pos.x + 1, world.player.pos.y), 
                            world.world_position));
                        world.player.pos.x = world.player.pos.x + 1;
                    }
                },
                Direction::West => {
                    if world_map.contains_key(&Position::new(world.player.pos.x - 1, world.player.pos.y )) {
                        world.player.damage(MAX_PLAYER_HEALTH);
                    } else {
                        World::update_position(world, world.player.pos, 
                            (Position::new(world.player.pos.x - 1, world.player.pos.y), 
                            world.world_position));
                        world.player.pos.x = world.player.pos.x - 1;
                    }
                }
            }
        }
    }
    
    //Returns a tuple (if hit_boss, if actually can do damage)
    pub fn can_hit_boss(world: &mut World, hit_pos: Position, world_pos: Position) -> (bool, bool) { 
        let mut index: usize = 0;
        for i in 0..world.bosses.len() {
            let boss = &mut world.bosses[i];
            if boss.world_position == world_pos {
                index = i;
                break;
            }
        }
        let hit: bool = Self::pos_inside_boss(world, hit_pos, world_pos);
        let can_hit: bool = world.bosses[index].vulnerable_time != 0;
        
        if let Some(spot) = world.boss_vulnerable_spot {
            if hit_pos == spot {
                world.bosses[index].shield_health -= 1;
                world.boss_vulnerable_spot = None;
            }
        }

        return (hit, can_hit);
    }
    
    pub fn damage(world: &mut World, damage: usize, world_pos: Position) {
        for index in 0..world.bosses.len() {
            let boss = &mut world.bosses[index];
            if boss.world_position == world_pos {
                boss.health = max(0, boss.health as i32 - damage as i32) as usize;
            }
        }
    }

    pub fn kill(world: &mut World, index: usize) {
        world.bosses.remove(index);
        // when kill is implemented this should reopen doors
        world.boss_defeated[world.world_position.y][world.world_position.x] = true;
        world.boss_safe_spot = None;
        world.boss_vulnerable_spot = None;
        world.boss_column_laser = None;
        world.boss_lasers.clear();
        world.stun_wells.clear();
        world.boss_asteroids.clear();
        World::toggle_doors(
            &mut world.terrain_map,
            world.world_position,
            world.player.pos,
            world.boss_defeated,
        );
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, serde::Deserialize, serde::Serialize)]

pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}
