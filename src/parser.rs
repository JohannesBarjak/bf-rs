use std::mem::discriminant;

mod optimizer;

use crate::instructions::Opcode;

pub fn parse(input: String) -> Vec<Opcode> {
    let input = sanitize(input).as_bytes().to_vec();
    let mut output = Vec::new();

    let mut i = 0;

    while i < input.len() {
        for c in b"+-><" {
            if input[i] == *c {
                let start = i;

                while i < input.len() && input[i] == *c {
                    i += 1;
                }

                let size = i - start;

                match *c {
                    b'+' => output.push(Opcode::Add(size as u8)),
                    b'-' => output.push(Opcode::Substract(size as u8)),
                    b'>' => output.push(Opcode::MovePtrRight(size)),
                    b'<' => output.push(Opcode::MovePtrLeft(size)),
                    _ => unreachable!(),
                }

                i -= 1;
            }
        }

        match input[i] {
            b'[' => output.push(Opcode::LoopStartPlaceholder),
            b']' => output.push(Opcode::LoopEndPlaceholder),
            b'.' => output.push(Opcode::PrintChar),
            b',' => output.push(Opcode::ReadChar),
            _ => (),
        }

        i += 1;
    }

    optimizer::optimize(&mut output);

    let mut i = 0;

    while i < output.len() {
        if is_loop_start(&output, &i) || is_loop_end(&output, &i) {
            let mut loop_count = 1;
            let start = i;

            if is_loop_start(&output, &i) {
                while loop_count != 0 {
                    i += 1;
                    if is_loop_start(&output, &i) {
                        loop_count += 1;
                    } else if is_loop_end(&output, &i) {
                        loop_count -= 1;
                    }
                }
            } else if is_loop_end(&output, &i) {
                while loop_count != 0 {
                    i -= 1;
                    if is_loop_start(&output, &i) {
                        loop_count -= 1
                    } else if is_loop_end(&output, &i) {
                        loop_count += 1;
                    }
                }
            }

            if is_loop_start(&output, &start) {
                output[start] = Opcode::LoopStart { loop_end_addr: i };
            } else if is_loop_end(&output, &start) {
                output[start] = Opcode::LoopEnd { loop_start_addr: i };
            }

            i = start;
        }

        i += 1;
    }

    output
}

fn sanitize(input: String) -> String {
    input
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}

fn is_loop_start(input: &[Opcode], i: &usize) -> bool {
    input[*i] == Opcode::LoopStartPlaceholder
        || discriminant(&input[*i]) == discriminant(&Opcode::LoopStart { loop_end_addr: 1 })
}

fn is_loop_end(input: &[Opcode], i: &usize) -> bool {
    input[*i] == Opcode::LoopEndPlaceholder
        || discriminant(&input[*i]) == discriminant(&Opcode::LoopEnd { loop_start_addr: 1 })
}
