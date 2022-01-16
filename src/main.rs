use std::{env, path::PathBuf};

mod cursor;
mod delta;
mod program;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = PathBuf::from(args.get(1).expect("Befunge program file not provided"));
    let mut program = program::Program::from(filepath);
    program.run();
}
