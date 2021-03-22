use std::io::Read;
use std::io::Write;

struct Tape {
    cell: [u8; 60_000],
    stack: Vec<usize>,
    ptr: usize,
}

fn main() {
    let tape = Tape {
        cell: [0; 60_000],
        stack: Vec::new(),
        ptr: 30_000,
    };

    let args: Vec<String> = std::env::args().collect();

    let input = args.get(1).expect("No input file specified");
    let input = cleanup_input(std::fs::read_to_string(input)
        .expect("Invalid filename"));

    interpreter(optimize_brainfuck(input).as_bytes(), tape);
}

fn interpreter(input: &[u8], mut tape: Tape) {
    let mut i = 0;
    let input_char = |i| input[i] as char;

    while i < input.len() {
        match input_char(i) {
            'a' => {
                i += 1;
                tape.cell[tape.ptr] += input_char(i).to_digit(10).unwrap() as u8;
            }

            '+' => tape.cell[tape.ptr] += 1,

            's' => {
                i += 1;
                tape.cell[tape.ptr] -= input_char(i).to_digit(10).unwrap() as u8;
            }

            '-' => tape.cell[tape.ptr] -= 1,

            'r' => {
                i += 1;
                tape.ptr += input_char(i).to_digit(10).unwrap() as usize;
            }

            '>' => tape.ptr += 1,

            'l' => {
                i += 1;
                tape.ptr -= input_char(i).to_digit(10).unwrap() as usize;
            }

            '<' => tape.ptr -= 1,

            '[' => {
                if tape.cell[tape.ptr] != 0 {
                    tape.stack.push(i);
                } else {
                    let mut loop_counter = 1;
                    while loop_counter != 0 {
                        i += 1;
                        if input_char(i) == '[' {
                            loop_counter += 1;
                        } else if input_char(i) == ']' {
                            loop_counter -= 1;
                        }
                    }
                }
            }

            ']' => {
                if tape.cell[tape.ptr] != 0 {
                    i = *tape.stack.last().expect("Unmatched ] character");
                } else {
                    tape.stack.pop().expect("Unmatched ] character");
                }
            }

            '.' => {
                print!("{}", tape.cell[tape.ptr] as char);
                std::io::stdout().flush().unwrap()
            }

            ',' => tape.cell[tape.ptr] = std::io::stdin()
                .bytes().next().unwrap().unwrap() as u8,

            'z' => tape.cell[tape.ptr] = 0,

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

fn optimize_brainfuck(mut input: String) -> String {
    for i in (2..=9).rev() {
        for c in 0..=3 {
            input = input.replace(
                (b"+-><"[c] as char).to_string().repeat(i).as_str(),
                format!("{}{}", b"asrl"[c] as char, i).as_str(),
            )
        }
    }

    input.replace("[-]", "z").replace("[+]", "z")
}
