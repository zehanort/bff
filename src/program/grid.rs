use super::fungetypes::FungeInteger;
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub(super) struct Grid<T: FungeInteger> {
    grid: Vec<Vec<T>>,
    offset: [T; 2],
}

impl<T: FungeInteger> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Self {
        Self {
            grid,
            offset: [T::zero(), T::zero()],
        }
    }
}

impl<T: FungeInteger> Index<T> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: T) -> &Self::Output {
        if index + self.offset[1] >= T::zero() {
            &self.grid[(index + self.offset[1]).to_usize().unwrap_or_default()]
        } else {
            &self.grid[(index + self.offset[1]).to_usize().unwrap_or_default()]
        }
    }
}

impl<T: FungeInteger> IndexMut<T> for Grid<T> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        if index + self.offset[1] >= T::zero() {
            &mut self.grid[(index + self.offset[1]).to_usize().unwrap_or_default()]
        } else {
            &mut self.grid[(index + self.offset[1]).to_usize().unwrap_or_default()]
        }
    }
}

impl<T: FungeInteger> Index<(T, T)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (T, T)) -> &Self::Output {
        let row = &self[index.1 + self.offset[1]];
        if index.0 + self.offset[0] >= T::zero() {
            &row[(index.0 + self.offset[0]).to_usize().unwrap_or_default()]
        } else {
            &row[(index.0 + self.offset[0]).to_usize().unwrap_or_default()]
        }
    }
}

impl<T: FungeInteger> IndexMut<(T, T)> for Grid<T> {
    fn index_mut(&mut self, index: (T, T)) -> &mut Self::Output {
        let offset_x = self.offset[0];
        let offset_y = self.offset[1];
        let row = &mut self[index.1 + offset_y];
        if index.0 + offset_x >= T::zero() {
            &mut row[(index.0 + offset_x).to_usize().unwrap_or_default()]
        } else {
            &mut row[(index.0 + offset_x).to_usize().unwrap_or_default()]
        }
    }
}

impl<T: FungeInteger> Grid<T> {
    /// Returns the number of rows of the Befunge program
    pub fn len(&self) -> usize {
        self.grid.len()
    }
}
