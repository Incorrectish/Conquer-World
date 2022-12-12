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
const MAX_PLAYER_HEALTH: usize = 30;
const PLAYER_MELEE_DAMAGE: usize = 1;
const TELEPORTATION_COST: usize = 10;
const HEAL_COST: usize = 5;
const MELEE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::A;
// TODO look over these values
const HEAL_ABILITY_RETURN: usize = 2;
const HEAL_KEYCODE: VirtualKeyCode = KeyCode::H;
const TELEPORT_KEYCODE: VirtualKeyCode = KeyCode::T;
const LIGHTNING_KEYCODE: VirtualKeyCode = KeyCode::L;
const BUILD_KEYCODE: VirtualKeyCode = KeyCode::B;
const PROJECTILE_ATTACK_KEYCODE: VirtualKeyCode = KeyCode::Space;
const PLAYER_PROJECTILE_SPEED: usize = 1;
const PLAYER_PROJECTILE_DAMAGE: usize = 1;
const PLAYER_INITIAL_SPEED: usize = 1;
const PLAYER_INITIAL_ENERGY: usize = 30;
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

    // This is the position queued by mouse clicks, used for teleportation, etc
    pub queued_position: Option<Position>,
}

impl Player {
    pub fn health(&self) -> usize {
        self.health
    }

    pub fn damage(&mut self, damage: usize) {
        self.health -= damage;
    }

    pub fn new() -> Self {
        let temp = Self {
            pos: Position::new(0,0),
            direction: Direction::South,
            speed: PLAYER_INITIAL_SPEED,
            color: tile::PLAYER,
            health: MAX_PLAYER_HEALTH,
            energy: PLAYER_INITIAL_ENERGY,
            queued_position: None
        };
        temp
    }

