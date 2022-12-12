use crate::{
    direction::Direction,
    enemy::{self, Enemy},
    entity::Entity,
    projectile::Projectile,
    tile,
    world::World,
    WORLD_SIZE, BOARD_SIZE, TILE_SIZE,
    utils::Position, UNIVERSAL_OFFSET,
};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::winit::event::VirtualKeyCode;
use ggez::graphics::{self, Canvas};
use std::{
    collections::HashMap,
};

// Can change easily
const MAX_PLAYER_HEALTH: usize = 10;
const PLAYER_MELEE_DAMAGE: usize = 1;
const MELEE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::A;
// TODO look over these values
const HEAL_ABILITY_RETURN: usize = 2;
const HEAL_KEYCODE: VirtualKeyCode = KeyCode::H;
const LIGHTNING_KEYCODE: VirtualKeyCode = KeyCode::L;
const BUILD_KEYCODE: VirtualKeyCode = KeyCode::B;
const PROJECTILE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::Space;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const PLAYER_PROJECTILE_DAMAGE: usize = 1;
const PLAYER_INITIAL_SPEED: usize = 1;
const PLAYER_INITIAL_ENERGY: usize = 5;
const PERMISSIBLE_TILES: [[f32; 4]; 1] = [tile::GRASS];

// This is with the covered tile model, but we could use the static/dynamic board paradighm or
// something else entirely
pub struct Player {
    // This is the position in the form (x, y)
    pub pos: Position,

    // The direction that the player is facing at the moment
    // It isn't needed for movement, and the way I wrote movement is a bit convoluted to allow this
    // attribute to make sense, but when we introduce projectiles, this will be needed to make them
    // shoot in the right direction
    pub direction: Direction,

    // This controls the number of tiles a player moves in a direction in a given keypress
    pub speed: usize,

    // This is the player color. NOTE: both this and the previous attribute assume that the game
    // world is a set of tiles and the player is represented as a solid color
    pub color: [f32; 4],

    // Stores player health: for player death and such
    health: usize,

    // planned energy, used for healing, projectiles, (teleportation?), building
    energy: usize,
    //
}

