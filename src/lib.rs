mod clap;
pub mod interpreter;
pub mod optimizers;

use std::fs;

pub struct Tape {
    pub memory: [u8; 180_000],
    pub stack: Vec<usize>,
    pub ptr: usize,
}

pub fn run(tape: Tape) {
    let file = clap::get_file();

    let program = optimizers::cleanup_input(fs::read_to_string(file).expect("Invalid filename"));

    interpreter::interpret(
        &optimizers::optimize_brainfuck(program.as_bytes().to_vec())[..],
        tape,
    );
}
