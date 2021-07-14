use crate::instructions::Opcode;
use crate::tape::Tape;

use std::io;
use std::io::{Read, Write};

pub fn interpret(input: Vec<Opcode>, mut tape: Tape) {
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
