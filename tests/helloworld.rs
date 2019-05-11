#[macro_use]
extern crate brainfuck_interpreter;

use brainfuck_interpreter::parser::ast::Interpreter;

#[test]
fn helloworld_case() {
    let interpreter: Interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        // 15 = > ---------------
        // stack: [8, 14]
        program: "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };

    interpreter.run();
}

#[test]
fn helloworld_debug_trait() {
    let interpreter: Interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };

    ::brainfuck_interpreter::ldbg!(interpreter);
}