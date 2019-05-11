use crate::parser::{io, program, ram};
use std::fmt::{self, Debug};

pub struct Interpreter<'program> {
    /// The program data.
    pub ram: [i32; 30_000],
    /// A pointer to access to the program data.
    pub ram_ptr: usize,
    /// Our input (a file, for example).
    pub program: &'program [u8],
    /// A pointer to read each character.
    pub program_ptr: usize,
    /// To resolve the loops.
    pub stack: Vec<usize>,
}

impl<'program> fmt::Debug for Interpreter<'program> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"
Interpreter {{
  ram: "#
        )?;
        self.ram[..].fmt(f);
        write!(
           f,
           r#",
  ram_ptr: {:?},
  program: "#,
            self.ram_ptr
        )?;
        self.program.fmt(f);
        write!(
            f,
            r#",
  program_ptr: {0:?},
  stack: {1:?}
}}"#,
            self.program_ptr,
            self.stack
        )
    }
}

impl Interpreter<'static> {

    pub fn run(self) {
        let mut interpreter = self;

        'parser: loop {

            let current_char: char = interpreter.program[interpreter.program_ptr] as char;
            interpreter = match &current_char {
                '>' => ram::right(interpreter),
                '<' => ram::left(interpreter),
                '+' => ram::increment(interpreter),
                '-' => ram::decrement(interpreter),
                '[' => program::loop_beginning(interpreter),
                ']' => program::loop_ending(interpreter),
                '.' => io::print(interpreter),
                ',' => io::feed(interpreter),
                default @ _ => panic!("Oops, unexpected token: {:#?}", default),
            };

            if current_char == ']' {
                continue 'parser;
            }
            else {
                interpreter.program_ptr += 1;
            }

            if interpreter.program_ptr >= interpreter.program.len() { break; }
        }
    }
}
