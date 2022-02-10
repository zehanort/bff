use anyhow::Result;

use self::library::Fingerprint;
use super::{fungetypes::FungeInteger, Program};

use library::*;

mod library;

type Istacks = [Vec<usize>; 26];

const OFFSET: usize = 'A' as usize;

pub struct FPManager<'a, T: FungeInteger> {
    available: Vec<Box<&'a dyn Fingerprint<T>>>,
    loaded: Vec<usize>,
    istacks: Istacks,
}

impl<'a, T: FungeInteger> FPManager<'a, T> {
    pub fn new() -> Self {
        Self {
            // holds all the implemented fingerprints
            available: vec![
                Box::new(&BASE {}),
                Box::new(&CPLI {}),
                Box::new(&EVAR {}),
                Box::new(&MODU {}),
                Box::new(&NULL {}),
                Box::new(&ROMA {}),
            ],
            // holds the indices in the `available` vector of the fingerprints
            // that have been loaded in order, as a stack
            loaded: vec![],
            // holds a stack for each capital letter;
            // each stack holds the indices in the `loaded` vector of the fingerprints that
            // implement this letter;
            // they are stacks, so the "active" implementation is always the topmost
            istacks: vec![vec![]; 26].try_into().unwrap(),
        }
    }

    pub fn load(&mut self, id: T) -> bool {
        for (fp_idx, fp) in self.available.iter().enumerate() {
            if fp.get_id() == id {
                for instruction in fp.get_instructions().chars() {
                    self.istacks[instruction as usize - OFFSET].push(fp_idx);
                }
                self.loaded.push(fp_idx);
                return true;
            }
        }
        false
    }

    pub fn unload(&mut self, id: T) -> bool {
        for (loaded_fp_idx, &fp_idx) in self.loaded.iter().enumerate().rev() {
            if self.available[fp_idx].get_id() == id {
                self.loaded.remove(loaded_fp_idx);
                let unloaded_fp = &self.available[fp_idx];
                for instruction in unloaded_fp.get_instructions().chars() {
                    self.istacks[instruction as usize - OFFSET].pop();
                }
                return true;
            }
        }
        false
    }

    pub fn execute(&mut self, program: &mut Program<T>, instruction: char) -> Result<bool> {
        let instruction_idx = instruction as usize - OFFSET;
        match self.istacks[instruction_idx].last() {
            Some(&fp_idx) => {
                let fp = &self.available[fp_idx];
                fp.execute(program, instruction)
            }
            None => Ok(false),
        }
    }
}
