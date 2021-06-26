use crate::Tape;

use std::io;
use std::io::{Read, Write};

pub fn interpret(input: &[u8], mut tape: Tape) {
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            b'a' => tape.cell[tape.ptr] += get_num(input, &mut i) as u8,
            b'+' => tape.cell[tape.ptr] += 1,

            b's' => tape.cell[tape.ptr] -= get_num(input, &mut i) as u8,
            b'-' => tape.cell[tape.ptr] -= 1,

            b'r' => tape.ptr += get_num(input, &mut i),
            b'>' => tape.ptr += 1,

            b'l' => tape.ptr -= get_num(input, &mut i),
            b'<' => tape.ptr -= 1,

            b'[' => {
                if tape.cell[tape.ptr] != 0 {
                    tape.stack.push(i);
                } else {
                    let mut loop_count = 1;
                    while loop_count != 0 {
                        i += 1;
                        if input[i] == b'[' {
                            loop_count += 1;
                        } else if input[i] == b']' {
                            loop_count -= 1;
                        }
                    }
                }
            }

            b']' => {
                if tape.cell[tape.ptr] != 0 {
                    i = *tape.stack.last().expect("Unmatched ] character");
                } else {
                    tape.stack.pop().expect("Unmatched ] character");
                }
            }

            b'.' => {
                print!("{}", tape.cell[tape.ptr] as char);
                io::stdout().flush().unwrap();
            }

            b',' => tape.cell[tape.ptr] = io::stdin().bytes().next().unwrap().unwrap() as u8,

            b'm' => {
                i += 1;
                if input[i] == b'r' {
                    while tape.cell[tape.ptr] != 0 {
                        tape.ptr += 1;
                    }
                } else if input[i] == b'l' {
                    while tape.cell[tape.ptr] != 0 {
                        tape.ptr -= 1;
                    }
                }
            }

            b'z' => tape.cell[tape.ptr] = 0,

            _ => (),
        }
        i += 1;
    }

    if !tape.stack.is_empty() {
        panic!("Unmatched [ character");
    }
}

fn get_num(input: &[u8], i: &mut usize) -> usize {
    *i += 1;
    if input[*i] != b'f' {
        let mut num = Vec::new();

        while *i < input.len() && 47 < input[*i] && input[*i] < 58 {
            num.insert(0, input[*i] - 48);
            *i += 1;
        }
        *i -= 1;

        let mut sum = 0;

        for (exp, n) in num.into_iter().enumerate() {
            sum += (n as usize) * (10_i32.pow(exp as u32) as usize);
        }

        sum
    } else {
        *i += 1;
        (input[*i] - 48).into()
    }
}
