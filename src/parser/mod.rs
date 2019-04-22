pub mod ast;

use std::fmt;

use nom::{is_alphanumeric, AsBytes};



fn increment(interpreter: ast::Interpreter) -> ast::Interpreter {
    let mut interpreter = interpreter;
    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell += 1;
    interpreter
}

#[test]
fn increment_single_token() {
    let mut interpreter = ast::Interpreter {
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

fn decrement(interpreter: ast::Interpreter) -> ast::Interpreter {
    let mut interpreter = interpreter;

    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell -= 1;
    interpreter
}

#[test]
fn decrement_single_token() {
    let mut interpreter = ast::Interpreter {
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

fn loop_beginning(interpreter: ast::Interpreter) -> ast::Interpreter {
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
    let mut interpreter = ast::Interpreter {
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
    let mut interpreter = ast::Interpreter {
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
    let mut interpreter = ast::Interpreter {
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
    let mut interpreter = ast::Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "+[[]]".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };
}*/

fn loop_ending(interpreter: ast::Interpreter) -> ast::Interpreter {
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
    let interpreter = ast::Interpreter {
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