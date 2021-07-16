mod optimizer;

use crate::instructions::Opcode;

pub fn parse(input: String) -> Vec<Opcode> {
    let mut output = Vec::new();

    {
        let input = remove_comments(input).as_bytes().to_vec();
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
    }

    optimizer::optimize(&mut output);

    {
        let mut stack = Vec::new();
        let mut i = 0;

        while i < output.len() {
            match output[i] {
                Opcode::LoopStartPlaceholder => stack.push(i),
                Opcode::LoopEndPlaceholder => {
                    let loop_start = stack.pop().unwrap();

                    output[i] = Opcode::LoopEnd {
                        loop_start_addr: loop_start,
                    };

                    output[loop_start] = Opcode::LoopStart { loop_end_addr: i };
                }

                _ => (),
            }

            i += 1;
        }
    }

    output
}

fn remove_comments(input: String) -> String {
    input
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}
