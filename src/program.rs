use anyhow::{bail, Context, Error, Result};
use std::default::Default;
use std::{
    fs::File,
    io,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use crate::cursor::Cursor;
use crate::delta::Delta;

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

impl Program {
    /// Helper function that converts an `i32` to a `char`.
    fn i32_to_char(i: i32) -> char {
        let c: u8 = i as u8;
        c as char
    }

    /// Pushes `x` into the program stack.
    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    /// Pops and returns an `i32` from the program stack.
    /// Note that an empty stack "generates" a 0 when poped, as per the Befunge docs.
    fn pop(&mut self) -> i32 {
        match &self.stack.pop() {
            Some(x) => *x,
            None => 0,
        }
    }

    /// Returns the `char` on the `position` coordinates of the program grid.
    fn get_cell(&self, position: (i32, i32)) -> char {
        if position.1 >= self.grid.len() as i32 || position.0 >= self.grid[0].len() as i32 {
            ' '
        } else {
            self.grid[position.1 as usize][position.0 as usize]
        }
    }

    /// Puts `c` on the `position` coordinates of the program grid.
    fn put_cell(&mut self, position: (i32, i32), c: char) {
        self.grid[position.1 as usize][position.0 as usize] = c;
    }

    /**
    Toggles string mode i.e.,
    pushing each character's ASCII value all the way up to the next `"`.
    */
    fn toggle_string_mode(&mut self) {
        self.string_mode = !self.string_mode;
    }

    /**
    A wrapper around the `move` method of the cursor object.
    */
    fn move_cursor(&mut self) {
        self.cursor.r#move(self.bounds);
    }

    /**
    Executes the cell on which the `cursor` lies.
    For the full Befunge 93 instruction list see
    [here](https://en.wikipedia.org/wiki/Befunge#Befunge-93_instruction_list).

    Returns `true` if the `@` cell was executed
    i.e., the program terminated, and `false` otherwise.
    */
    fn execute_current_cell(&mut self) -> Result<bool> {
        let position = self.cursor.position();
        let mut program_terminated = false;

        let x = self.get_cell(position);

        // special case: string mode ON
        if self.string_mode {
            if x == '"' {
                self.toggle_string_mode();
            } else {
                self.push(x as i32);
            }
        } else {
            match x {
                // Push this number on the stack
                d if ('0'..='9').contains(&d) => {
                    // these unwraps can't fail; we are sure we have a digit
                    self.push(d.to_digit(10).unwrap().try_into().unwrap())
                }
                // Addition: Pop a and b, then push a+b
                '+' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a + b);
                }
                // Subtraction: Pop a and b, then push b-a
                '-' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b - a);
                }
                // Multiplication: Pop a and b, then push a*b
                '*' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a * b);
                }
                // Integer division: Pop a and b, then push b/a, rounded towards 0.
                '/' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b / a);
                }
                // Modulo: Pop a and b, then push the remainder of the integer division of b/a.
                '%' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(b % a);
                }
                // Logical NOT: Pop a value. If the value is zero, push 1; otherwise, push zero.
                '!' => {
                    let a = self.pop();
                    self.push(if a == 0 { 1 } else { 0 })
                }
                // Greater than: Pop a and b, then push 1 if b>a, otherwise zero.
                '`' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(if b > a { 1 } else { 0 })
                }
                // Start moving right
                '>' => self.cursor.set_delta(Delta::east()),
                // Start moving left
                '<' => self.cursor.set_delta(Delta::west()),
                // Start moving up
                '^' => self.cursor.set_delta(Delta::north()),
                // Start moving down
                'v' => self.cursor.set_delta(Delta::south()),
                // Start moving in a random cardinal direction
                '?' => self.cursor.set_delta(rand::random()),
                // Pop a value; move right if value=0, left otherwise
                '_' => {
                    let a = self.pop();
                    self.cursor
                        .set_delta(if a == 0 { Delta::east() } else { Delta::west() })
                }
                // Pop a value; move down if value=0, up otherwise
                '|' => {
                    let a = self.pop();
                    self.cursor.set_delta(if a == 0 {
                        Delta::south()
                    } else {
                        Delta::north()
                    })
                }
                // Start string mode: push each character's ASCII value all the way up to the next "
                '"' => self.toggle_string_mode(),
                // Duplicate value on top of the stack
                ':' => {
                    let a = self.pop();
                    self.push(a);
                    self.push(a);
                }
                // Swap two values on top of the stack
                '\\' => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(a);
                    self.push(b);
                }
                // Pop value from the stack and discard it
                '$' => {
                    self.stack.pop();
                }
                // Pop value and output as an integer followed by a space
                '.' => {
                    print!("{} ", self.pop());
                    io::stdout()
                        .flush()
                        .context("Failed to write an integer to stdout")?;
                }
                // Pop value and output as ASCII character
                ',' => {
                    print!("{}", Program::i32_to_char(self.pop()));
                    io::stdout()
                        .flush()
                        .context("Failed to write a character to stdout")?;
                }
                // Bridge: Skip next cell
                '#' => self.move_cursor(),
                // A "put" call (a way to store a value for later use).
                // Pop y, x, and v, then change the character at (x,y) in the program to the character with ASCII value v
                'p' => {
                    let y = self.pop();
                    let x = self.pop();
                    let v = self.pop();
                    self.put_cell((x, y), Program::i32_to_char(v));
                }
                // A "get" call (a way to retrieve data in storage).
                // Pop y and x, then push ASCII value of the character at that position in the program
                'g' => {
                    let y = self.pop();
                    let x = self.pop();
                    let c = self.get_cell((x, y));
                    self.push(c as i32);
                }
                // Ask user for a number and push it
                '&' => {
                    let mut input_text = String::new();
                    io::stdin()
                        .read_line(&mut input_text)
                        .context("Failed while reading a number from stdin")?;

                    self.push(
                        input_text
                            .trim()
                            .parse::<i32>()
                            .context("Failed while parsing input from stdin as integer")?,
                    );
                }
                // Ask user for a character and push its ASCII value
                '~' => {
                    if let Some(b) = std::io::stdin().bytes().next() {
                        let c = b.context("Failed while reading a character from stdin")?;
                        self.push(c as i32);
                    } else {
                        bail!("Failed to read character from stdin")
                    }
                }
                // End program
                '@' => program_terminated = true,
                // No-op. Does nothing
                ' ' => {}
                // Every other character
                // Note that string mode is OFF here
                // (we checked the "ON" case before the match statement)
                // as per the standard, we will reflect
                // (imitating the "r" instruction which will be added later...)
                _ => self.cursor.reflect(),
            }
        }

        self.move_cursor();
        Ok(program_terminated)
    }

    /// Runs the Befunge program.
    pub fn run(&mut self) -> Result<()> {
        loop {
            let program_terminated = self.execute_current_cell().context("Runtime error")?;
            if program_terminated {
                break;
            }
        }
        Ok(())
    }
}
