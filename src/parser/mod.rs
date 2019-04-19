use std::fmt;

use nom::{is_alphanumeric, AsBytes};

pub enum ReservedWord {
    /// `>`
    GreaterThanSign,
    /// `<`
    LowerThanSign,
    /// `+`
    PlusSign,
    /// `-`
    MinusSign,
    /// `[`
    LeftSquareBracket,
    /// `]`
    RightSquareBracket,
    /// A simple point `.`.
    FullStop,
    /// `,`
    Comma,
}


impl ReservedWord {
    pub fn to_char(&self) -> char {
        match self {
            ReservedWord::GreaterThanSign => '>',
            ReservedWord::LowerThanSign => '<',
            ReservedWord::PlusSign => '+',
            ReservedWord::MinusSign => '-',
            ReservedWord::LeftSquareBracket => '[',
            ReservedWord::RightSquareBracket => ']',
            ReservedWord::FullStop => '.',
            ReservedWord::Comma => ',',
        }
    }
}

impl fmt::Display for ReservedWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

struct Interpreter<'program> {
    /// The program data.
    ram: [i32; 30_000],
    /// A pointer to access to the program data.
    ram_ptr: usize,
    /// Our input (a file, for example).
    program: &'program [u8],
    /// A pointer to read each character.
    program_ptr: usize,
    /// To resolve the loops.
    stack: Vec<usize>
}


/// "Moves" the pointer to the right.
fn right(interpreter: Interpreter) -> Interpreter {
    let mut interpreter = interpreter;
    interpreter.ram_ptr += 1;
    interpreter
}

#[test]
fn right_a_simple_token() {
    // note: the entire program is reduced
    // to a simple token (i.e. `>`) for
    // the example only.
    // Actually, the `right()` parser shouldn't
    // have access to `program` directly but
    // a sub-stream instead.
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: ">".as_bytes(),
        program_ptr: 0,
        stack: vec![],
    };

    let mut interpreter = right(interpreter);
    assert_eq!(interpreter.ram_ptr, 1);
}

fn left(interpreter: Interpreter) -> Interpreter {
    if interpreter.ram_ptr == 0 {
        // We cannot decrement `ptr` anymore.
        panic!("Your pointer is out of bound (negative)");
    }
    let mut interpreter = interpreter;
    interpreter.ram_ptr -= 1;
    interpreter
}

#[test]
#[should_panic="Your pointer is out of bound (negative)"]
fn left_a_simple_token() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "<".as_bytes(),
        program_ptr: 0,
        stack: vec![],
    };
    left(interpreter);
}

#[test]
fn left_go_to_right_and_go_back_to_left() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "><".as_bytes(),
        program_ptr: 0,
        stack: vec![],
    };

    let mut interpreter = right(interpreter);
    assert_eq!(interpreter.ram_ptr, 1);

    let mut interpreter = left(interpreter);
    assert_eq!(interpreter.ram_ptr, 0);
}


fn increment(interpreter: Interpreter) -> Interpreter {
    let mut interpreter = interpreter;
    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell += 1;
    interpreter
}

#[test]
fn increment_single_token() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "+".as_bytes(),
        program_ptr: 0,
        stack: vec![],
    };

    let mut interpreter = increment(interpreter);

    let current_cell: i32 = interpreter.ram[interpreter.ram_ptr];
    assert_eq!(current_cell, 1);
}

fn decrement(interpreter: Interpreter) -> Interpreter {
    let mut interpreter = interpreter;

    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell -= 1;
    interpreter
}

#[test]
fn decrement_single_token() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "-".as_bytes(),
        program_ptr: 0,
        stack: vec![],
    };

    let mut interpreter = decrement(interpreter);

    let current_cell: i32 = interpreter.ram[interpreter.ram_ptr];

    assert_eq!(current_cell, -1);
}

fn loop_beginning(interpreter: Interpreter) -> Interpreter {
    let current_cell = interpreter.ram[interpreter.ram_ptr];
    let mut interpreter = interpreter;
    // false
    if current_cell == 0 {
        // A bracket is already opened.
        let mut opened_bracket_counter = 1;
        while opened_bracket_counter > 0 {
            interpreter.program_ptr = {
                interpreter.program_ptr += 1;
                debug_assert!(interpreter.program_ptr < interpreter.program.len());
                let current_character = interpreter.program[interpreter.program_ptr] as char;
                if current_character == '[' {
                    opened_bracket_counter += 1;
                }
                else if current_character == ']' {
                    opened_bracket_counter -= 1;
                }
                let program_ptr: &usize = &interpreter.program_ptr;
                std::dbg!(interpreter.program_ptr);
                interpreter.program_ptr
            };
        }
        return interpreter;
    }
    // true
    interpreter.stack.push(interpreter.program_ptr);
    interpreter
}

#[test]
fn loop_beginning_empty_loop_and_cell_equals_zero() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "[]".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };
    let mut interpreter = loop_beginning(interpreter);
    assert_eq!(interpreter.program_ptr, interpreter.program.len() - 1);
    let current_cell = interpreter.ram[interpreter.ram_ptr];
    assert_eq!(current_cell, 0);
}

#[test]
fn loop_beginning_3_empty_nested_loop_1_level_of_imbrication() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "[[][]]".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };
    let mut interpreter = loop_beginning(interpreter);
    assert_eq!(interpreter.program_ptr, interpreter.program.len() - 1);
    let current_cell = interpreter.ram[interpreter.ram_ptr];
    assert_eq!(current_cell, 0);
}

#[test]
fn loop_beginning_3_empty_nested_loop_2_level_of_imbrication() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "[[[]]]".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };
    let mut interpreter = loop_beginning(interpreter);
    assert_eq!(interpreter.program_ptr, interpreter.program.len() - 1);
    let current_cell = interpreter.ram[interpreter.ram_ptr];
    assert_eq!(current_cell, 0);
}

/*#[test]
fn loop_beginning_1_active_loop_and_1_empty_loop_1_level_of_imbrication() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "+[[]]".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };
}*/

fn loop_ending(interpreter: Interpreter) -> Interpreter {
    let mut interpreter = interpreter;
    let current_cell = interpreter.ram[interpreter.ram_ptr];

    interpreter.program_ptr = if current_cell != 0 {
        let topmost_position = interpreter.stack.pop();
        debug_assert_eq!(topmost_position.is_some(), true);
        let topmost_position = topmost_position.unwrap();
        topmost_position
    }
    else {
        interpreter.stack.pop();
        interpreter.program_ptr
    };

    interpreter

    // "++[+++[---][>>++]]"
    // [2, 6,
}

#[test]
fn loop_ending_integration_with_loop_beginning_single_loop() {
    let interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "+[]".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };

    let mut interpreter = increment(interpreter);
    let current_cell = interpreter.ram[interpreter.ram_ptr];
    assert_eq!(current_cell, 1);
    // We do the work of `run()` for test purpose only.
    interpreter.program_ptr += 1;

    let mut interpreter = loop_beginning(interpreter);
    assert_eq!(interpreter.program_ptr, 1);
    assert_eq!(interpreter.stack, vec![1]);

    let interpreter = loop_ending(interpreter);
    assert_eq!(interpreter.program_ptr, 1);
    assert_eq!(interpreter.stack.is_empty(), true);
}

fn print(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }
fn feed(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }

/// Runs the interpreter.
fn run(input: &'static [u8]) -> Result<(), String> {
    unimplemented!()
}