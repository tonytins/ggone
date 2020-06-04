mod snake;

use std::env;
use std::path;
use std::time::{Duration, Instant};
use std::collections::LinkedList;

use ggez;
use ggez::{event, graphics, Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use rand;
use rand::Rng;

/*
Based on examples from:
https://github.com/ggez/ggez/tree/master/examples
*/

const GRID_SIZE: (i16, i16) = (30, 20);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
    );

const UPDATES_PER_SEC: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SEC * 1000.0) as u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct MainState {
    frames: usize,
    fps_text: graphics::Text,
}


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let fps = format!("FPS: {}", ggez::timer::fps(ctx));
        let fps_text = graphics::Text::new((fps, font, 25.0));
        let state = MainState { frames: 0, fps_text };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        graphics::clear(_ctx, [0.1, 0.2, 0.3, 1.0].into());

        let dest_point = na::Point2::new(10.0, 10.0);

        self.frames += 1;

        if (self.frames % 100) == 0 {
            graphics::draw(_ctx, &self.fps_text, (dest_point,))?;
        } else {
            graphics::draw(_ctx, &self.fps_text, (dest_point,))?;
        }

        graphics::present(_ctx)?;

        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("ggez_example","tonytins")
        .add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
