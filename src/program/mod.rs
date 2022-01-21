use anyhow::{Context, Error, Result};
use std::default::Default;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use crate::cursor::Cursor;

mod commands;

#[derive(Default)]
pub struct Program {
    grid: Vec<Vec<char>>,
    cursor: Cursor,
    bounds: (i32, i32),
    stack: Vec<i32>,
    string_mode: bool,
}

impl From<Vec<String>> for Program {
    /**
    Constucts a `Program` from a `Vec` of `String`s i.e.,
    the lines of the Befunge source code.
    */

    fn from(code: Vec<String>) -> Self {
        let width: i32 = match code.iter().map(|line| line.len()).max() {
            Some(w) => w as i32,
            None => 1, // empty program == infinite loop
        };
        let height = code.len() as i32;
        // the `format!` adds padding to the right to make sure that
        // the grid has all the columns needed
        let grid: Vec<Vec<char>> = code
            .iter()
            .map(|line| {
                format!("{:width$}", line, width = width as usize)
                    .chars()
                    .collect()
            })
            .collect();
        Self {
            grid,
            bounds: (width, height),
            ..Default::default()
        }
    }
}

impl TryFrom<PathBuf> for Program {
    /// Constructs a `Program` from the contents of a Befunge source code file.
    type Error = Error;

    fn try_from(filename: PathBuf) -> Result<Self> {
        let file = File::open(filename).context("Failed to open Befunge source file")?;
        let buf = BufReader::new(file);
        let code = buf
            .lines()
            .enumerate()
            .map(|(i, l)| l.context(format!("Failed to parse line {} of source file", i + 1)))
            .collect::<Result<Vec<String>>>()
            .context("Error while parsing lines of source file")?;

        Ok(Program::from(code))
    }
}