    //Draws hearts on open space above the screen
    pub fn draw_health(&self, canvas: &mut graphics::Canvas) {
        let outline = [(2,0),(3,0),(4,0),(5,0),(7,0),(8,0),(9,0),(10,0),(1,1),(6,1),(11,1),(0,2),(12,2),(0,3),(12,3),(0,4),(12,4),(0,5),(12,5),(0,6),(12,6),(1,7),(11,7),(2,8),(10,8),(3,9),(9,9),(4,10),(8,10),(5,11),(7,11),(6,12)]; //Manually input coordinates of the outline of the heart
        for i in 0..5 { //Draw one heart each time in the loop
            for coord in outline {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            ((coord.0) as i32 + 1) * 5 + i*70, //x coordinate of each outline pixel from array
                            ((coord.1) as i32 + 2) * 5, //y coordinate of each outline pixel from array
                            5,
                            5,
                        ))
                        .color([1.0,1.0,1.0,1.0]), //Color of outline
                )
            }
            Self::color_heart(&self, canvas, outline, i); //Color in the heart
        }   
    }

    //Draws energy symbols on space above screen, works exactly the same as draw_health() except has different outline positions
    pub fn draw_energy(&self, canvas: &mut graphics::Canvas) {
        let outline = [(3,0),(4,0),(5,0),(6,0),(7,0),(8,0),(9,0),(3,1),(9,1),(2,2),(8,2),(2,3),(7,3),(1,4),(6,4),(1,5),(5,5),(6,5),(7,5),(8,5),(0,6),(8,6),(0,7),(1,7),(2,7),(3,7),(7,7),(3,8),(6,8),(2,9),(5,9),(2,10),(4,10),(1,11),(3,11),(1,12),(2,12)];
        for i in 0..5 {
            for coord in outline {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            ((coord.0) as i32 + 80) * 5 + i*53,
                            ((coord.1) as i32 + 2) * 5,
                            5,
                            5,
                        ))
                        .color([1.0,1.0,1.0,1.0]),
                )
            }
            Self::color_energy(&self, canvas, outline, i);
        }  
    }

    //Colors in the hearts based on current health
    pub fn color_heart(&self, canvas: &mut graphics::Canvas, outline: [(usize,usize); 32], iteration: i32) {
        let master_heart_color: [f32; 4]; //True value for specific heart, used so half hearts can be colored correctly
        let stage3 = [0.2, 0.8, 0.2, 1.0];
        let stage2 = [1.0, 0.8, 0.1, 1.0];
        let stage1 = [1.0, 0.1, 0.1, 1.0];
        let mut health_check: i32 = self.health as i32 - (iteration as i32 * 2); //Checks if you have half, full, or no healthpoints on specific heart
        if health_check > 20 {
            master_heart_color = stage3;
            health_check -= 20;
        } else if health_check > 10 {
            master_heart_color = stage2;
            health_check -= 10;
        } else {
            master_heart_color = stage1;
        }
        if health_check > 0 {
            for i in 8..outline.len()-2 { //Skip first row of outline (first 8 pixels)
                if outline[i].1 == outline[i+1].1 { //while the outline pixel and next outline pixel are on the same y axis
                    //Color the pixels inbetween each outline position (fill in the heart)
                    let mut offset = 1;
                    let mut temp_heart_color = master_heart_color; //Temp color incase it switches due to half heart
                    while outline[i].0+offset != outline[i+1].0 {
                        let pos = (outline[i].0+offset, outline[i].1); //Get the position going to be colored (saves space)
                        if pos == (2,2) || pos == (3,2) || pos == (2,3) { //For the three white pixels :)
                            temp_heart_color = [1.0, 1.0, 1.0, 1.0];
                        }
                        //If it is only half a heart, only color in half (stop at x position 6)
                        //However, if the color isn't red, color in the other half the color one stage down
                        if health_check != 1 || (outline[i].0+offset <= 6 || master_heart_color != stage1) {
                            if health_check == 1 && outline[i].0+offset > 6 {
                                if master_heart_color == stage3 {
                                    temp_heart_color = stage2;
                                } else if master_heart_color == stage2 {
                                    temp_heart_color = stage1;
                                }
                            }
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(graphics::Rect::new_i32(
                                        ((pos.0) as i32 + 1) * 5 + iteration*70,
                                        ((pos.1) as i32 + 2) * 5,
                                        5,
                                        5,
                                    ))
                                    .color(temp_heart_color),
                            ); 
                            temp_heart_color = master_heart_color;
                        } 
                        offset += 1;
                    }
                }
            }
        }   
    }

    //Colors in the energies based on current energy
    //Works exactly the same as color_heart(), but instead the half energy uses half the height, not the width
    pub fn color_energy(&self, canvas: &mut graphics::Canvas, outline: [(usize,usize); 37], iteration: i32) {
        let master_energy_color: [f32; 4]; 
        let stage3 = [0.15, 0.2, 0.85, 1.0];
        let stage2 = [0.4, 0.45, 0.8, 1.0];
        let stage1 = [0.0,0.6,0.98,1.0];
        let mut energy_check: i32 = self.energy as i32 - (iteration as i32 * 2);

        if energy_check > 20 {
            master_energy_color = stage3;
            energy_check -= 20;
        } else if energy_check > 10 {
            master_energy_color = stage2;
            energy_check -= 10;
        } else {
            master_energy_color = stage1;
        }
        if energy_check > 0 {
            for i in 7..outline.len()-1 { 
                if outline[i].1 == outline[i+1].1 { 
                    let mut offset = 1;
                    let mut temp_energy_color = master_energy_color; 
                    if (energy_check != 1) || (outline[i+1].1 >= 6 || master_energy_color != stage1) {
                        if energy_check == 1 && outline[i].1 < 6 {
                            if master_energy_color == stage3 {
                                temp_energy_color = stage2;
                            } else if master_energy_color == stage2 {
                                temp_energy_color = stage1;
                            }
                        }
                        while outline[i].0+offset != outline[i+1].0 {
                            let pos = (outline[i].0+offset, outline[i].1);
                            canvas.draw(
                                &graphics::Quad,
                                graphics::DrawParam::new()
                                    .dest_rect(graphics::Rect::new_i32(
                                        ((pos.0) as i32 + 80) * 5 + iteration*53,
                                        ((pos.1) as i32 + 2) * 5,
                                        5,
                                        5,
                                    ))
                                    .color(temp_energy_color),
                            ); 
                            offset += 1;
                        }
                    }
                }
            }
        }    
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
                // MELEE_ATTACK_KEYCODE => {
                //     Player::melee_attack(world);
                // }
                // PROJECTILE_ATTACK_KEYCODE => {
                //     if (world.player.energy > 0) {
                //         Player::projectile_attack(world);
                //         world.player.energy -= 1;
                //         // commented out so I can test everything
                //     }
                // }
                HEAL_KEYCODE => {
                    if world.player.energy >= HEAL_COST {
                        world.player.health += HEAL_ABILITY_RETURN;
                        world.player.energy -= HEAL_COST;
                    }
                }
                // BUILD_KEYCODE => {
                //     Player::build(world);
                // }
                LIGHTNING_KEYCODE => {
                    Player::lightning(world);
                }

                TELEPORT_KEYCODE => {
                    if world.player.energy >= TELEPORTATION_COST && world.player.queued_position.is_some() {
                        Self::teleport(world);
                        world.player.energy -= TELEPORTATION_COST;
                    }
                } 
                _ => {return false;}
            },
            None => {return false;}
        }
        return true;
    }

    pub fn lightning(world: &mut World) {
        // if let Some(queued_position) = world.player.queued_position {
        //     // TODO: Damage
        //     let projectile = Projectile::new(
        //         queued_position.x,
        //         queued_position.y,
        //         0,
        //         3,
        //         Direction::North,
        //         tile::LIGHTNING_PLACEHOLDER,
        //     );
        //     world.projectiles.push(projectile)
        // }
    }

    // THIS METHOD EXPECTS A QUEUED POSITION
    pub fn teleport(world: &mut World) {
        if let Some(pos) = world.player.queued_position {
            world.entity_positions.remove(&world.player.pos);
            if !world.terrain_positions.contains_key(&pos) && !world.entity_positions.contains_key(&pos) {
                world.entity_positions.insert(pos, (tile::PLAYER, Entity::Player));
                world.player.pos = pos;
            } else {
                world.entity_positions.insert(world.player.pos, (tile::PLAYER, Entity::Player));
            }
        }
    }

    // pub fn build(world: &mut World) {
    //     let position = World::new_position(
    //         world.player.pos,
    //         world.player.direction.clone(),
    //         world,
    //         1,
    //     );

    //     // make sure there are no enemies       
    //     if !world.entity_positions.contains_key(&position) {
    //         // check if there is terrain at the position 
    //         // If there is nothing, then build there
    //         // If there is something, check if it's a build, and destroy it
    //         match world.terrain_positions.get(&position) {
    //             Some(color) => {
    //                 if *color == tile::STRUCTURE {
    //                     world.terrain_positions.remove(&position);
    //                 }
    //             } 
    //             None => {
    //                 world.terrain_positions.insert(position, tile::STRUCTURE);
    //             }
    //         }
    //     }
    // }

    // pub fn melee_attack(world: &mut World) {
    //     // gets the position that the attack will be applied to, one tile forward of the player in
    //     // the direction that they are facing
    //     let attacking_position = World::new_position(
    //         world.player.pos,
    //         world.player.direction.clone(),
    //         world,
    //         world.player.speed,
    //     );

    //     // We do not know what enemies are on the tile being attacked, so we need to go through the
    //     // enemies and check if any of them are on the attacking tile, then damage them
    //     for enemy in &mut world.enemies {
    //         if enemy.pos == attacking_position {
    //             enemy.damage(PLAYER_MELEE_DAMAGE);
    //         }
    //     }
    // }

    // // This function should just spawn a projectile, the mechanics of dealing with the projectile
    // // and such should be determined by the projectile object itself
    // pub fn projectile_attack(world: &mut World) {
    //     let projectile_spawn_pos = World::new_position(
    //         world.player.pos,
    //         world.player.direction.clone(),
    //         world,
    //         world.player.speed,
    //     );
    //     if projectile_spawn_pos != world.player.pos {
    //         let projectile = Projectile::new(
    //             projectile_spawn_pos.x,
    //             projectile_spawn_pos.y,
    //             PLAYER_PROJECTILE_SPEED,
    //             PLAYER_PROJECTILE_DAMAGE,
    //             world.player.direction.clone(),
    //             tile::PROJECTILE_PLAYER,
    //         );
    //         for index in 0..world.enemies.len()  { //Check if it's spawning on enemy, if so damage the enenmy and not spawn a projectile
    //             if projectile_spawn_pos == world.enemies[index].pos {
    //                 world.enemies[index].damage(projectile.damage);
    //                 return;
    //             }
    //         }
    //         world.entity_positions.insert(projectile.pos, (tile::PROJECTILE_PLAYER, Entity::Projectile(world.projectiles.len())));
    //         world.projectiles.push(projectile);
    //     }
    // }

    pub fn can_travel_to(
        world: &mut World,
        position_info: (Position, Position) //Where .0 is the position, and .1 is the world_position
    ) -> bool {
        //Get the map on which the position is on
        let terrain_map = &world.terrain_map;
        let entity_map = &world.entity_map;
        let curr_terrain_map = &terrain_map[position_info.1.y][position_info.1.x];
        let curr_entity_map = &entity_map[position_info.1.y][position_info.1.x];
        if curr_entity_map.contains_key(&position_info.0) || curr_terrain_map.contains_key(&position_info.0) {
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

