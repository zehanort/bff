use std::{env, path::PathBuf};

mod cursor;
mod direction;
mod program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = PathBuf::from(
        args.iter()
            .nth(1)
            .expect("Befunge program file not provided"),
    );
    let mut program = program::Program::from(filepath);
    program.run();
}
