use ggez;
use ggez::{event, Context, GameResult};
use ggez::graphics;
use ggez::nalgebra as na;

/*
This is based off of the example on ggez's homepage
in order to a proper feel of the engine.
*/

struct MainState {
    circle_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let state = MainState { circle_x: 0.0 };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Slowly move circle to the right
        self.circle_x = self.circle_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            _ctx,
            graphics::DrawMode::fill(), // mode
            na::Point2::new(self.circle_x, 380.0),
            100.0, // radius
            2.0, // tolerance
            graphics::WHITE,
        )?;

        graphics::draw(_ctx, &circle,
                       (na::Point2::new(0.0,0.0),))?;
        graphics::present(_ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("ggez_example","tonytins");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
