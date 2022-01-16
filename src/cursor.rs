use std::default::Default;

use crate::direction::Direction;

#[derive(Default, Debug)]
pub struct Cursor {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Cursor {
    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn reflect(&mut self) {
        match self.direction {
            Direction::North => self.set_direction(Direction::South),
            Direction::East => self.set_direction(Direction::West),
            Direction::South => self.set_direction(Direction::North),
            Direction::West => self.set_direction(Direction::East),
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
