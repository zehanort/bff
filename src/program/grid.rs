extern crate num;

use std::ops::{Index, IndexMut};

#[derive(Default)]
pub(super) struct Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    grid: Vec<Vec<T>>,
    offset: [T; 2],
}

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    fn from(grid: Vec<Vec<T>>) -> Self {
        Self {
            grid,
            offset: [T::from::<i32>(0).unwrap(), T::from::<i32>(0).unwrap()],
        }
    }
}

impl<T> Index<usize> for Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if T::from::<usize>(index).unwrap_or_default() + self.offset[1]
            >= T::from::<usize>(0).unwrap()
        {
            &self.grid[(T::from::<usize>(index).unwrap_or_default() + self.offset[1])
                .to_usize()
                .unwrap_or_default()]
        } else {
            &self.grid[(T::from::<usize>(index).unwrap_or_default() + self.offset[1])
                .to_usize()
                .unwrap_or_default()]
        }
    }
}

impl<T> IndexMut<usize> for Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if T::from::<usize>(index).unwrap_or_default() + self.offset[1]
            >= T::from::<i32>(0).unwrap()
        {
            &mut self.grid[index + self.offset[1].to_usize().unwrap_or_default()]
        } else {
            &mut self.grid[index + self.offset[1].to_usize().unwrap_or_default()]
        }
    }
}

impl<T> Index<[usize; 2]> for Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let row = &self[(T::from::<usize>(index[1]).unwrap_or_default() + self.offset[1])
            .to_usize()
            .unwrap_or_default()];
        if T::from::<usize>(index[0]).unwrap_or_default() + self.offset[0]
            >= T::from::<i32>(0).unwrap()
        {
            &row[index[1] + self.offset[0].to_usize().unwrap_or_default()]
        } else {
            &row[index[1] + self.offset[0].to_usize().unwrap_or_default()]
        }
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let offset_x = self.offset[0];
        let offset_y = self.offset[1];
        let row = &mut self[(T::from::<usize>(index[1]).unwrap_or_default() + offset_y)
            .to_usize()
            .unwrap_or_default()];
        if T::from::<usize>(index[0]).unwrap_or_default() + offset_x >= T::from::<i32>(0).unwrap() {
            &mut row[index[1] + offset_x.to_usize().unwrap_or_default()]
        } else {
            &mut row[index[1] + offset_x.to_usize().unwrap_or_default()]
        }
    }
}

impl<T> Grid<T>
where
    T: num::PrimInt + num::Signed + num::ToPrimitive + Default,
{
    /// Returns the number of rows of the Befunge program
    pub fn len(&self) -> usize {
        self.grid.len()
    }
}
