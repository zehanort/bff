use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::default::Default;
use std::ops;

use super::fungetypes::FungeInteger;

pub(super) struct Delta<T: FungeInteger> {
    pub x: T,
    pub y: T,
}

impl<T: FungeInteger> ops::MulAssign<T> for Delta<T> {
    fn mul_assign(&mut self, factor: T) {
        self.x *= factor;
        self.y *= factor;
    }
}

impl<T: FungeInteger> ops::Add<(T, T)> for &Delta<T> {
    type Output = (T, T);

    fn add(self, rhs: (T, T)) -> Self::Output {
        (self.x + rhs.0, self.y + rhs.1)
    }
}

impl<T: FungeInteger> Delta<T> {
    pub fn east() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn south() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }

    pub fn west() -> Self {
        Self {
            x: -T::one(),
            y: T::zero(),
        }
    }

    pub fn north() -> Self {
        Self {
            x: T::zero(),
            y: -T::one(),
        }
    }

    pub fn reflect(&mut self) {
        *self *= -T::one();
    }
}

impl<T: FungeInteger> Default for Delta<T> {
    fn default() -> Self {
        Self::east()
    }
}

impl<T: FungeInteger> Distribution<Delta<T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Delta<T> {
        match rng.gen_range(0..=3) {
            0 => Delta::<T>::east(),
            1 => Delta::<T>::south(),
            2 => Delta::<T>::west(),
            _ => Delta::<T>::north(),
        }
    }
}
