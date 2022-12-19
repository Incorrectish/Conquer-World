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
use std::io::Write;

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

pub const RNG_SEED: u64 = 0;
// const MOVES_TILL_ENERGY_REGEN: usize = 5;

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    should_draw: bool,
    command: bool,
    songs: [audio::Source; 8],
    // Abstraction for the world and what is contained within it
    world: Option<World>,
    title_screen: bool,
    pub rng: Option<ChaCha8Rng>,
    player_curr_world_position: Position,
}

impl State {
    // just returns the default values
    pub fn new(ctx: &mut Context, title_screen: bool) -> GameResult<State> {
        let songs = [
            audio::Source::new(ctx, "/overworld.ogg")?,
            audio::Source::new(ctx, "/final_boss.ogg")?,
            audio::Source::new(ctx, "/blackout_boss.ogg")?,
            audio::Source::new(ctx, "/column_laser_boss.ogg")?,
            audio::Source::new(ctx, "/chasing_boss.ogg")?,
            audio::Source::new(ctx, "/laser_boss.ogg")?,
            audio::Source::new(ctx, "/title_music.ogg")?,
            audio::Source::new(ctx, "/Sad_Violin_-_Sound_Effect_(HD).ogg")?
        ];
        let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
        let temp = State {
            should_draw: true,
            command: false,
            songs,
            world: Some(World::new(&mut rng)),
            title_screen,
            rng: Some(rng),
            player_curr_world_position: Position::new(0, 0),
        };
        Ok(temp)
    }

    pub fn title_screen(ctx: &mut Context) -> GameResult<State> {
        // TODO make this the title screen music
        let songs = [
            audio::Source::new(ctx, "/overworld.ogg")?,
            audio::Source::new(ctx, "/final_boss.ogg")?,
            audio::Source::new(ctx, "/blackout_boss.ogg")?,
            audio::Source::new(ctx, "/column_laser_boss.ogg")?,
            audio::Source::new(ctx, "/chasing_boss.ogg")?,
            audio::Source::new(ctx, "/laser_boss.ogg")?,
            audio::Source::new(ctx, "/title_music.ogg")?,
            audio::Source::new(ctx, "/Sad_Violin_-_Sound_Effect_(HD).ogg")?
        ];
        Ok(State {
            should_draw: true,
            command: false,
            songs,
            world: None,
            title_screen: true,
            rng: None,
            player_curr_world_position: Position::new(0, 0),
        })
    }

