use anyhow::Result;
use std::env;

use super::Fingerprint;
use crate::program::{fungetypes::FungeInteger, Program};

pub struct EVAR;

impl<T: FungeInteger> Fingerprint<T> for EVAR {
    fn get_name(&self) -> &str {
        "EVAR"
    }

    fn get_instructions(&self) -> &str {
        "GNPV"
    }

    fn execute(&self, program: &mut Program<T>, instruction: char) -> Result<bool> {
        match instruction {
            'G' => {
                let key = program.get_string();
                let mut found = false;
                for (k, v) in env::vars() {
                    if k == key {
                        program.push_string(v);
                        found = true;
                        break;
                    }
                }
                if !found {
                    program.push_string("".to_string());
                }
            }
            'N' => program.push(T::from(env::vars().count()).unwrap()),
            'P' => {
                let query = program.get_string();
                let evar: Vec<&str> = query.split('=').collect();
                match evar.get(0) {
                    Some(&key) => {
                        let key = key.to_string();
                        if key == "" {
                            return Ok(true);
                        }
                        let value = evar.get(1).unwrap_or(&"").to_string();
                        env::set_var(key, value);
                    }
                    None => return Ok(true),
                }
            }
            'V' => {
                let i = program.pop().to_usize().unwrap_or_default();
                let evars = env::vars().collect::<Vec<_>>();
                if i >= evars.len() {
                    return Ok(false);
                } else {
                    let (key, value) = &evars[i];
                    program.push_string(format!("{}={}", key, value));
                }
            }
            _ => return Ok(false),
        };
        Ok(true)
    }
}
