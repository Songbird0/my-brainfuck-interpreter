extern crate brainfuck_interpreter;

use brainfuck_interpreter::parser::ast::Interpreter;

fn main() {
    let interpreter: Interpreter = Interpreter {
        ram: [0; 30_000],
        ram_ptr: 0,
        program: "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".as_bytes(),
        program_ptr: 0,
        stack: vec![]
    };

    interpreter.run();
}
