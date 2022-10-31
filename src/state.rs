use ggez::*;
use crate::{TILE_SIZE, SCREEN_SIZE, WORLD_SIZE};

pub struct State {
    delta: u128,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
    tile: i16,
    world: [[[f32; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize],
}

impl State {
    pub fn new() -> Self {
        Self { delta: 0, r: 0., g: 0., b: 0., a: 0., tile: 0, world: [[[0.; 4]; WORLD_SIZE.0 as usize]; WORLD_SIZE.1 as usize] }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut time = ctx.time.delta().as_nanos();
        self.delta += time;
        // println!("time = {time}");
        // println!("Delta = {}", self.delta);
        // println!("Delta > 1 billion? {}", self.delta > 1000000000);
        let r = (time % 100) as f32 / 100.0;
        time /= 100;
        let g = (time % 100) as f32 / 100.0;
        time /= 100;
        let b = (time % 100) as f32 / 100.0;
        time /= 100;
        let a = (time % 100) as f32 / 100.0;
        // if self.delta > 1000000000 {
        //     (self.r, self.g, self.b, self.a) = (r, g, b, a);
        //     self.delta = 0;
        // }
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([self.r, self.g, self.b, self.a]));
        let color = [r, g, b, a];
        let pos_y = self.tile / WORLD_SIZE.0; 
        let pos_x = self.tile % WORLD_SIZE.0; 
        self.tile = if self.tile == WORLD_SIZE.0 * WORLD_SIZE.1 - 1 { 0 } else { self.tile + 1 };
            // 0 0 0 0 0 0 0 0 0 0 
            // 0
        self.world[pos_y as usize][pos_x as usize] = color;
        for i in 0..WORLD_SIZE.1 {
            for j in 0..WORLD_SIZE.0 {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                    .dest_rect(
                        graphics::Rect::new_i32(
                            j as i32 * TILE_SIZE.0 as i32,
                            i as i32 * TILE_SIZE.1 as i32,
                            TILE_SIZE.0 as i32,
                            TILE_SIZE.1 as i32,
                            )
                        )
                    .color(self.world[i as usize][j as usize]),
                    );
            }
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}
