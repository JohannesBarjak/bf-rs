use crate::instructions::Op;
use crate::MEMORY_SIZE;

use std::io;
use std::io::{Read, Write};

pub struct Tape {
    memory: [u8; MEMORY_SIZE],
    ptr: usize,
}

impl Tape {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            ptr: MEMORY_SIZE / 2,
        }
    }
}

pub fn interpret(instructions: &[Op], tape: &mut Tape) {
    for instruction in instructions {
        match instruction {
            Op::Add(n) => tape.memory[tape.ptr] = tape.memory[tape.ptr].wrapping_add(*n),
            Op::Move(n) => tape.ptr += *n as usize,

            Op::Loop(body) => {
                while tape.memory[tape.ptr] != 0 {
                    interpret(body, tape);
                }
            }

            Op::PrintChar => print!("{}", tape.memory[tape.ptr] as char),

            Op::ReadChar => {
                io::stdout().flush().unwrap();
                tape.memory[tape.ptr] = io::stdin().bytes().next().unwrap().unwrap() as u8;
            }

            Op::Clear => tape.memory[tape.ptr] = 0,

            Op::Mul(offset, mul) => {
                let copy_index = tape.ptr + *offset as usize;
                tape.memory[copy_index] =
                    tape.memory[copy_index].wrapping_add(tape.memory[tape.ptr] * *mul);
            }

            Op::Set(n) => tape.memory[tape.ptr] = *n,

            Op::Shift(n) => {
                while tape.memory[tape.ptr] != 0 {
                    tape.ptr += *n as usize;
                }
            }
        }
    }
}
