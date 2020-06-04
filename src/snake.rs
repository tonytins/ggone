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

trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

impl<T> ModuloSigned for T where T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: Self) -> Self {
        (self.clone() % n.clone() + n.clone()) % n.clone()
    }
}

impl GridPosition {
    pub fn new(x: i16, y: i16) -> Self {
        GridPosition { x, y }
    }

    pub fn random(max_x: i16, max_y: i16) -> Self {
        let mut rng = rand::thread_rng();

        (

            rng.gen_range::<i16, i16, i16>(0, max_x),
            rng.gen_range::<i16, i16, i16>(0, max_y),

        ).into()
    }

}