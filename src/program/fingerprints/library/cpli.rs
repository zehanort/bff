use std::io::{self, Write};

use anyhow::{Context, Result};

use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct CPLI;

impl<T: FungeInteger> Fingerprint<T> for CPLI {
    fn get_name(&self) -> &str {
        "CPLI"
    }

    fn get_instructions(&self) -> &str {
        "ADMOSV"
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> Result<bool> {
        match instruction {
            'A' => {
                let (bi, br, ai, ar) = (program.pop(), program.pop(), program.pop(), program.pop());
                program.push(ar + br);
                program.push(ai + bi);
            }
            'D' => {
                let (bi, br, ai, ar) = (program.pop(), program.pop(), program.pop(), program.pop());
                let denom = br * br + bi * bi;
                program.push((ar * br + ai + bi) / denom);
                program.push((ai * br - ar * bi) / denom);
            }
            'M' => {
                let (bi, br, ai, ar) = (program.pop(), program.pop(), program.pop(), program.pop());
                program.push(ar * br - ai * bi);
                program.push(ar * bi + ai * br);
            }
            'O' => {
                let (ai, ar) = (program.pop(), program.pop());
                print!(
                    "{}{}{}i",
                    ar,
                    if ai >= T::zero() { '+' } else { '-' },
                    ai.abs()
                );
                io::stdout()
                    .flush()
                    .context("Failed to write a complex integer to stdout")?;
            }
            'S' => {
                let (bi, br, ai, ar) = (program.pop(), program.pop(), program.pop(), program.pop());
                program.push(ar - br);
                program.push(ai - bi);
            }
            'V' => {
                let (ai, ar) = (program.pop(), program.pop());
                program.push(T::from((ai * ai + ar * ar).to_f32().unwrap().sqrt()).unwrap());
            }
            _ => return Ok(false),
        }
        Ok(true)
    }
}
