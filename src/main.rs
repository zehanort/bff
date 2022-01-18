use anyhow::{Context, Result};
use std::{env, path::PathBuf};

mod cursor;
mod delta;
mod program;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filepath = PathBuf::from(
        args.get(1)
            .context("No Befunge 98 program file was provided")?,
    );
    let mut program = program::Program::try_from(filepath)?;
    program.run()
}
