use std::io;
use std::io::{Read, Write};

use crate::instructions::Op;
use crate::MEMORY_SIZE;

pub struct Tape {
    memory: [u8; MEMORY_SIZE],
    ptr: usize,
}

impl Tape {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, byte: u8, offset: isize) {
        self.memory[self.ptr + offset as usize] += byte;
    }

    pub fn get(&self, offset: isize) -> u8 {
        self.memory[self.ptr + offset as usize]
    }

    pub fn set(&mut self, byte: u8, offset: isize) {
        self.memory[self.ptr + offset as usize] = byte;
    }

    pub fn mov(&mut self, offset: isize) {
        self.ptr += offset as usize;
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
            Op::Add(n, offset) => tape.add(*n, *offset),
            Op::Move(n) => tape.mov(*n),

            Op::Loop(body) => {
                while tape.get(0) != 0 {
                    interpret(body, tape);
                }
            }

            Op::PrintChar(offset) => print!("{}", tape.get(*offset) as char),

            Op::ReadChar(offset) => {
                io::stdout().flush().unwrap();
                tape.set(io::stdin().bytes().next().unwrap().unwrap() as u8, *offset);
            }

            Op::Clear(offset) => tape.set(0, *offset),

            Op::Mul(offset, mul) => tape.add(tape.get(0) * mul, *offset),
            Op::Set(n, offset) => tape.set(*n, *offset),

            Op::Shift(n) => {
                while tape.get(0) != 0 {
                    tape.mov(*n);
                }
            }
        }
    }
}
