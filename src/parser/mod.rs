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
    ram: [u8; 30_000],
    program: &'program [u8],
    ptr: u32
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
        program: ">".as_bytes(),
        ptr: 0
    };
    assert_eq!(right(&mut interpreter, ">".as_bytes()), Ok(("".as_bytes(), ">".as_bytes())));
    assert_eq!(interpreter.ptr, 1);
}

/// "Moves" the pointer to the right.
fn right<'program>(interpreter: &mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    // The stream result will be useful later (future version)
    // for error report tracking.
    let (i, o) = try_parse!(input, tag!(">"));
    interpreter.ptr += 1;
    Ok((i, o))
}

#[test]
#[should_panic="Your pointer is out of bound (negative)"]
fn left_a_simple_token() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        program: "<".as_bytes(),
        ptr: 0
    };
    left(&mut interpreter, "<".as_bytes());
}

#[test]
#[ignore]
fn left_go_to_right_and_go_back_to_left() {
    let mut interpreter = Interpreter {
        ram: [0; 30_000],
        program: "><".as_bytes(),
        ptr: 0
    };
    // FIXME The interpreter shouldn't panic and
    // FIXME `ptr` should be equal to 0
    unimplemented!()
}


fn left<'program>(interpreter: &'program mut Interpreter, input: &'program [u8]) -> nom::IResult<&'program [u8], &'program [u8], u32> {
    if interpreter.ptr == 0 {
        // We cannot decrement `ptr` anymore.
        panic!("Your pointer is out of bound (negative)");
    }
    let (i, o) = try_parse!(input, tag!("<"));
    interpreter.ptr -= 1;
    Ok((i, o))
}
fn increment(input: &[u8]) -> nom::IResult<&[u8], char, u32> {
    unimplemented!()
}
fn decrement(input: &[u8]) -> nom::IResult<&[u8], (), u32> {
    unimplemented!()
}
fn loop_(input: &[u8]) -> nom::IResult<&[u8], (), u32> {
    unimplemented!()
}
fn print(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }
fn feed(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }

/// Runs the interpreter.
fn run(input: &'static [u8]) -> Result<(), String> {
    unimplemented!()
}