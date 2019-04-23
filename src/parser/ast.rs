use crate::parser::{io, program, ram};

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

impl Interpreter<'static> {

    pub fn run(self) {
        let mut interpreter = self;

        loop {

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

            if interpreter.program_ptr >= interpreter.program.len() { break; }
        }
    }
}
