use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct ROMA;

impl<T: FungeInteger> Fingerprint<T> for ROMA {
    fn get_name(&self) -> &str {
        "ROMA"
    }

    fn get_instructions(&self) -> &str {
        "CDILMVX"
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> bool {
        match instruction {
            'C' => program.push(T::from(100).unwrap()),
            'D' => program.push(T::from(500).unwrap()),
            'I' => program.push(T::from(1).unwrap()),
            'L' => program.push(T::from(50).unwrap()),
            'M' => program.push(T::from(1000).unwrap()),
            'V' => program.push(T::from(5).unwrap()),
            'X' => program.push(T::from(10).unwrap()),
            _ => return false,
        };
        true
    }
}
