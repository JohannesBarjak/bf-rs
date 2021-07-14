pub mod instructions;
pub mod interpreter;
pub mod parser;
pub mod tape;

use crate::tape::Tape;
use std::fs;

pub fn run(tape: Tape, file: String) {
    let input = parser::parse(fs::read_to_string(file).expect("Invalid filename"));
    interpreter::interpret(input, tape);
}
