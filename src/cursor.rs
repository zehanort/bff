use std::default::Default;

use crate::delta::Delta;

#[derive(Default)]
pub struct Cursor {
    x: i32,
    y: i32,
    delta: Delta,
}

impl Cursor {
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn delta(&self) -> &Delta {
        &self.delta
    }

    pub fn set_delta(&mut self, new_delta: Delta) {
        self.delta = new_delta;
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    /// Reflects delta to point to "the opposite way".
    pub fn reflect(&mut self) {
        self.delta *= -1;
    }

    /// Rotates delta 90 degrees to the left (counterclockwise).
    pub fn turn_left(&mut self) {
        let x = self.delta.x;
        let y = self.delta.y;
        self.delta.x = y;
        self.delta.y = -x;
    }

    /// Rotates delta 90 degrees to the right (clockwise).
    pub fn turn_right(&mut self) {
        let x = self.delta.x;
        let y = self.delta.y;
        self.delta.x = -y;
        self.delta.y = x;
    }

    /**
    Moves the cursor one step on the `delta` direction
    and takes care of any possible wrap-around, effectively updating
    the cursor's `position`.
    */
    pub fn r#move(&mut self, bounds: (i32, i32)) {
        let (x, y) = self.position();
        let delta = self.delta();
        let mut new_x = x + delta.x;
        let mut new_y = y + delta.y;
        if new_x < 0 {
            new_x = bounds.0 - 1;
        }
        if new_y < 0 {
            new_y = bounds.1 - 1;
        }
        if new_x >= bounds.0 {
            new_x = 0;
        }
        if new_y >= bounds.1 {
            new_y = 0;
        }
        self.set_position(new_x, new_y);
    }
}
