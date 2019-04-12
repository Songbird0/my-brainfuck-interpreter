use std::fmt;

use nom::is_alphanumeric;

/// "horizontal" whitespaces are 0x20 and 0x09.
fn is_horizontal_whitespace(chr: u8) -> bool {
    let tmp = chr as char;
    tmp == ' ' || tmp == '\t'
}

/// A line is a string of alphanumeric (and non-alphanumeric like
/// whitespace, except `\n` and `\r`) characters optionally ended by `\n` or `\r\n`.
///
/// A line may be a simple `\n` too.
fn lines(input: &[u8]) -> nom::IResult<&[u8], Vec<Line<T>>, u32> {
    // What is a line?
    let (i1, line_content) = try_parse!(
      opt!(
        alt!(
          take_while!(is_alpanumeric) |
          take_while!(is_horizontal_whitespace)
        )
      )
    );
    std::dbg!(i1);
    std::dbg!(line_content);
    let (i2, line_separator) = try_parse!(
      opt!(
        alt!(
          complete!(tag!("\n")) | complete!(tag!("\r\n"))
        )
      )
    );
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

