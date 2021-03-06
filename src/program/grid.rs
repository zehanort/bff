use super::{bounds::Bounds, fungetypes::FungeInteger};
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub(super) struct Grid<T: FungeInteger> {
    grid: Vec<Vec<T>>,
    bounds: Bounds<T>,
}

impl<T: FungeInteger> From<(Vec<Vec<T>>, Bounds<T>)> for Grid<T> {
    fn from((grid, bounds): (Vec<Vec<T>>, Bounds<T>)) -> Self {
        Self { grid, bounds }
    }
}

impl<T: FungeInteger> Index<T> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: T) -> &Self::Output {
        let y = index - self.bounds.lower_y();
        // if this panics, something is broken in the logic:
        // Execution should NEVER reach here if y < 0
        &self.grid[y.to_usize().unwrap()]
    }
}

// Resizes Funge-Space if necessary
impl<T: FungeInteger> IndexMut<T> for Grid<T> {
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let i_t = index - self.bounds.lower_y();
        if i_t >= T::zero() {
            if i_t >= self.bounds.upper_y() {
                // need to resize height to the positive
                self.grid.resize(
                    i_t.to_usize().unwrap_or_default() + 1,
                    vec![T::from(32).unwrap(); self.grid[0].len()],
                );
                self.bounds.set_upper_y(index + T::one());
            }
        } else {
            // need to resize height to the negative
            let neg_len = i_t.abs().to_usize().unwrap_or_default();
            let mut neg_expansion = vec![vec![T::from(32).unwrap(); self.grid[0].len()]; neg_len];
            neg_expansion.append(&mut self.grid);
            self.grid = neg_expansion;
            self.bounds.set_lower_y(self.bounds.lower_y() + i_t);
        }
        let new_y_t = index - self.bounds.lower_y();
        &mut self.grid[new_y_t.to_usize().unwrap()]
    }
}

impl<T: FungeInteger> Index<(T, T)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (T, T)) -> &Self::Output {
        let x = index.0 - self.bounds.lower_x();
        // if this panics, something is broken in the logic:
        // Execution should NEVER reach here if x < 0
        &self[index.1][x.to_usize().unwrap()]
    }
}

// Resizes Funge-Space if necessary
impl<T: FungeInteger> IndexMut<(T, T)> for Grid<T> {
    fn index_mut(&mut self, index: (T, T)) -> &mut Self::Output {
        let neg_offset_x = self.bounds.lower_x();
        let x_t = index.0 - neg_offset_x;
        if x_t >= T::zero() {
            if x_t >= self.bounds.upper_x() {
                // need to resize width OF ALL ROWS to the positive
                for idx in 0..self.grid.len() {
                    self.grid[idx]
                        .resize(x_t.to_usize().unwrap_or_default() + 1, T::from(32).unwrap());
                }
                self.bounds.set_upper_x(index.0 + T::one());
            }
        } else {
            // need to resize width OF ALL ROWS to the negative
            let neg_len = x_t.abs().to_usize().unwrap_or_default();
            for idx in 0..self.grid.len() {
                let mut neg_expansion = vec![T::from(32).unwrap(); neg_len];
                neg_expansion.append(&mut self.grid[idx]);
                self.grid[idx] = neg_expansion;
            }
            self.bounds.set_lower_x(self.bounds.lower_x() + x_t);
        }
        let new_x_t = index.0 - self.bounds.lower_x();
        &mut self[index.1][new_x_t.to_usize().unwrap()]
    }
}

impl<T: FungeInteger> Grid<T> {
    pub fn get_bounds(&self) -> &Bounds<T> {
        &self.bounds
    }

    /// Wraps the `out_of_bounds` method of the `bounds` struct.
    pub fn out_of_bounds(&self, (x, y): (T, T)) -> bool {
        self.bounds.out_of_bounds((x, y))
    }

