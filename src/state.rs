use crate::direction::Direction;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::utils::Boss;
use crate::utils::Position;
use crate::UNIVERSAL_OFFSET;
use ggez::audio;
use ggez::audio::SoundSource;
use rand_chacha::ChaChaRng;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};

use crate::{
    entity::Entity,
    projectile::Projectile,
    tile,
    world::{World, BOSS_ROOMS, FINAL_BOSS_ROOM},
    BOARD_SIZE, SCREEN_SIZE, TILE_SIZE, WORLD_SIZE,
};

use rand::prelude::*;
use rand::rngs::ThreadRng;
use rand_chacha::ChaCha8Rng;

use ggez::{
    event,
    graphics::{self, Canvas},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

use ::std::collections::HashMap;

pub const SOUND_PATH: &'static str = "/overworld.ogg";

pub const RNG_SEED: u64 = 0;

// const MOVES_TILL_ENERGY_REGEN: usize = 5;

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    should_draw: bool,
    command: bool,
    sound: audio::Source,
    is_playing: bool,
    // Abstraction for the world and what is contained within it
    world: Option<World>,
    title_screen: bool,
    pub rng: Option<ChaCha8Rng>,
}

impl State {
    // just returns the default values
    pub fn new(ctx: &mut Context, title_screen: bool) -> GameResult<State> {
        let sound = audio::Source::new(ctx, SOUND_PATH)?;
        let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
        let temp = State {
            should_draw: true,
            command: false,
            sound,
            is_playing: false,
            world: Some(World::new(&mut rng)),
            title_screen,
            rng: Some(rng),
        };
        Ok(temp)
    }

    pub fn title_screen(ctx: &mut Context) -> GameResult<State> {
        // TODO make this the title screen music
        let sound = audio::Source::new(ctx, SOUND_PATH)?;
        Ok(State {
            should_draw: true,
            command: false,
            sound,
            is_playing: false,
            world: None,
            title_screen: true,
            rng: None,
        })
    }

