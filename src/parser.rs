mod optimizer;

use crate::opcodes::*;

pub fn parse_input(input: String) -> Vec<OpKind> {
    let input = cleanup_input(input).as_bytes().to_vec();
    let mut parsed_input = Vec::new();

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
                    b'+' => parsed_input.push(OpKind::Add(size as u8)),
                    b'-' => parsed_input.push(OpKind::Substract(size as u8)),
                    b'>' => parsed_input.push(OpKind::MovePtrRight(size)),
                    b'<' => parsed_input.push(OpKind::MovePtrLeft(size)),
                    _ => unreachable!(),
                }

                i -= 1;
            }
        }

        if input[i] == b'[' {
            parsed_input.push(OpKind::LoopStartPlaceholder);
        } else if input[i] == b']' {
            parsed_input.push(OpKind::LoopEndPlaceholder);
        } else if input[i] == b'.' {
            parsed_input.push(OpKind::PrintChar);
        } else if input[i] == b',' {
            parsed_input.push(OpKind::ReadChar);
        }

        i += 1;
    }

    let mut parsed_input = optimizer::optimize_input(parsed_input);

    let mut i = 0;

    while i < parsed_input.len() {
        if is_loop_start(&parsed_input, &i) || is_loop_end(&parsed_input, &i) {
            let mut loop_count = 1;
            let start = i;

            if is_loop_start(&parsed_input, &i) {
                while loop_count != 0 {
                    i += 1;
                    if is_loop_start(&parsed_input, &i) {
                        loop_count += 1;
                    } else if is_loop_end(&parsed_input, &i) {
                        loop_count -= 1;
                    }
                }
            } else if is_loop_end(&parsed_input, &i) {
                while loop_count != 0 {
                    i -= 1;
                    if is_loop_start(&parsed_input, &i) {
                        loop_count -= 1
                    } else if is_loop_end(&parsed_input, &i) {
                        loop_count += 1;
                    }
                }
            }

            if is_loop_start(&parsed_input, &start) {
                parsed_input[start] = OpKind::LoopStart { loop_end_addr: i };
            } else if is_loop_end(&parsed_input, &start) {
                parsed_input[start] = OpKind::LoopEnd { loop_start_addr: i };
            }

            i = start;
        }

        i += 1;
    }

    parsed_input
}

fn cleanup_input(input: String) -> String {
    input
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}
