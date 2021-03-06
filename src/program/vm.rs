use super::{delta::Delta, fungetypes::FungeInteger, sysinfo::SystemInfoReporter, Program};
use anyhow::{bail, Context, Result};
use colour::e_yellow;
use std::io::{self, Read, Write};

macro_rules! warning {
    ($message:expr) => {
        e_yellow!("warning: ");
        eprintln!($message);
    };
}

impl<T: FungeInteger> Program<T> {
    /// Pushes `x` into the program stack.
    fn push(&mut self, x: T) {
        self.sstack.push_onto_toss(x);
    }

    /**
    Pops and returns a `T` from the program stack.
    */
    fn pop(&mut self) -> T {
        self.sstack.pop_from_toss()
    }

    /// A wrapper around the `clear_toss` method of the `sstack` struct.
    fn clear_toss(&mut self) {
        self.sstack.clear_toss();
    }

    /**
    Returns the `T` on the `position` coordinates of the program grid.

    If `position` is out of bounds, returns ` ` (space), without annoying
    the underlying `grid` struct.
    */
    fn get_cell(&self, position: (T, T)) -> T {
        if self.grid.out_of_bounds(position) {
            T::from(32).unwrap() // space
        } else {
            self.grid[position]
        }
    }

    /**
    Puts `c` on the `position` coordinates of the program grid.

    The underlying `grid` struct will resize the Funge-Space in case
    `position` is out of bounds.
    */
    fn put_cell(&mut self, position: (T, T), c: T) {
        self.grid[position] = c;
        if c == T::from(32).unwrap() {
            self.grid.shrink(position);
        }
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
        self.cursor.r#move(self.grid.get_bounds());
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
        while [' ', ';'].contains(&current_cell) {
            // skip `;` blocks
            if current_cell == ';' {
                loop {
                    self.move_cursor();
                    times_moved += 1;
                    current_cell = char::from_u32(
                        self.get_cell(self.cursor.position())
                            .to_u32()
                            .unwrap_or_default(),
                    )
                    .unwrap_or_default();
                    if current_cell == ';' {
                        break;
                    }
                }
            }
            self.move_cursor();
            times_moved += 1;
            current_cell = char::from_u32(
                self.get_cell(self.cursor.position())
                    .to_u32()
                    .unwrap_or_default(),
            )
            .unwrap_or_default();
        }

        // reset cursor
        self.cursor.reflect();
        for _ in 0..times_moved {
            self.move_cursor();
        }
        self.cursor.reflect();

        current_cell
    }

    /// Skips all consecutive spaces by moving the `cursor` appropriately.
    fn skip_spaces(&mut self) {
        loop {
            self.move_cursor();
            let new_x = self.get_cell(self.cursor.position());
            if char::from_u32(new_x.to_u32().unwrap_or_default()).unwrap_or_default() != ' ' {
                // get cursor one space back
                // because it will move again at the end of the method
                self.cursor.reflect();
                self.move_cursor();
                self.cursor.reflect();
                break;
            }
        }
    }

    fn build_fingerprint(&mut self) -> T {
        let count = self.pop();
        let mut fp = T::zero();
        let a = T::from(256).unwrap_or_default();
        for _ in 0..count.to_usize().unwrap_or_default() {
            fp *= a;
            fp += self.pop();
        }
        fp
    }

