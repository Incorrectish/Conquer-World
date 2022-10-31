use ggez::*;

pub struct State {
    delta: u128,
}

impl State {
    pub fn new() -> Self {
        Self { delta: 0 }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut time = ctx.time.delta().as_nanos();
        self.delta += time;
        println!("time = {time}");
        println!("Delta = {}", self.delta);
        println!("Delta > 10 billion? {}", self.delta > 10000000000);
        if self.delta > 10000000000 {
            let r = (time % 100) as f32 / 100.0;
            time /= 100;
            let g = (time % 100) as f32 / 100.0;
            time /= 100;
            let b = (time % 100) as f32 / 100.0;
            time /= 100;
            let a = (time % 100) as f32 / 100.0;
            time /= 100;
            let mut canvas =
                graphics::Canvas::from_frame(ctx, graphics::Color::from([r, g, b, a]));
            canvas.finish(ctx)?;
            self.delta = 0;
        }
        Ok(())
    }
}
