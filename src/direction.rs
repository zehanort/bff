use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::default::Default;

#[derive(Debug)]
pub enum Direction {
    East,
    South,
    West,
    North,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::North,
        }
    }
}
