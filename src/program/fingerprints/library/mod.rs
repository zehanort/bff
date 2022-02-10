use anyhow::Result;

use crate::program::{fungetypes::FungeInteger, Program};

pub(super) use base::BASE;
pub(super) use cpli::CPLI;
pub(super) use evar::EVAR;
pub(super) use modu::MODU;
pub(super) use null::NULL;
pub(super) use roma::ROMA;

pub(super) mod base;
pub(super) mod cpli;
pub(super) mod evar;
pub(super) mod modu;
pub(super) mod null;
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

    fn get_instructions(&self) -> &str;

    fn execute(&self, program: &mut Program<T>, instruction: char) -> Result<bool>;
}
