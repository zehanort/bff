use anyhow::{Context, Result};
use radix_fmt::radix;
use std::io::{self, Write};

use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct BASE;

impl<T: FungeInteger> Fingerprint<T> for BASE {
    fn get_name(&self) -> &str {
        "BASE"
    }

    fn get_instructions(&self) -> &str {
        "BHINO"
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> Result<bool> {
        match instruction {
            'B' => {
                print!("{:#b}", program.pop());
                io::stdout()
                    .flush()
                    .context("Failed to write a binary integer to stdout")?;
            }
            'H' => {
                print!("{:#x}", program.pop());
                io::stdout()
                    .flush()
                    .context("Failed to write a hexadecimal integer to stdout")?;
            }
            'I' => {
                let b = program.pop().to_u32().unwrap_or(10);
                if b > 36 {
                    return Ok(false);
                }
                let mut input_text = String::new();
                io::stdin()
                    .read_line(&mut input_text)
                    .context("Failed while reading raw input from stdin")?;

                let mut n: T = T::zero();
                let mut discard_done = false;
                let mut negative = 1;
                for dchar in input_text.trim().chars() {
                    if let Some(d) = dchar.to_digit(10) {
                        if !discard_done {
                            discard_done = true;
                        }
                        let (shifted_res, mul_overflowed) =
                            n.overflowing_mul(&T::from(10).unwrap());
                        // u32 -> i32 is safe here, it is just a single digit
                        let (new_res, add_overflowed) =
                            shifted_res.overflowing_add(&T::from(d).unwrap());
                        if mul_overflowed || add_overflowed {
                            break;
                        }
                        n = new_res;
                    } else {
                        // maybe we start reading a negative number?
                        if discard_done {
                            break;
                        } else if dchar == '-' {
                            negative = -1;
                            discard_done = true;
                        }
                    }
                }

                // check if negative underflows
                if negative == -1 {
                    let (neg_res, underflowed) = n.overflowing_mul(&T::from(negative).unwrap());
                    n = if underflowed {
                        neg_res / T::from(10).unwrap()
                    } else {
                        neg_res
                    };
                }

                program.push(if b == 0 {
                    T::zero()
                } else {
                    T::from(
                        char::from_digit(n.to_u32().unwrap_or_default(), b).unwrap_or_default()
                            as u32,
                    )
                    .unwrap_or_default()
                });
            }
            'N' => {
                let b = program.pop().to_u8().unwrap_or(10);
                if b < 2 || b > 36 {
                    return Ok(false);
                }
                let n = program.pop().to_u32().unwrap_or_default();
                print!("{}", radix(n, b));
                io::stdout()
                    .flush()
                    .context("Failed to write an integer of arbitrary base to stdout")?;
            }
            'O' => {
                print!("{:#o}", program.pop());
                io::stdout()
                    .flush()
                    .context("Failed to write a octal integer to stdout")?;
            }
            _ => return Ok(false),
        };
        Ok(true)
    }
}
