use crate::program::{fungetypes::FungeInteger, Program};

pub(super) use modu::MODU;
pub(super) use roma::ROMA;

pub(super) mod modu;
pub(super) mod roma;

pub trait Fingerprint<T: FungeInteger> {
    fn get_name(&self) -> &str;

    fn get_id(&self) -> T {
        T::from(
            self.get_name()
                .chars()
                .fold(0, |acc, c| acc * 256 + u32::from(c)),
        )
        .unwrap_or_default()
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> bool;
}
