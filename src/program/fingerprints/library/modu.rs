use anyhow::Result;

use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct MODU;

impl<T: FungeInteger> Fingerprint<T> for MODU {
    fn get_name(&self) -> &str {
        "MODU"
    }

    fn get_instructions(&self) -> &str {
        "MUR"
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> Result<bool> {
        if !"MUR".contains(instruction) {
            Ok(false)
        } else {
            let (a, b) = (program.pop(), program.pop());
            if a == T::zero() {
                program.push(T::zero())
            } else {
                match instruction {
                    'M' => program.push((b % a + a) % a),
                    'U' => {
                        let r = b % a;
                        program.push(if r < T::zero() {
                            if a > T::zero() {
                                r + a
                            } else {
                                -a + r
                            }
                        } else {
                            r
                        });
                    }
                    'R' => program.push(b % a),
                    _ => return Ok(false),
                };
            }
            Ok(true)
        }
    }
}
