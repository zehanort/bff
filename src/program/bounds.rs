use super::fungetypes::FungeInteger;

#[derive(Default)]
pub struct Bounds<T> {
    lower: (T, T),
    upper: (T, T),
}

impl<T: FungeInteger> Bounds<T> {
    pub fn new(x_lower: T, y_lower: T, x_upper: T, y_upper: T) -> Self {
        Self {
            lower: (x_lower, y_lower),
            upper: (x_upper, y_upper),
        }
    }

    pub fn lower_x(&self) -> T {
        self.lower.0
    }

    pub fn lower_y(&self) -> T {
        self.lower.1
    }

    pub fn upper_x(&self) -> T {
        self.upper.0
    }

    pub fn upper_y(&self) -> T {
        self.upper.1
    }

    pub fn set_lower_x(&mut self, x: T) {
        self.lower.0 = x;
    }

    pub fn set_lower_y(&mut self, y: T) {
        self.lower.1 = y;
    }

    pub fn set_upper_x(&mut self, x: T) {
        self.upper.0 = x;
    }

    pub fn set_upper_y(&mut self, y: T) {
        self.upper.1 = y;
    }

    /// Returns `true` if `(x, y)` coordinates are out of current bounds.
    pub fn out_of_bounds(&self, (x, y): (T, T)) -> bool {
        x < self.lower_x() || x >= self.upper_x() || y < self.lower_y() || y >= self.upper_y()
    }
}
