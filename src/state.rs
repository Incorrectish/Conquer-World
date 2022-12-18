use crate::direction::Direction;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::utils::Boss;
use crate::utils::Position;
use crate::UNIVERSAL_OFFSET;
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

pub struct State {
    should_draw: bool,

    // Abstraction for the world and what is contained within it
    world: World,
    player_move_count: usize, 
}

impl State {
    // just returns the default values
    pub fn new() -> Self {
        Self {
            should_draw: true,
            world: World::new(),
            player_move_count: 0,
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
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