impl Player {
    pub fn draw_health(&self, canvas: &mut graphics::Canvas) {
        let outline = [(2,0),(3,0),(4,0),(5,0),(7,0),(8,0),(9,0),(10,0),(1,1),(6,1),(11,1),(0,2),(12,2),(0,3),(12,3),(0,4),(12,4),(0,5),(12,5),(0,6),(12,6),(1,7),(11,7),(2,8),(10,8),(3,9),(9,9),(4,10),(8,10),(5,11),(7,11),(6,12)];
        for i in 0..5 {
            for coord in outline {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            ((coord.0) as i32 + 1) * 4 + i*60,
                            ((coord.1) as i32 + 2) * 4,
                            4,
                            4,
                        ))
                        .color([1.0,1.0,1.0,1.0]),
                )
            }
            Self::color_heart(&self, canvas, outline, i);
        }   
    }

    pub fn draw_energy(&self, canvas: &mut graphics::Canvas) {
        let outline = [(3,0),(4,0),(5,0),(6,0),(7,0),(8,0),(9,0),(3,1),(9,1),(2,2),(8,2),(2,3),(7,3),(1,4),(6,4),(1,5),(5,5),(6,5),(7,5),(8,5),(0,6),(8,6),(0,7),(1,7),(2,7),(3,7),(7,7),(3,8),(6,8),(2,9),(5,9),(2,10),(4,10),(1,11),(3,11),(1,12),(2,12)];
        for i in 0..5 {
            for coord in outline {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            ((coord.0) as i32 + 130) * 4 + i*48,
                            ((coord.1) as i32 + 2) * 4,
                            4,
                            4,
                        ))
                        .color([1.0,1.0,1.0,1.0]),
                )
            }
            Self::color_energy(&self, canvas, outline, i);
        }  
    }
 
    pub fn color_heart(&self, canvas: &mut graphics::Canvas, outline: [(usize,usize); 32], iteration: i32) {
        let health_check: i32 = self.health as i32 - (iteration as i32 * 2);
        if(health_check > 0) {
            for i in 8..outline.len()-2 {
                if outline[i].1 == outline[i+1].1 {
                    let mut offset = 1;
                    while outline[i].0+offset != outline[i+1].0 {
                        if (health_check != 1) || outline[i].0+offset <= 6 {
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(graphics::Rect::new_i32(
                                        ((outline[i].0+offset) as i32 + 1) * 4 + iteration*60,
                                        ((outline[i].1) as i32 + 2) * 4,
                                        4,
                                        4,
                                    ))
                                    .color([1.0,0.1,0.1,1.0]),
                            );
                        }   
                        offset += 1;
                    }
                }
            }
        }   
    }

    pub fn color_energy(&self, canvas: &mut graphics::Canvas, outline: [(usize,usize); 37], iteration: i32) {
        for i in 7..outline.len()-1 {
            if outline[i].1 == outline[i+1].1 {
                let mut offset = 1;
                while outline[i].0+offset != outline[i+1].0 {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new_i32(
                                ((outline[i].0+offset) as i32 + 130) * 4 + iteration*48,
                                ((outline[i].1) as i32 + 2) * 4,
                                4,
                                4,
                            ))
                            .color([1.0,0.8,0.0,1.0]),
                    );
                    offset += 1;
                }
                
            }
        }
    }

    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        self.health -= damage;
        dbg!(self.health);
    }

    pub fn new() -> Self {
        let temp = Self {
            pos: Position::new(0,0),
            direction: Direction::South,
            speed: PLAYER_INITIAL_SPEED,
            color: tile::PLAYER,
            health: MAX_PLAYER_HEALTH,
            energy: PLAYER_INITIAL_ENERGY,
        };
        temp
    }

    // eventually this should be the functionality to like shoot projectiles and stuff but for now
    // it just handles like arrow keys
    pub fn use_input(key: KeyInput, world: &mut World) -> bool {
        match key.keycode {
            Some(key_pressed) => match key_pressed {
                KeyCode::Down => {
                    world.player.direction = Direction::South;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Up => {
                    world.player.direction = Direction::North;
                     World::travel(world, Entity::Player);
                }
                KeyCode::Left => {
                    world.player.direction = Direction::West;
                    World::travel(world, Entity::Player);
                }
                KeyCode::Right => {
                    world.player.direction = Direction::East;
                    World::travel(world, Entity::Player);
                }

                // Arbitrarily chosen for attack, can change later
                MELEE_ATTACK_KEYCODE => {
                    Player::melee_attack(world);
                }
                PROJECTILE_ATTACK_KEYCODE => {
                    if (world.player.energy > 0) {
                        Player::projectile_attack(world);
                        // world.player.energy -= 1;
                        // commented out so I can test everything
                    }
                }
                HEAL_KEYCODE => {
                    if world.player.energy >= 5 {
                        world.player.health += HEAL_ABILITY_RETURN;
                        world.player.energy -= 5;
                    }
                }
                BUILD_KEYCODE => {
                    Player::build(world);
                }
                _ => {return false;}
            },
            None => {return false;}
        }
        return true;
    }

    pub fn build(world: &mut World) {
        let position = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            1,
        );

        // make sure there are no enemies       
        if !world.entity_positions.contains_key(&position) {
            // check if there is terrain at the position 
            // If there is nothing, then build there
            // If there is something, check if it's a build, and destroy it
            match world.terrain_positions.get(&position) {
                Some(color) => {
                    if *color == tile::STRUCTURE {
                        world.terrain_positions.remove(&position);
                    }
                } 
                None => {
                    world.terrain_positions.insert(position, tile::STRUCTURE);
                }
            }
        }
    }

    pub fn melee_attack(world: &mut World) {
        // gets the position that the attack will be applied to, one tile forward of the player in
        // the direction that they are facing
        let attacking_position = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            world.player.speed,
        );

        // We do not know what enemies are on the tile being attacked, so we need to go through the
        // enemies and check if any of them are on the attacking tile, then damage them
        for enemy in &mut world.enemies {
            if enemy.pos == attacking_position {
                enemy.damage(PLAYER_MELEE_DAMAGE);
            }
        }
    }

    // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // and such should be determined by the projectile object itself
    pub fn projectile_attack(world: &mut World) {
        let projectile_spawn_pos = World::new_position(
            world.player.pos,
            world.player.direction.clone(),
            world,
            world.player.speed,
        );
        if projectile_spawn_pos != world.player.pos {
            let projectile = Projectile::new(
                projectile_spawn_pos.x,
                projectile_spawn_pos.y,
                PLAYER_PROJECTILE_SPEED,
                PLAYER_PROJECTILE_DAMAGE,
                world.player.direction.clone(),
            );
            world.entity_positions.insert(projectile.pos, (tile::PROJECTILE, Entity::Projectile(world.projectiles.len())));
            world.projectiles.push(projectile);
        }
    }

    pub fn can_travel_to(
        position: Position,
        entity_positions: &HashMap<Position, ([f32; 4], Entity)>,
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

