use crate::opcodes::OpKind;
use crate::Tape;

use std::io;
use std::io::{Read, Write};

pub fn interpret(input: Vec<OpKind>, mut tape: Tape) {
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            OpKind::Add(n) => tape.memory[tape.ptr] += n,
            OpKind::Substract(n) => tape.memory[tape.ptr] -= n,

            OpKind::MovePtrRight(n) => tape.ptr += n,
            OpKind::MovePtrLeft(n) => tape.ptr -= n,

            OpKind::LoopStart { loop_end_addr } => {
                if tape.memory[tape.ptr] == 0 {
                    i = loop_end_addr;
                }
            }

            OpKind::LoopEnd { loop_start_addr } => {
                if tape.memory[tape.ptr] != 0 {
                    i = loop_start_addr;
                }
            }

            OpKind::PrintChar => {
                print!("{}", tape.memory[tape.ptr] as char);
                io::stdout().flush().unwrap();
            }

            OpKind::ReadChar => {
                tape.memory[tape.ptr] = io::stdin().bytes().next().unwrap().unwrap() as u8
            }

            OpKind::SetToZero => tape.memory[tape.ptr] = 0,

            _ => unreachable!(),
        }

        i += 1;
    }
}
