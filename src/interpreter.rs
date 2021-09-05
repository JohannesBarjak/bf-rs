use crate::instructions::Opcode;
use crate::MEMORY_SIZE;

use std::io;
use std::io::{Read, Write};

pub struct Tape {
    pub memory: [u8; MEMORY_SIZE],
    pub ptr: usize,
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

pub fn interpret(instructions: Vec<Opcode>, tape: &mut Tape) {
    let mut i = 0;

    while i < instructions.len() {
        match instructions[i] {
            Opcode::Add(n) => tape.memory[tape.ptr] = tape.memory[tape.ptr].wrapping_add(n as u8),
            Opcode::Move(n) => tape.ptr = tape.ptr.wrapping_add(n as usize),

            Opcode::LoopStart(loop_end_addr) => {
                if tape.memory[tape.ptr] == 0 {
                    i = loop_end_addr;
                }
            }

            Opcode::LoopEnd(loop_start_addr) => {
                if tape.memory[tape.ptr] != 0 {
                    i = loop_start_addr;
                }
            }

            Opcode::PrintChar => print!("{}", tape.memory[tape.ptr] as char),

            Opcode::ReadChar => {
                io::stdout().flush().unwrap();
                tape.memory[tape.ptr] = io::stdin().bytes().next().unwrap().unwrap() as u8;
            }

            Opcode::Clear => tape.memory[tape.ptr] = 0,
        }

        i += 1;
    }
}
