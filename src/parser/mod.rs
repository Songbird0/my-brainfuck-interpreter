use std::fmt;
use crate::parser::ast::Row;

pub mod ast;


/// Get all rows found by the parser.
fn rows(input: &[u8]) -> nom::IResult<&[u8], Vec<ast::Row>, u32> {
    // What is a row?


    unimplemented!()
}

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

impl fmt::Display for ReservedWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
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

fn right(input: &[u8]) -> nom::IResult<&[u8], char, u32> {
    unimplemented!()
}
fn left(input: &[u8]) -> nom::IResult<&[u8], char, u32> {
    unimplemented!()
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

