use std::default::Default;

use super::{delta::Delta, fungetypes::FungeInteger};

#[derive(Default)]
pub(super) struct Cursor<T: FungeInteger> {
    x: T,
    y: T,
    delta: Delta<T>,
}

impl<T: FungeInteger> Cursor<T> {
    pub fn position(&self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn delta(&self) -> &Delta<T> {
        &self.delta
    }

    pub fn set_delta(&mut self, new_delta: Delta<T>) {
        self.delta = new_delta;
    }

    pub fn set_position(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }

    /// Reflects delta to point to "the opposite way".
    pub fn reflect(&mut self) {
        self.delta.reflect();
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
    pub fn r#move(&mut self, bounds: (T, T)) {
        let (x, y) = self.position();
        let delta = self.delta();
        let mut new_x = x + T::from(delta.x).unwrap();
        let mut new_y = y + T::from(delta.y).unwrap();
        if new_x < T::zero() {
            new_x = bounds.0 - T::one();
        }
        if new_y < T::zero() {
            new_y = bounds.1 - T::one();
        }
        if new_x >= bounds.0 {
            new_x = T::zero();
        }
        if new_y >= bounds.1 {
            new_y = T::zero();
        }
        self.set_position(new_x, new_y);
    }
}
