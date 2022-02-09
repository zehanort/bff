use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct MODU;

impl<T: FungeInteger> Fingerprint<T> for MODU {
    fn get_name(&self) -> &str {
        "MODU"
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> bool {
        if !"MUR".contains(instruction) {
            false
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
                    _ => {}
                };
            }
            true
        }
    }
}
