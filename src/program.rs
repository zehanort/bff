use std::default::Default;
use std::{
    fs::File,
    io,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use crate::cursor::Cursor;
use crate::direction::Direction;

const BEFUNGE_93_BOUNDS: (i32, i32) = (80, 25);

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
        let width = code.iter().map(|line| line.len()).max().unwrap() as i32;
        let height = code.len() as i32;
        // the `format!` adds padding to the right to make sure that
        // the grid has all the columns needed
        let grid: Vec<Vec<char>> = code
            .iter()
            .map(|line| {
                format!("{:width$}", line, width = width as usize)
                    .to_owned()
                    .chars()
                    .collect()
            })
            .collect();
        Program {
            grid,
            bounds: (width, height),
            ..Default::default()
        }
    }
}

impl From<PathBuf> for Program {
    /// Constructs a `Program` from the contents of a Befunge source code file.
    fn from(filename: PathBuf) -> Program {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        let code = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect::<Vec<String>>();

        Program::from(code)
    }
}

impl Program {
    /// Helper function that converts an `i32` to a `char`.
    fn i32_to_char(i: i32) -> char {
        let c: u8 = i.try_into().unwrap();
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
        } else if position.0 >= BEFUNGE_93_BOUNDS.0 || position.1 >= BEFUNGE_93_BOUNDS.1 {
            panic!(
                "\"get\" attempt outside the Befunge 93 bounding box -> (80x25), attempted ({}x{})",
                position.0, position.1
            );
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
    Moves the cursor one step on the `cursor.direction`
    and takes care of any possible wrap-around, effectively updating
    the `cursor.position`.

    Note that this is a method of `Program` instead of `Cursor` because
    it is more reasonable for the bounds checking logic
    to be implemented in the `Program` component.
    */
    fn move_cursor(&mut self) {
        let (x, y) = self.cursor.position();
        match self.cursor.direction() {
            Direction::East => self
                .cursor
                .set_position(if x + 1 == self.bounds.0 { 0 } else { x + 1 }, y),
            Direction::South => self
                .cursor
                .set_position(x, if y + 1 == self.bounds.1 { 0 } else { y + 1 }),
            Direction::West => self.cursor.set_position(
                if x - 1 == -1 {
                    self.bounds.0 - 1
                } else {
                    x - 1
                },
                y,
            ),
            Direction::North => self.cursor.set_position(
                x,
                if y - 1 == -1 {
                    self.bounds.1 - 1
                } else {
                    y - 1
                },
            ),
        }
    }

    /**
    Executes the cell on which the `cursor` lies.
    For the full Befunge 93 instruction list see
    [here](https://en.wikipedia.org/wiki/Befunge#Befunge-93_instruction_list).

    Returns `true` if the `@` cell was executed
    i.e., the program terminated, and `false` otherwise.
    */
    fn execute_current_cell(&mut self) -> bool {
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
                '>' => self.cursor.set_direction(Direction::East),
                // Start moving left
                '<' => self.cursor.set_direction(Direction::West),
                // Start moving up
                '^' => self.cursor.set_direction(Direction::North),
                // Start moving down
                'v' => self.cursor.set_direction(Direction::South),
                // Start moving in a random cardinal direction
                '?' => self.cursor.set_direction(rand::random()),
                // Pop a value; move right if value=0, left otherwise
                '_' => {
                    let a = self.pop();
                    self.cursor.set_direction(if a == 0 {
                        Direction::East
                    } else {
                        Direction::West
                    })
                }
                // Pop a value; move down if value=0, up otherwise
                '|' => {
                    let a = self.pop();
                    self.cursor.set_direction(if a == 0 {
                        Direction::South
                    } else {
                        Direction::North
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
                    io::stdout().flush().unwrap();
                }
                // Pop value and output as ASCII character
                ',' => {
                    print!("{}", Program::i32_to_char(self.pop()));
                    io::stdout().flush().unwrap();
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
                        .expect("failed to read from stdin");

                    self.push(input_text.trim().parse::<i32>().unwrap());
                }
                // Ask user for a character and push its ASCII value
                '~' => {
                    let c = std::io::stdin()
                        .bytes()
                        .next()
                        .and_then(|result| result.ok())
                        .map(|byte| byte as i32)
                        .unwrap();

                    self.push(c);
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
        program_terminated
    }

    /// Runs the Befunge program.
    pub fn run(&mut self) {
        loop {
            let program_terminated = self.execute_current_cell();
            if program_terminated {
                break;
            }
        }
    }
}
