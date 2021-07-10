pub mod interpreter;
pub mod tokenizer;
pub mod tokens;

use std::fs;
pub const MEMORY_SIZE: usize = 180_000;

pub struct Tape {
    pub memory: [u8; MEMORY_SIZE],
    pub ptr: usize,
}

pub fn run(tape: Tape, file: String) {
    let input = tokenizer::tokenize(fs::read_to_string(file).expect("Invalid filename"));
    interpreter::interpret(input, tape);
}
