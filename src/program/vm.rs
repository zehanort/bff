use crate::delta::Delta;

use super::Program;
use anyhow::{bail, Context, Result};
use colour::e_yellow;
use std::io::{self, Read, Write};

macro_rules! warning {
    ($message:expr) => {
        e_yellow!("warning: ");
        eprintln!($message);
    };
}

impl Program<i32> {
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

    /// Returns the `i32` on the `position` coordinates of the program grid.
    fn get_cell(&self, position: (i32, i32)) -> i32 {
        if position.1 >= self.grid.len() as i32 || position.0 >= self.grid[0].len() as i32 {
            32 // space
        } else {
            self.grid[position.1 as usize][position.0 as usize]
        }
    }

    /// Puts `c` on the `position` coordinates of the program grid.
    fn put_cell(&mut self, position: (i32, i32), c: i32) {
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
    Returns the next instruction at the direction of `delta`,
    respecting the wraparound rules.
    It ignores the ` ` and `;` instructions.
    */
    fn peek(&mut self) -> char {
        let mut times_moved = 0;
        let mut current_cell = ' ';

        // find next useful instruction
        while vec![' ', ';'].contains(&current_cell) {
            // skip `;` blocks
            if current_cell == ';' {
                loop {
                    self.move_cursor();
                    times_moved += 1;
                    current_cell = char::from_u32(self.get_cell(self.cursor.position()) as u32)
                        .unwrap_or_default();
                    if current_cell == ';' {
                        break;
                    }
                }
            }
            self.move_cursor();
            times_moved += 1;
            current_cell =
                char::from_u32(self.get_cell(self.cursor.position()) as u32).unwrap_or_default();
        }

        // reset cursor
        self.cursor.reflect();
        for _ in 0..times_moved {
            self.move_cursor();
        }
        self.cursor.reflect();

        current_cell
    }

    /*
    INSTRUCTION SET IMPLEMENTATIONS
    One method for each instruction follows
    */

    /**
    Executes the cell on which the `cursor` lies.
    For the full Befunge 93 instruction list see
    [here](https://en.wikipedia.org/wiki/Befunge#Befunge-93_instruction_list).

    Returns `true` if the `@` cell was executed
    i.e., the program terminated, and `false` otherwise.
    */
    fn execute_current_cell(&mut self) -> Result<bool> {
        // define a helper macro for overflow checks
        macro_rules! push_with_overflow_check {
            ($a:expr, $op:ident, $b:expr, $message:expr) => {
                let (res, overflowed) = $a.$op($b);
                if overflowed {
                    warning!($message);
                }
                self.push(res);
            };
        }

        let position = self.cursor.position();
        let mut k = 1;
        let mut program_terminated = false;

        let x = self.get_cell(position);

        // special case: string mode ON
        if self.string_mode {
            if x == '"' as i32 {
                self.toggle_string_mode();
            } else {
                self.push(x as i32);
            }
        } else if let Some(mut xchar) = char::from_u32(x as u32) {
            while k > 0 {
                match xchar {
                    // Push this decimal number on the stack
                    d if ('0'..='9').contains(&d) => {
                        // these unwraps can't fail; we are sure we have a digit
                        self.push(d.to_digit(10).unwrap().try_into().unwrap());
                    }
                    // Push this hex number on the stack
                    d if ('a'..='f').contains(&d) => {
                        // these unwraps can't fail; we are sure we have a (hex) digit
                        self.push(d.to_digit(16).unwrap().try_into().unwrap());
                    }
                    // Addition: Pop a and b, then push a+b
                    '+' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            a,
                            overflowing_add,
                            b,
                            "An addition resulted in overflow."
                        );
                    }
                    // Subtraction: Pop a and b, then push b-a
                    '-' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            b,
                            overflowing_sub,
                            a,
                            "A subtraction resulted in overflow."
                        );
                    }
                    // Multiplication: Pop a and b, then push a*b
                    '*' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            a,
                            overflowing_mul,
                            b,
                            "A multiplication resulted in overflow."
                        );
                    }
                    /*
                    Integer division: Pop a and b, then push b/a, rounded towards 0.
                    [SPEC] division by 0 returns 0
                    */
                    '/' => {
                        let (a, b) = (self.pop(), self.pop());
                        if a == 0 {
                            warning!("Division by 0 occured. Will return 0 as per the language specification.");
                            self.push(0);
                        } else {
                            push_with_overflow_check!(
                                b,
                                overflowing_div,
                                a,
                                "A division resulted in overflow."
                            );
                        }
                    }
                    // Modulo: Pop a and b, then push the remainder of the integer division of b/a.
                    '%' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            b,
                            overflowing_rem,
                            a,
                            "A remainder operation resulted in overflow."
                        );
                    }
                    // Logical NOT: Pop a value. If the value is zero, push 1; otherwise, push zero.
                    '!' => {
                        let a = self.pop();
                        self.push(if a == 0 { 1 } else { 0 })
                    }
                    // Greater than: Pop a and b, then push 1 if b>a, otherwise zero.
                    '`' => {
                        let (a, b) = (self.pop(), self.pop());
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
                        let (a, b) = (self.pop(), self.pop());
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
                        let (y, x, v) = (self.pop(), self.pop(), self.pop());
                        self.put_cell((x, y), v);
                    }
                    // A "get" call (a way to retrieve data in storage).
                    // Pop y and x, then push ASCII value of the character at that position in the program
                    'g' => {
                        let (y, x) = (self.pop(), self.pop());
                        let c = self.get_cell((x, y));
                        self.push(c as i32);
                    }
                    /*
                    Ask user for a number and push it
                    [SPEC] Decimal input reads and discards characters until it encounters decimal digit characters,
                    at which point it reads a decimal number from those digits, up until (but not including) the point at which
                    input characters stop being digits, or the point where the next digit would cause a cell overflow, whichever comes first.

                    Design choice: If input is empty or it contains characters only, the command will read 0.
                    */
                    '&' => {
                        let mut input_text = String::new();
                        io::stdin()
                            .read_line(&mut input_text)
                            .context("Failed while reading raw input from stdin")?;

                        let mut res: i32 = 0;
                        let mut discard_done = false;
                        let mut negative = 1;
                        for dchar in input_text.trim().chars() {
                            if let Some(d) = dchar.to_digit(10) {
                                if !discard_done {
                                    discard_done = true;
                                }
                                let (shifted_res, mul_overflowed) = res.overflowing_mul(10);
                                // u32 -> i32 is safe here, it is just a single digit
                                let (new_res, add_overflowed) =
                                    shifted_res.overflowing_add(d as i32);
                                if mul_overflowed || add_overflowed {
                                    break;
                                }
                                res = new_res;
                            } else {
                                // maybe we start reading a negative number?
                                if discard_done {
                                    break;
                                } else if dchar == '-' {
                                    negative = -1;
                                    discard_done = true;
                                }
                            }
                        }

                        // check if negative underflows
                        if negative == -1 {
                            let (neg_res, underflowed) = res.overflowing_mul(negative);
                            res = if underflowed { neg_res / 10 } else { neg_res };
                        }

                        self.push(res);
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
                    // Turn left
                    '[' => self.cursor.turn_left(),
                    // Turn right
                    ']' => self.cursor.turn_right(),
                    // Jump over i.e., execute nothing until next ";"
                    ';' => loop {
                        self.move_cursor();
                        if char::from_u32(self.get_cell(self.cursor.position()) as u32)
                            .unwrap_or_default()
                            == ';'
                        {
                            break;
                        }
                    },
                    // Iterate i.e., execute the next "useful" instruction (i.e., not ` ` or `;`)
                    // A very thorough explanation of the `k` instruction and of its special cases
                    // can be found here: http://www.rcfunge98.com/tutorial4.html
                    'k' => {
                        k = self.pop();
                        if k == 0 {
                            self.move_cursor(); // simply skip next instruction, like `#`
                        } else {
                            k += 1; // to counter the -= 1 at the end of the while loop
                            xchar = self.peek();
                        }
                    }
                    // Every other character
                    // Note that string mode is OFF here
                    // (we checked the "ON" case before the match statement)
                    // as per the standard, we will reflect
                    // (imitating the "r" instruction which will be added later...)
                    _ => self.cursor.reflect(),
                }
                k -= 1;
            }
        } else {
            self.cursor.reflect()
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
