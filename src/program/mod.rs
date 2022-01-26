use anyhow::{Context, Error, Result};
use bstr::ByteSlice;
use std::{default::Default, fs, path::PathBuf};

use crate::cursor::Cursor;

mod vm;

#[derive(Default)]
pub struct Program<T> {
    grid: Vec<Vec<T>>,
    cursor: Cursor,
    bounds: (T, T),
    stack: Vec<T>,
    string_mode: bool,
}

impl From<Vec<Vec<u8>>> for Program<i32> {
    /**
    Constucts a `Program` from a `Vec` of `String`s i.e.,
    the lines of the Befunge source code.
    */

    fn from(mut code: Vec<Vec<u8>>) -> Self {
        let width = code.iter().map(|line| line.len()).max().unwrap_or(1);
        let height: i32 = code.len() as i32;

        // make all lines have the same width
        for line in &mut code {
            line.resize(width, 32);
        }

        let grid = code
            .iter()
            .map(|line| line.iter().map(|b| *b as i32).collect())
            .collect();

        Self {
            grid,
            bounds: (width as i32, height),
            ..Default::default()
        }
    }
}

impl TryFrom<PathBuf> for Program<i32> {
    /// Constructs a `Program` from the contents of a Befunge source code file.
    type Error = Error;

    fn try_from(filename: PathBuf) -> Result<Self> {
        // step 1: read raw bytes from file
        let contents = fs::read(filename).context("Failed to read Befunge source file")?;
        // step 2: split in lines by...
        let code: Vec<&[u8]> = contents
            .split_str(b"\r\n") // ...\r\n
            .flat_map(|line| line.split_str(b"\r")) // ...\r
            .flat_map(|line| line.split_str(b"\n")) // ...and \n
            .collect();
        let code: Vec<Vec<u8>> = code.iter().map(|line| line.to_vec()).collect();

        Ok(Program::from(code))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_all_lines_same_width() {
        let program = Program::from(vec![
            b"123".to_vec(),
            b"12345678".to_vec(),
            vec![],
            b"12345".to_vec(),
        ]);
        let width = program.grid[0].len();
        assert!(program.grid.iter().all(|line| line.len() == width));
    }
}
