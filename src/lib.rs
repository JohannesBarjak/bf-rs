mod arg_parser;
mod interpreter;
mod optimizers;

use std::fs;

pub struct Tape {
    pub cell: [u8; 180_000],
    pub stack: Vec<usize>,
    pub ptr: usize,
}

pub fn run(tape: Tape, args: Vec<String>) {
    let input = args.get(1).expect("Expected one argument");
    arg_parser::process_args(&input);

    let input = optimizers::cleanup_input(fs::read_to_string(input).expect("Invalid filename"));

    interpreter::interpret(
        &optimizers::optimize_brainfuck(input.as_bytes().to_vec())[..],
        tape,
    );
}
