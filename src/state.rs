use crate::direction::Direction;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::utils::Boss;
use crate::utils::Position;
use crate::UNIVERSAL_OFFSET;
use ggez::audio;
use ggez::audio::SoundSource;

use crate::{
    projectile::Projectile,
    tile,
    world::{World, BOSS_ROOMS, FINAL_BOSS_ROOM},
    SCREEN_SIZE, TILE_SIZE, WORLD_SIZE,
};
use ggez::{
    event,
    graphics::{self, Canvas},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

// const MOVES_TILL_ENERGY_REGEN: usize = 5;

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    should_draw: bool,
    command: bool,
    songs: [audio::Source; 6],
    // Abstraction for the world and what is contained within it
    world: World,
    player_move_count: usize,
    player_curr_world_position: Position,
}

impl State {
    // just returns the default values
    pub fn new(ctx: &mut Context) -> GameResult<State> {
        let songs = [
        audio::Source::new(ctx, "/overworld.ogg")?,
        audio::Source::new(ctx, "/final_boss.ogg")?,
        audio::Source::new(ctx, "/blackout_boss.ogg")?,
        audio::Source::new(ctx, "/column_laser_boss.ogg")?,
        audio::Source::new(ctx, "/chasing_boss.ogg")?,
        audio::Source::new(ctx, "/laser_boss.ogg")?];
        let temp = State {
            should_draw: true,
            command: false,
            songs,
            world: World::new(),
            player_move_count: 0,
            player_curr_world_position: Position::new(0,0),
        };
        Ok(temp)
    }

    pub fn from(
        should_draw: bool,
        is_playing: bool,
        world: World,
        player_move_count: usize,
        ctx: &mut Context,
    ) -> GameResult<State> {
        let songs = [
        audio::Source::new(ctx, "/overworld.ogg")?,
        audio::Source::new(ctx, "/final_boss.ogg")?,
        audio::Source::new(ctx, "/blackout_boss.ogg")?,
        audio::Source::new(ctx, "/column_laser_boss.ogg")?,
        audio::Source::new(ctx, "/chasing_boss.ogg")?,
        audio::Source::new(ctx, "/laser_boss.ogg")?];
        let temp = State {
            should_draw,
            command: false,
            songs,
            world,
            player_move_count,
            player_curr_world_position: Position::new(0,0),
        };
        Ok(temp)
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let world_pos = self.world.world_position;
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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // render the graphics with the rgb value, it will always be black though because the
        // self.r,g,b,a values never change from 0
        if self.should_draw {
            let mut canvas;
            let mut boss_room = false;
            let mut final_boss = false;
            for boss_room_position in BOSS_ROOMS {
                if self.world.world_position == boss_room_position {
                    boss_room = true;
                }
            }
            if self.world.world_position == FINAL_BOSS_ROOM {
                final_boss = true;
            }
            if final_boss {
                canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::BOSS_FLOOR));
            } else if boss_room {
                canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::FLOOR));
            } else {
                canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from(tile::GRASS));
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
            }
            self.world.draw(&mut canvas);

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
        Ok(())
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        // _repeated: bool,
    ) -> Result<(), GameError> {
        // Just takes in the user input and makes an action based off of it
        if let Some(key) = input.keycode {
            if key == KeyCode::Colon {
                self.command = true;
            } else if key == KeyCode::Q {
                Self::save_state(self);
            }
        }

        if Player::use_input(input, &mut self.world) {
            self.player_move_count += 1;
            // if self.player_move_count >= MOVES_TILL_ENERGY_REGEN {
            //     self.world.player.change_energy(1);
            //     self.player_move_count = 0;
            // }
            Projectile::update(&mut self.world);

            // updates all the enemies in the world, for now only removes them once their health is
            // less than or equal to 0
            Enemy::update(&mut self.world);
            self.should_draw = true;
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
        if (_y / TILE_SIZE.1 as f32) as usize >= UNIVERSAL_OFFSET as usize {
            self.world.player.queued_position = Some(Position::new(
                (_x / TILE_SIZE.0 as f32) as usize,
                (_y / TILE_SIZE.1 as f32) as usize - UNIVERSAL_OFFSET as usize,
            ));
        }
        Ok(())
    }
}

impl State {
    fn save_state(state: &mut State) {}
}
