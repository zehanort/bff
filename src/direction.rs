use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::default::Default;

#[derive(Debug)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Up,
        }
    }
}
