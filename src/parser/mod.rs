pub mod ast;
pub mod ram;

use std::fmt;

use nom::{is_alphanumeric, AsBytes};


fn print(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }
fn feed(input: &[u8]) -> nom::IResult<&[u8], (), u32> { unimplemented!() }
