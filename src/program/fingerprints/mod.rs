use self::library::Fingerprint;
use super::{fungetypes::FungeInteger, Program};

use library::*;

mod library;

pub struct FPManager<'a, T: FungeInteger> {
    available: Vec<Box<&'a dyn Fingerprint<T>>>,
    loaded: Vec<Box<&'a dyn Fingerprint<T>>>,
}

impl<'a, T: FungeInteger> FPManager<'a, T> {
    pub fn load(&mut self, id: T) -> bool {
        for fp in self.available.iter() {
            if fp.get_id() == id {
                self.loaded.push(fp.clone());
                return true;
            }
        }
        false
    }

    pub fn unload(&mut self, id: T) -> bool {
        for (i, fp) in self.loaded.iter().enumerate() {
            if fp.get_id() == id {
                self.loaded.remove(i);
                return true;
            }
        }
        false
    }

    pub fn execute(&mut self, program: &mut Program<T>, instruction: char) -> bool {
        for fp in self.loaded.iter_mut().rev() {
            if fp.execute(program, instruction) {
                return true;
            }
        }
        false
    }
}

impl<'a, T: FungeInteger> Default for FPManager<'a, T> {
    fn default() -> Self {
        Self {
            available: vec![Box::new(&ROMA {}), Box::new(&MODU {})],
            loaded: vec![],
        }
    }
}
