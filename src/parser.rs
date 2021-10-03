use crate::instructions::Op;
use crate::tokens::Token;

#[must_use]
pub fn parse(tokens: &[Token]) -> Vec<Op> {
    let mut tokens = tokens.iter().peekable();
    let (mut loop_stack, mut instructions) = (Vec::new(), Vec::new());

    while let Some(token) = tokens.next() {
        match token {
            Token::Add | Token::Sub => {
                let mut value: isize = if *token == Token::Add { 1 } else { -1 };

                while matches!(tokens.peek(), Some(Token::Add | Token::Sub)) {
                    value += if *tokens.next().unwrap() == Token::Add {
                        1
                    } else {
                        -1
                    };
                }

                if value != 0 {
                    instructions.push(Op::Add(value as u8));
                }
            }

            Token::MoveRight | Token::MoveLeft => {
                let mut value: isize = if *token == Token::MoveRight { 1 } else { -1 };

                while matches!(tokens.peek(), Some(Token::MoveRight | Token::MoveLeft)) {
                    value += if *tokens.next().unwrap() == Token::MoveRight {
                        1
                    } else {
                        -1
                    };
                }

                if value != 0 {
                    instructions.push(Op::Move(value));
                }
            }

            Token::LoopStart => {
                instructions.push(Op::Loop(Vec::new()));
                loop_stack.push(instructions.len());
            }

            Token::LoopEnd => {
                let loop_start = loop_stack.pop().expect("unmatched `[`");

                let loop_body = instructions.split_off(loop_start);
                *instructions.last_mut().unwrap() = Op::Loop(loop_body);
            }

            Token::PrintChar => instructions.push(Op::PrintChar),
            Token::ReadChar => instructions.push(Op::ReadChar),
        }
    }

    if !loop_stack.is_empty() {
        panic!("unmatched `]`");
    }

    instructions
}