    /**
    Returns 1 vector containing the least point
    which contains a non-space cell, relative to the origin
    */
    pub fn get_least_point(&self) -> Vec<T> {
        let (mut x, mut y) = (0, 0);
        for i in 0..self.grid[0].len() {
            if self
                .grid
                .iter()
                .map(|row| row[i])
                .any(|b| b != T::from(32).unwrap_or_default())
            {
                x = i;
                break;
            }
        }
        for j in 0..self.grid.len() {
            if self.grid[j]
                .iter()
                .any(|&b| b != T::from(32).unwrap_or_default())
            {
                y = j;
                break;
            }
        }
        vec![
            T::from(x).unwrap_or_default() + self.bounds.lower_x(),
            T::from(y).unwrap_or_default() + self.bounds.lower_y(),
        ]
    }

    /**
    Returns 1 vector containing the greatest point
    which contains a non-space cell, relative to the least point
    */
    pub fn get_greatest_point(&self) -> Vec<T> {
        let (mut x, mut y) = (0, 0);
        for i in (0..self.grid[0].len()).rev() {
            if self
                .grid
                .iter()
                .map(|row| row[i])
                .any(|b| b != T::from(32).unwrap_or_default())
            {
                x = i;
                break;
            }
        }
        for j in (0..self.grid.len()).rev() {
            if self.grid[j]
                .iter()
                .any(|&b| b != T::from(32).unwrap_or_default())
            {
                y = j;
                break;
            }
        }
        vec![
            T::from(x).unwrap_or_default(),
            T::from(y).unwrap_or_default(),
        ]
    }

    /**
    Checks if a shrinking of the grid space is in order and,
    if it is, it performs it.

    This method should only be called after a `p` instruction that puts a space;
    otherwise, it does nothing and just wastes execution time in redundant checks.
    */
    pub fn shrink(&mut self, position: (T, T)) {
        /* should the rows be shrunk? */
        if position.1 == T::from(self.grid.len()).unwrap() + self.bounds.lower_y() - T::one() {
            // we MAY need to shrink Y from the positive
            while self
                .grid
                .last()
                .unwrap()
                .iter()
                .all(|&c| c == T::from(32).unwrap())
            {
                self.grid.pop();
                self.bounds.set_upper_y(self.bounds.upper_y() - T::one());
            }
        } else if position.1 == self.bounds.lower_y() {
            // we MAY need to shrink Y from the negative
            while self.grid[0].iter().all(|&c| c == T::from(32).unwrap()) {
                self.grid.remove(0);
                self.bounds.set_lower_y(self.bounds.lower_y() + T::one());
            }
        }

        /* should the columns be shrunk? */
        if position.0 == T::from(self.grid[0].len()).unwrap() + self.bounds.lower_x() - T::one() {
            // we MAY need to shrink X from the positive
            while self
                .grid
                .iter()
                .map(|row| row.last().unwrap())
                .all(|&c| c == T::from(32).unwrap())
            {
                for row in self.grid.iter_mut() {
                    row.pop();
                }
                self.bounds.set_upper_x(self.bounds.upper_x() - T::one());
            }
        } else if position.0 == self.bounds.lower_x() {
            // we MAY need to shrink X from the negative
            while self
                .grid
                .iter()
                .map(|row| row[0])
                .all(|c| c == T::from(32).unwrap())
            {
                for row in self.grid.iter_mut() {
                    row.remove(0);
                }
                self.bounds.set_lower_x(self.bounds.lower_x() + T::one());
            }
        }
    }

    #[cfg(test)]
    /// Returns the number of rows of the Befunge program
    pub fn len(&self) -> usize {
        self.grid.len()
    }
}

impl<T: FungeInteger> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.grid.len() {
            writeln!(
                f,
                "{}",
                self.grid[i]
                    .iter()
                    .map(|b| { char::from_u32(b.to_u32().unwrap_or_default()).unwrap_or_default() })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}