    pub fn from(world: World, ctx: &mut Context, rng: ChaCha8Rng) -> GameResult<State> {
        let sound = audio::Source::new(ctx, SOUND_PATH)?;
        let temp = State {
            should_draw: true,
            command: false,
            sound,
            is_playing: false,
            world: Some(world),
            title_screen: false,
            rng: Some(rng),
        };
        Ok(temp)
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.is_playing {
            let _ = self.sound.play_detached(ctx);
            self.is_playing = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.title_screen {
            let canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::TITLE_SCREEN_FLOOR));
            canvas.finish(ctx)?;
        } else {
            if self.should_draw {
                let world = self.world.as_mut().unwrap();
                let rng = self.rng.as_mut().unwrap();
                let mut boss_room = false;
                let mut final_boss = false;
                for boss_room_position in BOSS_ROOMS {
                    if world.world_position == boss_room_position {
                        boss_room = true;
                    }
                }
                if world.world_position == FINAL_BOSS_ROOM {
                    final_boss = true;
                }
                let mut canvas = if final_boss {
                    graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::BOSS_FLOOR))
                } else if boss_room {
                    graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::FLOOR))
                } else {
                    graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::GRASS))
                    // for x in 0..WORLD_SIZE.0 {
                    //     for y in 0..WORLD_SIZE.1 {
                    //         let mut rng = rand::thread_rng();
                    //         canvas.draw(
                    //             &graphics::Quad,
                    //             graphics::DrawParam::new()
                    //                 .dest_rect(graphics::Rect::new_i32(
                    //                     (x as usize * TILE_SIZE.0 as usize) as i32,
                    //                     ((y as usize + UNIVERSAL_OFFSET as usize) as i32) * TILE_SIZE.1 as i32,
                    //                     // (loc.x - (self.world_position.x * WORLD_SIZE.0 as usize)) as i32
                    //                     //     * TILE_SIZE.0 as i32,
                    //                     // (loc.y - (self.world_position.y * WORLD_SIZE.1 as usize)
                    //                     //     + UNIVERSAL_OFFSET as usize) as i32
                    //                     //     * TILE_SIZE.1 as i32,

                    //                     TILE_SIZE.0 as i32,
                    //                     TILE_SIZE.1 as i32,
                    //                 ))
                    //                 .color(World::related_color(&mut rng, tile::GRASS)),
                    //         )
                    //     }
                    // }
                };
                world.draw(&mut canvas, rng);

                //For Text
                // let level_dest = bevy::math::Vec2::new(10.0, 10.0);
                // let score_dest = bevy::math::Vec2::new(200.0, 10.0);

                // let level_str = format!("Level: 59");
                // let score_str = format!("Score: 23423");

                // canvas.draw(
                //     &graphics::Text::new(level_str),
                //     graphics::DrawParam::from(level_dest).color(tile::PORTAL),
                // );

                // canvas.draw(
                //     &graphics::Text::new(score_str),
                //     graphics::DrawParam::from(score_dest).color(tile::PORTAL),
                // );
                canvas.finish(ctx)?;
                self.should_draw = false;
            }
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        // _repeated: bool,
    ) -> Result<(), GameError> {
        // Just takes in the user input and makes an action based off of it
        if self.title_screen {
            if let Some(key) = input.keycode {
                if key == KeyCode::N {
                    // new game
                    *self = Self::new(ctx, false)?;
                } else if key == KeyCode::L {
                    // load game
                }
            }
        } else {
            if let Some(key) = input.keycode {
                if key == KeyCode::Colon {
                    self.command = true;
                } else if key == KeyCode::Q {
                    self.save_state();
                }
            }

            let world = self.world.as_mut().unwrap();

            if Player::use_input(input, world) {
                // self.player_move_count += 1;
                // if self.player_move_count >= MOVES_TILL_ENERGY_REGEN {
                //     self.world.player.change_energy(1);
                //     self.player_move_count = 0;
                // }
                Projectile::update(world);

                // updates all the enemies in the world, for now only removes them once their health is
                // less than or equal to 0
                Enemy::update(world);
                self.should_draw = true;
            }
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        if !self.title_screen && (_y / TILE_SIZE.1 as f32) as usize >= UNIVERSAL_OFFSET as usize {
            self.world.as_mut().unwrap().player.queued_position = Some(Position::new(
                (_x / TILE_SIZE.0 as f32) as usize,
                (_y / TILE_SIZE.1 as f32) as usize - UNIVERSAL_OFFSET as usize,
            ));
        }
        Ok(())
    }
}

impl State {
    fn save_state(&self) {
        let serialized = ron::to_string(&self.world).unwrap();
        println!("serialized = {serialized}\n\n\n");
        let deserialized: World = ron::from_str(&serialized).unwrap();
        println!("deserialized = {deserialized:?}\n\n\n");
        let serialized = serde_json::to_string(&self.rng).unwrap();
        println!("serialized = {serialized}\n\n\n");
        let deserialized: ChaChaRng = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {deserialized:?}\n\n\n");
    }
    fn load_save() {
        /* Here is how serialization works:
         * In the directory serialization, there are a couple files
         * is_serialized:
         *      Contains either "0" or "1", where one is that there is a game serialized while zero
         *      means there is none
         * world:
         *      Contains the actual world object, written to in RON
         * rng:
         *      Contains the rng object, in JSON
         *
         */

        let mut serialized_game_str = fs::read_to_string("./serialization/is_serialized")
            .expect("Should have been able to read the file")
            .to_string();
        // pops the newline character
        serialized_game_str.pop();

        let serialized_game = serialized_game_str
            .parse::<u8>()
            .expect("Save data corrupted");
        if serialized_game == 0 {
            println!("There are no saved games");
        } else if serialized_game == 1 {
        } else {
            println!("Save data corrupted")
        }
    }
}
