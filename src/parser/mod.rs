use std::fmt;

use nom::is_alphanumeric;

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
fn right<'program>(interpreter: &mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    // The stream result will be useful later (future version)
    // for error report tracking.
    let (i, o) = try_parse!(input, tag!(">"));
    interpreter.ram_ptr += 1;
    Ok((i, o))
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
    assert_eq!(right(&mut interpreter, ">".as_bytes()), Ok(("".as_bytes(), ">".as_bytes())));
    assert_eq!(interpreter.ram_ptr, 1);
}

fn left<'program, 'func>(interpreter: &'func mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    if interpreter.ram_ptr == 0 {
        // We cannot decrement `ptr` anymore.
        panic!("Your pointer is out of bound (negative)");
    }
    let (i, o) = try_parse!(input, tag!("<"));
    interpreter.ram_ptr -= 1;
    Ok((i, o))
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
    left(&mut interpreter, "<".as_bytes());
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

    let (i1, o1) = right(&mut interpreter, "><".as_bytes()).expect("Something went wrong:");

    assert_eq!(i1, "<".as_bytes());
    assert_eq!(interpreter.ram_ptr, 1);

    assert_eq!(left(&mut interpreter, i1), Ok(("".as_bytes(), "<".as_bytes())));
    assert_eq!(interpreter.ram_ptr, 0);
}


fn increment<'program, 'func>(interpreter: &'func mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    let (i, o) = try_parse!(input, tag!("+"));

    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell += 1;

    Ok((i, o))
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

    assert_eq!(increment(&mut interpreter, "+".as_bytes()), Ok(("".as_bytes(), "+".as_bytes())));

    let current_cell: i32 = interpreter.ram[interpreter.ram_ptr];

    assert_eq!(current_cell, 1);
}

fn decrement<'program, 'func>(interpreter: &'func mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    let (i, o) = try_parse!(input, tag!("-"));

    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell -= 1;

    Ok((i, o))
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

    assert_eq!(decrement(&mut interpreter, "-".as_bytes()), Ok(("".as_bytes(), "-".as_bytes())));

    let current_cell: i32 = interpreter.ram[interpreter.ram_ptr];

    assert_eq!(current_cell, -1);
}

fn reach_the_matching_bracket<'program, 'func>(interpreter: &'func  mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    unimplemented!()
}

fn loop_beginning(interpreter: &mut Interpreter) {
    let current_cell = interpreter.ram[interpreter.ram_ptr];

    if current_cell == 0 {
        let mut opened_bracket_counter = 1;
        while opened_bracket_counter > 0 {
            let current_character = interpreter.program[interpreter.program_ptr] as char;
            if current_character == '[' {
                opened_bracket_counter += 1;
            }
            else if current_character == ']' {
                opened_bracket_counter -= 1;
            }
            std::dbg!(interpreter.program_ptr += 1);
        }
    }
    else {
        interpreter.stack.push(interpreter.program_ptr);
    }
}

fn loop_ending(interpreter: &mut Interpreter, input: &[u8]) {
    let current_cell = interpreter.ram[interpreter.ram_ptr];

    if current_cell > 0 {
        let topmost_position = interpreter.stack[0];
        interpreter.program_ptr = topmost_position;
    }
    else {
        interpreter.stack.pop();
    }
}

fn loop_<'program, 'func>(interpreter: &'func mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {

    unimplemented!()
}
fn print(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }
fn feed(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }

/// Runs the interpreter.
fn run(input: &'static [u8]) -> Result<(), String> {
    unimplemented!()
}