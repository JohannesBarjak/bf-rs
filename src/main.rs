use std::io::Read;
use std::io::Write;

use std::env;
use std::fs;
use std::io;
use std::process;
use std::vec::Vec;

struct Tape {
    cell: [u8; 180_000],
    stack: Vec<usize>,
    ptr: usize,
}

fn main() {
    let tape = Tape {
        cell: [0; 180_000],
        stack: Vec::new(),
        ptr: 90_000,
    };

    let args: Vec<String> = env::args().collect();

    let input = args.get(1).expect("Expected one argument");
    process_args(&input);

    let input = cleanup_input(fs::read_to_string(input).expect("Invalid filename"));

    interpret(&optimize_brainfuck(input.as_bytes().to_vec())[..], tape);
}

fn interpret(input: &[u8], mut tape: Tape) {
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

fn cleanup_input(input: String) -> String {
    input
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}

fn optimize_brainfuck(mut input: Vec<u8>) -> Vec<u8> {
    let char_mapping = [(b'+', 'a'), (b'-', 's'), (b'>', 'r'), (b'<', 'l')];
    let mut i = 0;

    while i < input.len() {
        for c in &char_mapping {
            if input[i] == c.0 {
                let start = i;

                while i < input.len() && input[i] == c.0 {
                    i += 1;
                }

                let size = i - start;

                if size > 1 {
                    if size < 10 {
                        input.splice(
                            start..i,
                            format!("{}f{}", c.1, size).as_bytes().iter().cloned(),
                        );
                    } else {
                        input.splice(
                            start..i,
                            format!("{}{}", c.1, size).as_bytes().iter().cloned(),
                        );
                    }
                }

                i = start;
            }
        }

        if i < input.len() - 2 && input[i] == b'[' && input[i + 2] == b']' {
            let midpoint = input[i + 1];

            if midpoint == b'-' || midpoint == b'+' {
                input.splice(i..i + 3, [b'z'].iter().cloned());
            } else if midpoint == b'>' || midpoint == b'<' {
                input.splice(
                    i..i + 3,
                    [b'm', if midpoint == b'>' { b'r' } else { b'l' }]
                        .iter()
                        .cloned(),
                );
            }
        }

        i += 1;
    }

    input
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

fn process_args(input: &str) {
    if input == "-h" || input == "--help" {
        show_help();
        process::exit(0);
    } else if input == "-v" || input == "--version" {
        println!("bf-rs v3.0.0");
        process::exit(0);
    }
}

fn show_help() {
    println!(
        "Usage:
    bf [file]

    -h, --help        show help
    -v, --version     show bf-rs version"
    );
}
