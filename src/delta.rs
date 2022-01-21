use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::default::Default;
use std::ops;

pub struct Delta {
    pub x: i32,
    pub y: i32,
}

impl ops::MulAssign<i32> for Delta {
    fn mul_assign(&mut self, factor: i32) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl Delta {
    pub fn east() -> Self {
        Self { x: 1, y: 0 }
    }

    pub fn south() -> Self {
        Self { x: 0, y: 1 }
    }

    pub fn west() -> Self {
        Self { x: -1, y: 0 }
    }

    pub fn north() -> Self {
        Self { x: 0, y: -1 }
    }

    pub fn reflect(&mut self) {
        *self *= -1;
    }
}

impl Default for Delta {
    fn default() -> Self {
        Self::east()
    }
}

impl Distribution<Delta> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Delta {
        match rng.gen_range(0..=3) {
            0 => Delta::east(),
            1 => Delta::south(),
            2 => Delta::west(),
            _ => Delta::north(),
        }
    }
}
