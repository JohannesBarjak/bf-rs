use std::io;
use std::io::{Read, Write};

use crate::instructions::*;
use crate::MEMORY_SIZE;

use OffOp::*;
use Op::*;

pub struct Tape {
    memory: [u8; MEMORY_SIZE],
    ptr: u16,
}

impl Tape {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, byte: u8, offset: isize) {
        unsafe {
            *self
                .memory
                .get_unchecked_mut(self.ptr as usize + offset as usize) += byte;
        }
    }

    pub fn get(&self, offset: isize) -> u8 {
        unsafe {
            *self
                .memory
                .get_unchecked(self.ptr as usize + offset as usize)
        }
    }

    pub fn set(&mut self, byte: u8, offset: isize) {
        unsafe {
            *self
                .memory
                .get_unchecked_mut(self.ptr as usize + offset as usize) = byte;
        }
    }

    pub fn mov(&mut self, offset: isize) {
        self.ptr += offset as u16;
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
            ptr: (MEMORY_SIZE / 2) as u16,
        }
    }
}

pub fn interpret(instructions: &[Op], tape: &mut Tape) {
    for instruction in instructions {
        match instruction {
            Off(offset, Add(n)) => tape.add(*n, *offset),
            Move(n) => tape.mov(*n),

            Loop(body) => {
                while tape.get(0) != 0 {
                    interpret(body, tape);
                }
            }

            Off(offset, PrintChar) => print!("{}", tape.get(*offset) as char),

            Off(offset, ReadChar) => {
                io::stdout().flush().unwrap();
                tape.set(io::stdin().bytes().next().unwrap().unwrap() as u8, *offset);
            }

            Off(offset, Clear) => tape.set(0, *offset),

            Mul(offset, mul) => tape.add(tape.get(0) * mul, *offset),
            Off(offset, Set(n)) => tape.set(*n, *offset),

            Shift(n) => {
                while tape.get(0) != 0 {
                    tape.mov(*n);
                }
            }
        }
    }
}
