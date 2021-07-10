use crate::tokens::Token;
use crate::Tape;

use std::io;
use std::io::{Read, Write};

pub fn interpret(input: Vec<Token>, mut tape: Tape) {
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            Token::Add(n) => tape.memory[tape.ptr] += n,
            Token::Substract(n) => tape.memory[tape.ptr] -= n,

            Token::MovePtrRight(n) => tape.ptr += n,
            Token::MovePtrLeft(n) => tape.ptr -= n,

            Token::LoopStart { loop_end_addr } => {
                if tape.memory[tape.ptr] == 0 {
                    i = loop_end_addr;
                }
            }

            Token::LoopEnd { loop_start_addr } => {
                if tape.memory[tape.ptr] != 0 {
                    i = loop_start_addr;
                }
            }

            Token::PrintChar => {
                print!("{}", tape.memory[tape.ptr] as char);
                io::stdout().flush().unwrap();
            }

            Token::ReadChar => {
                tape.memory[tape.ptr] = io::stdin().bytes().next().unwrap().unwrap() as u8
            }

            Token::SetToZero => tape.memory[tape.ptr] = 0,

            _ => unreachable!(),
        }

        i += 1;
    }
}