    pub fn from(world: World, ctx: &mut Context, rng: ChaCha8Rng) -> GameResult<State> {
        let songs = [
            audio::Source::new(ctx, "/overworld.ogg")?,
            audio::Source::new(ctx, "/final_boss.ogg")?,
            audio::Source::new(ctx, "/blackout_boss.ogg")?,
            audio::Source::new(ctx, "/column_laser_boss.ogg")?,
            audio::Source::new(ctx, "/chasing_boss.ogg")?,
            audio::Source::new(ctx, "/laser_boss.ogg")?,
            audio::Source::new(ctx, "/title_music.ogg")?,
            audio::Source::new(ctx, "/Sad_Violin_-_Sound_Effect_(HD).ogg")?
        ];
        let temp = State {
            should_draw: true,
            command: false,
            songs,
            world: Some(world),
            title_screen: false,
            rng: Some(rng),
            player_curr_world_position: Position::new(0, 0),
        };
        Ok(temp)
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.title_screen {
            if !self.world.as_ref().unwrap().player.is_alive() {
                if !self.songs[7].playing() {
                    for song in &mut self.songs {
                        song.stop(ctx);
                    }
                    let _ = self.songs[7].set_repeat(true);
                    let _ = self.songs[7].play(ctx);
                }
            } else {
                let world_pos = self.world.as_mut().unwrap().world_position;
                let boss_rooms = BOSS_ROOMS;
                if world_pos == boss_rooms[0] {
                    if !self.songs[5].playing() {
                        for song in &mut self.songs {
                            song.stop(ctx);
                        }
                        let _ = self.songs[5].set_repeat(true);
                        let _ = self.songs[5].play(ctx);
                    }
                } else if world_pos == boss_rooms[1] {
                    if !self.songs[3].playing() {
                        for song in &mut self.songs {
                            song.stop(ctx);
                        }
                        let _ = self.songs[3].set_repeat(true);
                        let _ = self.songs[3].play(ctx);
                    }
                } else if world_pos == boss_rooms[3] {
                    if !self.songs[4].playing() {
                        for song in &mut self.songs {
                            song.stop(ctx);
                        }
                        let _ = self.songs[4].set_repeat(true);
                        let _ = self.songs[4].play(ctx);
                    }
                } else if world_pos == boss_rooms[4] {
                    if !self.songs[2].playing() {
                        for song in &mut self.songs {
                            song.stop(ctx);
                        }
                        let _ = self.songs[2].set_repeat(true);
                        let _ = self.songs[2].play(ctx);
                    }
                } else if world_pos == boss_rooms[2] {
                    if !self.songs[1].playing() {
                        for song in &mut self.songs {
                            song.stop(ctx);
                        }
                        let _ = self.songs[1].set_repeat(true);
                        let _ = self.songs[1].play(ctx);
                    }
                } else {
                    if !self.songs[0].playing() {
                        for song in &mut self.songs {
                            song.stop(ctx);
                        }
                        let _ = self.songs[0].set_repeat(true);
                        let _ = self.songs[0].play(ctx);
                    }
                }
            }
        } else {
            if !self.songs[6].playing() {
                let _ = self.songs[6].set_repeat(true);
                let _ = self.songs[6].play(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.title_screen {
            let canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::TITLE_SCREEN_FLOOR));
            canvas.finish(ctx)?;
        } else if !self.world.as_mut().unwrap().player.is_alive() {
            let canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::BLACK));
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
                    *self = Self::load_save(ctx).unwrap();
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

            if Player::use_input(input, world, self.rng.as_mut().unwrap()) {
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
        if self.world.as_ref().unwrap().player.is_alive() {
            let serialized_world = ron::to_string(self.world.as_ref().unwrap()).unwrap();
            fs::write("./serialization/world", serialized_world.as_bytes());
            let serialized_rng = serde_json::to_string(self.rng.as_ref().unwrap()).unwrap();
            fs::write("./serialization/rng", serialized_rng.as_bytes());
            fs::write("./serialization/is_serialized", b"1");
        }
        std::process::exit(0);
    }
    fn load_save(ctx: &mut Context) -> Option<State> {
        /* Here is how serialization works:
         * serialization
         *      is_serialized:
         *          Contains either "0" or "1", where one is that there is a game serialized
         *          while zero means there is none
         *      world:
         *          Contains the actual world object, written to in RON
         *      rng:
         *          Contains the rng object, in JSON
         *
         */

        let mut serialized_game_str = fs::read_to_string("./serialization/is_serialized")
            .expect("Should have been able to read the file")
            .to_string();
        // pops the newline character
        if serialized_game_str.len() > 1 {
            serialized_game_str.pop();
        }

        let serialized_game = serialized_game_str
            .parse::<u8>()
            .expect("Save data corrupted");
        return if serialized_game == 0 {
            println!("No serialized game");
            None
        } else if serialized_game == 1 {
            let world_str =
                fs::read_to_string("./serialization/world").expect("Couldn't read world file");
            let world: World = ron::from_str(&world_str).unwrap();
            let rng_str =
                fs::read_to_string("./serialization/rng").expect("Couldn't read rng file");
            let rng: ChaCha8Rng = serde_json::from_str(&rng_str).unwrap();
            return Some(State::from(world, ctx, rng).expect("couldn't do audio for some reason"));
        } else {
            panic!("Save data corrupted")
        };
    }
}
