use anyhow::Result;

use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct NULL;

impl<T: FungeInteger> Fingerprint<T> for NULL {
    fn get_name(&self) -> &str {
        "NULL"
    }

    fn get_instructions(&self) -> &str {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    }

    fn execute(&self, program: &mut Program<T>, _: char) -> Result<bool> {
        program.cursor.reflect();
        Ok(true)
    }
}
