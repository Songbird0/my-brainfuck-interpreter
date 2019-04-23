use std::io::{self, Read};

use crate::parser::ast::Interpreter;


/// Prints the cell content to the output.
///
/// ## Attention
///
/// Be sure that your cell doesn't contain a negative value.
/// You could have an invalid character otherwise.
pub fn print(interpreter: Interpreter) -> Interpreter {
    let current_cell = interpreter.ram[interpreter.ram_ptr];
    debug_assert!(current_cell >= 0, "`current_cell` is negative");
    let current_byte: u8 = current_cell as u8;
    let current_char: char = current_byte as char;
    print!("{}", current_char);
    interpreter
}

/// Takes a number (as string) from stdin.
///
/// ## Attention
///
/// The interpreter will consume the number 'as is' not
/// byte by byte.
///
/// ### Example:
///
/// ```text
/// 1500
/// ```
///
/// It will be consumed as `1500`, not `1`, `5`, `0`, `0`.
pub fn feed(interpreter: Interpreter) -> Interpreter {
    let mut raw_input: String = String::new();

    io::stdin().read_line(&mut raw_input).expect("Something went wrong while the stdin reading:");

    let value: Result<i32, String> = raw_input.trim().parse().map_err(|e| {
        eprintln!("An error occurred while parsing your integer.\
        See below for more:\n{:#?}\n\
        The current cell value is untouched.", e);
        "parsing_error".to_owned()
    });

    if let Ok(number) = value {
        let mut interpreter = interpreter;
        let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];

        *current_cell = number;

        return interpreter;
    }

    interpreter
}

#[test]
fn print_single_token() {

    let interpreter: Interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: ".".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };

    let interpreter = print(interpreter);

    let mut buffer = String::new();
    buffer.push((interpreter.ram[interpreter.ram_ptr] as u8) as char);
    assert_eq!(buffer, "\u{0}");
}

#[test]
fn print_the_character_b() {
     // "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
    let interpreter: Interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };

    let interpreter = do_while_is_lower_than_99(interpreter);

    let mut buffer = String::new();
    buffer.push((interpreter.ram[interpreter.ram_ptr] as u8) as char);
    assert_eq!(buffer, "b");

    fn do_while_is_lower_than_99(interpreter: Interpreter) -> Interpreter {
        let mut interpreter = interpreter;
        let current_cell = &mut interpreter.ram[interpreter.ram_ptr];
        while *current_cell < 98 {
            *current_cell += 1;
            let cc: &i32 = &current_cell;
            std::dbg!(cc);
        }
//            let b_unicode_number = 0x62; // 98
        interpreter
    }
}
