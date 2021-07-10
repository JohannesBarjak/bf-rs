use std::mem::discriminant;

mod optimizer;

use crate::tokens::Token;

pub fn tokenize(input: String) -> Vec<Token> {
    let input = sanitize(input).as_bytes().to_vec();
    let mut tokenized_input = Vec::new();

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
                    b'+' => tokenized_input.push(Token::Add(size as u8)),
                    b'-' => tokenized_input.push(Token::Substract(size as u8)),
                    b'>' => tokenized_input.push(Token::MovePtrRight(size)),
                    b'<' => tokenized_input.push(Token::MovePtrLeft(size)),
                    _ => unreachable!(),
                }

                i -= 1;
            }
        }

        match input[i] {
            b'[' => tokenized_input.push(Token::LoopStartPlaceholder),
            b']' => tokenized_input.push(Token::LoopEndPlaceholder),
            b'.' => tokenized_input.push(Token::PrintChar),
            b',' => tokenized_input.push(Token::ReadChar),
            _ => (),
        }

        i += 1;
    }

    optimizer::optimize(&mut tokenized_input);

    let mut i = 0;

    while i < tokenized_input.len() {
        if is_loop_start(&tokenized_input, &i) || is_loop_end(&tokenized_input, &i) {
            let mut loop_count = 1;
            let start = i;

            if is_loop_start(&tokenized_input, &i) {
                while loop_count != 0 {
                    i += 1;
                    if is_loop_start(&tokenized_input, &i) {
                        loop_count += 1;
                    } else if is_loop_end(&tokenized_input, &i) {
                        loop_count -= 1;
                    }
                }
            } else if is_loop_end(&tokenized_input, &i) {
                while loop_count != 0 {
                    i -= 1;
                    if is_loop_start(&tokenized_input, &i) {
                        loop_count -= 1
                    } else if is_loop_end(&tokenized_input, &i) {
                        loop_count += 1;
                    }
                }
            }

            if is_loop_start(&tokenized_input, &start) {
                tokenized_input[start] = Token::LoopStart { loop_end_addr: i };
            } else if is_loop_end(&tokenized_input, &start) {
                tokenized_input[start] = Token::LoopEnd { loop_start_addr: i };
            }

            i = start;
        }

        i += 1;
    }

    tokenized_input
}

fn sanitize(input: String) -> String {
    input
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}

fn is_loop_start(input: &[Token], i: &usize) -> bool {
    input[*i] == Token::LoopStartPlaceholder
        || discriminant(&input[*i]) == discriminant(&Token::LoopStart { loop_end_addr: 1 })
}

fn is_loop_end(input: &[Token], i: &usize) -> bool {
    input[*i] == Token::LoopEndPlaceholder
        || discriminant(&input[*i]) == discriminant(&Token::LoopEnd { loop_start_addr: 1 })
}
