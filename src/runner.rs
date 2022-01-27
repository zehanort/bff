use anyhow::{Error, Result};
use std::path::PathBuf;

use crate::program::{fungetypes::FungeInteger, Program};

pub struct Runner<T: FungeInteger> {
    program: Program<T>,
}

impl<T: FungeInteger> TryFrom<(Option<PathBuf>, Option<String>)> for Runner<T> {
    type Error = Error;

    fn try_from(source: (Option<PathBuf>, Option<String>)) -> Result<Self> {
        let program = match source.0 {
            Some(filepath) => Program::<T>::try_from(filepath)?,
            None => {
                // unwrap is safe here
                // args.ucode is a "Some" for sure at this point
                let unefunge_code = source.1.unwrap().bytes().collect();
                Program::<T>::from(vec![unefunge_code])
            }
        };

        Ok(Self { program })
    }
}

impl<T: FungeInteger> Runner<T> {
    pub fn run(&mut self) -> Result<()> {
        self.program.run()
    }
}
