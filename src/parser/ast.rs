/// A column is a "column vector" (https://en.wikipedia.org/wiki/Row_and_column_vectors)
/// composed of characters.
#[derive(Debug)]
pub struct Column {
    number: u32
}

/// A row is a string of alphanumeric (and non-alphanumeric like
/// whitespace, except `\n` and `\r`) characters optionally ended by `\n` or `\r\n`.
///
/// A row may be a simple `\n` too.
#[derive(Debug)]
pub  struct Row {
    content: &'static [u8],
    number: u32
}

/// The intersection of a row and column number.
#[derive(Debug)]
pub struct Position {
    row: Row,
    column: Column
}

// FIXME Implement `get_row_bytes()` and `get_at()` for `Position`.

/// An abstract representation of the brainfuck source file.u
pub struct BrainfuckSourceFile {
    positions: Vec<Position>
}


struct SourceCrawler {
    rows: Vec<Row>
}

impl SourceCrawler {

    pub fn new() -> Self {
        SourceCrawler {
            rows: vec!()
        }
    }

    pub fn get_rows(&self, input: &'static [u8]) -> Result<(), String> {
        let position = 0;
        let mut current_stream: &[u8] = input;
        while position < input.len() {
            let result = self.get_row(input);
            if let Ok((i, o)) = result {
                // Finish to implement it
            }
        }
        Ok(())
    }

    /// "horizontal" whitespaces are 0x20 and 0x09.
    fn get_row(&self, input: &'static [u8]) -> nom::IResult<&'static [u8], Row, u32> {
        let row_content_wrapper: nom::IResult<&[u8], Option<&[u8]>, u32> = opt!(input,
          alt!(
            take_while!(is_other_than_new_line) |
            take_while!(is_horizontal_whitespace)
          )
        );
        let row_separator_wrapper: nom::IResult<&[u8], Option<&[u8]>, u32> = opt!(input,
          alt!(
            complete!(tag!("\n")) | complete!(tag!("\r\n"))
          )
        );
        match &row_content_wrapper {
            Ok((i, o)) => {
                let (i2, row_separator_wrapper) = match &row_separator_wrapper {
                    Ok((i, opt)) => (i, opt),
                    _ => unreachable!()
                };
                match o {
                    None => if row_separator_wrapper.is_none() {
                        // input is empty?
                        std::dbg!(row_content_wrapper);
                        std::dbg!(row_separator_wrapper);
                        panic!("Cannot consume an empty stream.");
                    } else {
                        // the row contains '\n' or '\r\n' only.
                        let row_separator: &[u8] = row_separator_wrapper.unwrap();
                        let tmp = Row {
                            content: row_separator,
                            number: (self.rows.len() + 1) as u32
                        };
                        Ok((*i2, tmp))
                    },
                    Some(bytes) => {
                        // input is containing other than '\n' or '\r\n'.
                        let tmp = Row {
                            content: bytes,
                            number: (self.rows.len() + 1) as u32
                        };
                        Ok((*i2, tmp))
                    }
                }
            }
            _ => unreachable!()
        }
    }
}

fn is_horizontal_whitespace(chr: u8) -> bool {
    let tmp = chr as char;
    tmp == ' ' || tmp == '\t'
}

fn is_other_than_new_line(chr: u8) -> bool {
    let current_char = chr as char;
    nom::is_alphanumeric(chr) || (current_char) != '\n' || (current_char) != '\r'
}
