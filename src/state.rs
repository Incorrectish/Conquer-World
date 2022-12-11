use crate::direction::Direction;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::{projectile::Projectile, tile, world::World, SCREEN_SIZE, TILE_SIZE, WORLD_SIZE};
use ggez::{
    event,
    graphics::{self, Canvas},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};
use rand::rngs::ThreadRng;

pub struct State {
    // RGBA values
    r: f32,
    g: f32,
    b: f32,
    a: f32,

    // Abstraction for the world and what is contained within it
    world: World,
}

impl State {
    // just returns the default values
    pub fn new() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
            world: World::new(),
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
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from(tile::GRASS),
        );


        // draw our state matrix "world" to the screen
        // We must partition our window into small sectors of 32 by 32 pixels and then for each
        // individual one, change each of the pixels to the color corresponding to its place in the
        // state matrix
        // for i in 0..WORLD_SIZE.1 {
        //     for j in 0..WORLD_SIZE.0 {
        //         canvas.draw(
        //             &graphics::Quad,
        //             graphics::DrawParam::new()
        //                 .dest_rect(graphics::Rect::new_i32(
        //                     j as i32 * TILE_SIZE.0 as i32,
        //                     i as i32 * TILE_SIZE.1 as i32,
        //                     TILE_SIZE.0 as i32,
        //                     TILE_SIZE.1 as i32,
        //                 ))
        //                 .color(self.world.world[i as usize][j as usize]),
        //         );
        //     }            
        // }

        self.world.draw(&mut canvas);
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
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        // Just takes in the user input and makes an action based off of it
        Player::use_input(input, &mut self.world);
        Projectile::update(&mut self.world);

        // updates all the enemies in the world, for now only removes them once their health is
        // less than or equal to 0
        Enemy::update(&mut self.world);
        
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        Ok(())
    }
}
