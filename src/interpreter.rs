use crate::instructions::Opcode;
use crate::MEMORY_SIZE;

use std::io;
use std::io::{Read, Write};

pub struct Tape {
    pub memory: [u8; MEMORY_SIZE],
    pub ptr: usize,
}

pub fn interpret(input: Vec<Opcode>) {
    let mut tape = Tape {
        memory: [0; MEMORY_SIZE],
        ptr: MEMORY_SIZE / 2,
    };

    let mut i = 0;

    while i < input.len() {
        match input[i] {
            Opcode::Add(n) => tape.memory[tape.ptr] += n,
            Opcode::Substract(n) => tape.memory[tape.ptr] -= n,

            Opcode::MovePtrRight(n) => tape.ptr += n,
            Opcode::MovePtrLeft(n) => tape.ptr -= n,

            Opcode::LoopStart { loop_end_addr } => {
                if tape.memory[tape.ptr] == 0 {
                    i = loop_end_addr;
                }
            }

            Opcode::LoopEnd { loop_start_addr } => {
                if tape.memory[tape.ptr] != 0 {
                    i = loop_start_addr;
                }
            }

            Opcode::PrintChar => {
                print!("{}", tape.memory[tape.ptr] as char);
                io::stdout().flush().unwrap();
            }

            Opcode::ReadChar => {
                tape.memory[tape.ptr] = io::stdin().bytes().next().unwrap().unwrap() as u8
            }

            Opcode::SetToZero => tape.memory[tape.ptr] = 0,

            _ => unreachable!(),
        }

        i += 1;
    }
}
