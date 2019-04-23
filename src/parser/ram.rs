use crate::parser::ast;

/// "Moves" the pointer to the right.
pub fn right(interpreter: ast::Interpreter) -> ast::Interpreter {
    let mut interpreter = interpreter;
    interpreter.ram_ptr += 1;
    interpreter
}

pub fn left(interpreter: ast::Interpreter) -> ast::Interpreter {
    if interpreter.ram_ptr == 0 {
        // We cannot decrement `ptr` anymore.
        panic!("Your pointer is out of bound (negative)");
    }
    let mut interpreter = interpreter;
    interpreter.ram_ptr -= 1;
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
    let mut interpreter = ast::Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: ">".as_bytes(),
        program_ptr: 0,
        stack: vec![],
    };

    let mut interpreter = right(interpreter);
    assert_eq!(interpreter.ram_ptr, 1);
}

#[test]
#[should_panic = "Your pointer is out of bound (negative)"]
fn left_a_simple_token() {
    let mut interpreter = ast::Interpreter {
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
    let mut interpreter = ast::Interpreter {
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

pub fn increment(interpreter: ast::Interpreter) -> ast::Interpreter {
    let mut interpreter = interpreter;
    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell += 1;
    interpreter
}

pub fn decrement(interpreter: ast::Interpreter) -> ast::Interpreter {
    let mut interpreter = interpreter;

    let current_cell: &mut i32 = &mut interpreter.ram[interpreter.ram_ptr];
    *current_cell -= 1;
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
