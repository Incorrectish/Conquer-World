use crate::direction::Direction;
use crate::player::Player;
use crate::{SCREEN_SIZE, TILE_SIZE, WORLD_SIZE};
use ggez::{
    event,
    graphics::{self, Canvas},
    input::keyboard::{KeyCode, KeyInput},
    Context, GameError, GameResult,
};

pub struct State {
    // Time delta, unused for now I think
    delta: u128,
    // RGBA values
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    // Current tile, in order to iterate over the thing
    tile: i16,

    // world to store the state of tiles in between frames
    world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],

    // store an instance of a player
    player: Player,
}

impl State {
    // just returns the default values
    pub fn new() -> Self {
        let world = [[[0.; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize]; 
        Self {
            delta: 0,
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
            tile: 0,
            world,
            player: Player::new(world),
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // let mut time = ctx.time.delta().as_nanos();
        // self.delta += time;

        // I observed that the time.delta() was always around an 8 digit number in an unoptimized
        // build, so I partitioned it into 4 blocks of 2 digit numbers for a psuedorandom number
        // for each of the values
        // let r = (time % 100) as f32 / 100.0;
        // time /= 100;
        // let g = (time % 100) as f32 / 100.0;
        // time /= 100;
        // let b = (time % 100) as f32 / 100.0;
        // time /= 100;
        // let a = (time % 100) as f32 / 100.0;

        // this was just drawing the frames different colors each second, that is what the delta
        // was for
        // if self.delta > 1000000000 {
        //     (self.r, self.g, self.b, self.a) = (r, g, b, a);
        //     self.delta = 0;
        // }

        // render the graphics with the rgb value, it will always be black though because the
        // self.r,g,b,a values never change from 0
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([self.r, self.g, self.b, self.a]),
        );

        // create our psuedorandomized color from the time delta
        // let color = [r, g, b, a];

        // every frame we color another square in our world, so find the x and y positions of the
        // next square. Becuse the value of tile varies from between 0 and WORLD_SIZE.0 *
        // WORLD_SIZE.1 - 1, we have to get the x and y coordinates by modding/dividing by the row
        // length so that increasing one from the end of a row wraps around to the start of the row
        // let pos_y = self.tile / WORLD_SIZE.0;
        // let pos_x = self.tile % WORLD_SIZE.0;

        // Make sure that the wrap also applies to the last block, so that increasing from the last
        // column in the last row wraps to the first column of the first row
        // self.tile = if self.tile == WORLD_SIZE.0 * WORLD_SIZE.1 - 1 {
        //     0
        // } else {
        //     self.tile + 1
        // };

        // set the next tile to the psuedorandom color that we got from our time delta
        // self.world[pos_y as usize][pos_x as usize] = color;

        // draw our state matrix "world" to the screen
        // We must partition our window into small sectors of 32 by 32 pixels and then for each
        // individual one, change each of the pixels to the color corresponding to its place in the
        // state matrix
        for i in 0..WORLD_SIZE.1 {
            for j in 0..WORLD_SIZE.0 {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(graphics::Rect::new_i32(
                            j as i32 * TILE_SIZE.0 as i32,
                            i as i32 * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                        ))
                        .color(self.world[i as usize][j as usize]),
                );
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        // ALT, SUPER KEY RESULTS IN A "NONE" VALUE CRASHING THIS STUFF
        self.player.use_input(input, &mut self.world);
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
}