    /**
    Executes the cell on which the `cursor` lies.
    For the full Befunge 93 instruction list see
    [here](https://en.wikipedia.org/wiki/Befunge#Befunge-93_instruction_list).

    Returns `true` if the `@` cell was executed
    i.e., the program terminated, and `false` otherwise.
    */
    fn execute_current_cell(&mut self) -> Result<(bool, i32)> {
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
        let mut k = T::one();
        let mut program_terminated = false;
        let mut exit_code = 0;

        let x = self.get_cell(position);

        // special case: string mode ON
        if self.string_mode {
            match char::from_u32(x.to_u32().unwrap_or_default()) {
                Some('"') => {
                    self.toggle_string_mode();
                }
                Some(' ') => {
                    self.push(x);
                    self.skip_spaces();
                }
                _ => self.push(x),
            }
        } else if let Some(mut xchar) = char::from_u32(x.to_u32().unwrap_or_default()) {
            while k > T::zero() {
                match xchar {
                    // Push this decimal number on the stack
                    d if ('0'..='9').contains(&d) => {
                        // these unwraps can't fail; we are sure we have a digit
                        self.push(T::from(d.to_digit(10).unwrap()).unwrap());
                    }
                    // Push this hex number on the stack
                    d if ('a'..='f').contains(&d) => {
                        // these unwraps can't fail; we are sure we have a (hex) digit
                        self.push(T::from(d.to_digit(16).unwrap()).unwrap());
                    }
                    // Addition: Pop a and b, then push a+b
                    '+' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            a,
                            overflowing_add,
                            &b,
                            "An addition resulted in overflow."
                        );
                    }
                    // Subtraction: Pop a and b, then push b-a
                    '-' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            b,
                            overflowing_sub,
                            &a,
                            "A subtraction resulted in overflow."
                        );
                    }
                    // Multiplication: Pop a and b, then push a*b
                    '*' => {
                        let (a, b) = (self.pop(), self.pop());
                        push_with_overflow_check!(
                            a,
                            overflowing_mul,
                            &b,
                            "A multiplication resulted in overflow."
                        );
                    }
                    /*
                    Integer division: Pop a and b, then push b/a, rounded towards 0.
                    [SPEC] division by 0 returns 0
                    */
                    '/' => {
                        let (a, b) = (self.pop(), self.pop());
                        if a == T::zero() {
                            warning!("Division by 0 occured. Will return 0 as per the language specification.");
                            self.push(T::zero());
                        } else {
                            self.push(b / a);
                        }
                    }
                    // Modulo: Pop a and b, then push the remainder of the integer division of b/a.
                    '%' => {
                        let (a, b) = (self.pop(), self.pop());
                        if a == T::zero() {
                            warning!("Remainder with divisor of 0 occured. Will return 0 as per the language specification.");
                            self.push(T::zero());
                        } else {
                            self.push(b % a);
                        }
                    }
                    // Logical NOT: Pop a value. If the value is zero, push 1; otherwise, push zero.
                    '!' => {
                        let a = self.pop();
                        self.push(if a == T::zero() { T::one() } else { T::zero() })
                    }
                    // Greater than: Pop a and b, then push 1 if b>a, otherwise zero.
                    '`' => {
                        let (a, b) = (self.pop(), self.pop());
                        self.push(if b > a { T::one() } else { T::zero() })
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
                        self.cursor.set_delta(if a == T::zero() {
                            Delta::east()
                        } else {
                            Delta::west()
                        })
                    }
                    // Pop a value; move down if value=0, up otherwise
                    '|' => {
                        let a = self.pop();
                        self.cursor.set_delta(if a == T::zero() {
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
                        self.pop();
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
                        let c = self.pop().to_u32().unwrap_or_default();
                        print!("{}", char::from_u32(c).unwrap_or_default());
                        io::stdout()
                            .flush()
                            .context("Failed to write a character to stdout")?;
                    }
                    // Bridge: Skip next cell
                    '#' => self.move_cursor(),
                    // A "put" call (a way to store a value for later use).
                    // Pop y, x, and v, then change the character at position ((x,y) + storage offset) in the program
                    // to the character with ASCII value v
                    'p' => {
                        let (y, x, v) = (self.pop(), self.pop(), self.pop());
                        self.put_cell(self.cursor.translate_to_storage_position((x, y)), v);
                    }
                    // A "get" call (a way to retrieve data in storage).
                    // Pop y and x, then push ASCII value of the character at the position (position + storage offset) in the program
                    'g' => {
                        let (y, x) = (self.pop(), self.pop());
                        let c = self.get_cell(self.cursor.translate_to_storage_position((x, y)));
                        self.push(c);
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

                        let mut res: T = T::zero();
                        let mut discard_done = false;
                        let mut negative = 1;
                        for dchar in input_text.trim().chars() {
                            if let Some(d) = dchar.to_digit(10) {
                                if !discard_done {
                                    discard_done = true;
                                }
                                let (shifted_res, mul_overflowed) =
                                    res.overflowing_mul(&T::from(10).unwrap());
                                // u32 -> i32 is safe here, it is just a single digit
                                let (new_res, add_overflowed) =
                                    shifted_res.overflowing_add(&T::from(d).unwrap());
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
                            let (neg_res, underflowed) =
                                res.overflowing_mul(&T::from(negative).unwrap());
                            res = if underflowed {
                                neg_res / T::from(10).unwrap()
                            } else {
                                neg_res
                            };
                        }

                        self.push(res);
                    }
                    // Ask user for a character and push its ASCII value
                    '~' => {
                        if let Some(b) = std::io::stdin().bytes().next() {
                            let c = b.context("Failed while reading a character from stdin")?;
                            self.push(T::from(c).unwrap());
                        } else {
                            bail!("Failed to read character from stdin")
                        }
                    }
                    // End program
                    '@' => program_terminated = true,
                    // No-op. Do nothing and skip all consecutive spaces
                    ' ' => self.skip_spaces(),
                    // Turn left
                    '[' => self.cursor.turn_left(),
                    // Turn right
                    ']' => self.cursor.turn_right(),
                    // Jump over i.e., execute nothing until next ";"
                    ';' => loop {
                        self.move_cursor();
                        if char::from_u32(
                            self.get_cell(self.cursor.position())
                                .to_u32()
                                .unwrap_or_default(),
                        )
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
                        if k == T::zero() {
                            self.move_cursor(); // simply skip next instruction, like `#`
                        } else {
                            k += T::one(); // to counter the -= 1 at the end of the while loop
                            xchar = self.peek();
                        }
                    }
                    // Clear stack
                    'n' => self.clear_toss(),
                    // Compare two values and turn left/right
                    'w' => {
                        let (b, a) = (self.pop(), self.pop());
                        match a.cmp(&b) {
                            std::cmp::Ordering::Less => self.cursor.turn_left(),
                            std::cmp::Ordering::Greater => self.cursor.turn_right(),
                            std::cmp::Ordering::Equal => {}
                        }
                    }
                    // Fetch character: push ASCII of position + delta onto the stack
                    // and then jump over it
                    '\'' => {
                        self.push(self.get_cell(self.cursor.delta() + position));
                        self.move_cursor(); // skip c
                    }
                    // Store character: pop ASCII value and write it into position + delta
                    's' => {
                        let c = self.pop();
                        let write_pos = self.cursor.delta() + position;
                        self.put_cell(write_pos, c);
                        self.move_cursor(); // skip c
                    }
                    // (Actual) nop
                    'z' => {}
                    // Jump n spaces forward/backward
                    'j' => {
                        let mut n = self.pop();
                        let negative = n < T::zero();
                        if negative {
                            self.cursor.reflect();
                        }
                        n = n.abs();
                        while n > T::zero() {
                            self.move_cursor();
                            n -= T::one();
                        }
                        if negative {
                            self.cursor.reflect();
                        }
                    }
                    // Set delta to absolute vector value
                    'x' => {
                        let (dy, dx) = (self.pop(), self.pop());
                        self.cursor.set_delta_members((dx, dy));
                    }
                    // Begin block; see specification for details
                    '{' => {
                        let n = self.pop();
                        self.sstack.create_stack(n, self.cursor.storage_offset());
                        // update storage offset
                        self.cursor
                            .set_storage_offset(self.cursor.delta() + self.cursor.position());
                    }
                    // End block; see specification or the `end_block` method for details
                    '}' => {
                        let n = self.pop();
                        match self.sstack.destroy_stack(n) {
                            Some(so) => self.cursor.set_storage_offset(so),
                            None => self.cursor.reflect(),
                        }
                    }
                    // Stack under stack; transfer between TOSS and SOSS
                    'u' => {
                        let count = self.pop();
                        if self.sstack.transfer(count).is_none() {
                            self.cursor.reflect();
                        }
                    }
                    // System information retrieval
                    'y' => {
                        let query = self.pop();
                        let report = self.get_full_report();
                        let report_length = report.len();
                        if query > T::zero() {
                            let q = query.to_usize().unwrap_or_default();
                            if q <= report_length {
                                // a part of the report was requested
                                self.push(report[report_length - q]);
                            } else {
                                // a value outside of the report was requested
                                let dup = self.sstack.get(q - report_length - 1);
                                self.push(dup);
                            }
                        } else {
                            // the whole report was requested
                            for cell in report {
                                self.push(cell);
                            }
                        }
                    }
                    // Load semantics
                    '(' => {
                        let _fp = self.build_fingerprint();
                        // no fingerprints implemented for now
                        self.cursor.reflect();
                    }
                    // Unload semantics
                    ')' => {
                        let _fp = self.build_fingerprint();
                        // no fingerprints implemented for now
                        self.cursor.reflect();
                    }
                    // Terminate program with exit code
                    'q' => {
                        exit_code = self.pop().to_i32().unwrap_or_default();
                        program_terminated = true;
                    }
                    // Every other character
                    // Note that string mode is OFF here
                    // (we checked the "ON" case before the match statement)
                    // as per the standard, we will reflect
                    // (imitating the "r" instruction which will be added later...)
                    _ => self.cursor.reflect(),
                }
                k -= T::one();
            }
        } else {
            self.cursor.reflect()
        }

        self.move_cursor();
        Ok((program_terminated, exit_code))
    }

    /// Runs the Befunge program and returns an exit code
    pub fn run(&mut self) -> Result<i32> {
        loop {
            let (program_terminated, exit_code) =
                self.execute_current_cell().context("Runtime error")?;
            if program_terminated {
                return Ok(exit_code);
            }
        }
    }
}
