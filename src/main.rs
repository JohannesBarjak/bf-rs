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

    interpret(optimize_brainfuck(input).as_bytes(), tape);
}

fn interpret(input: &[u8], mut tape: Tape) {
    let mut i = 0;

    while i < input.len() {
        match input[i] {
            b'a' => {
                i += 1;
                tape.cell[tape.ptr] += input[i] - 48;
            }

            b'+' => tape.cell[tape.ptr] += 1,

            b's' => {
                i += 1;
                tape.cell[tape.ptr] -= input[i] - 48;
            }

            b'-' => tape.cell[tape.ptr] -= 1,

            b'r' => {
                i += 1;
                tape.ptr += (input[i] - 48) as usize;
            }

            b'>' => tape.ptr += 1,

            b'l' => {
                i += 1;
                tape.ptr -= (input[i] - 48) as usize;
            }

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
                std::io::stdout().flush().unwrap();
            }

            b',' => tape.cell[tape.ptr] = std::io::stdin()
                .bytes().next().unwrap().unwrap() as u8,

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

fn optimize_brainfuck(mut input: String) -> String {
    for i in (2..=9).rev() {
        for c in 0..=3 {
            input = input.replace(
                &"+-><".chars().nth(c).unwrap().to_string().repeat(i),
                &format!("{}{}", "asrl".chars().nth(c).unwrap(), i),
            )
        }
    }

    input.replace("[-]", "z").replace("[+]", "z")
}
