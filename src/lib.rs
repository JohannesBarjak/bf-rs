pub mod interpreter;
pub mod opcodes;
pub mod parser;

use std::fs;
pub const MEMORY_SIZE: usize = 180_000;

pub struct Tape {
    pub memory: [u8; MEMORY_SIZE],
    pub ptr: usize,
}

pub fn run(tape: Tape, file: String) {
    let input = parser::parse_input(fs::read_to_string(file).expect("Invalid filename"));
    interpreter::interpret(input, tape);
}
