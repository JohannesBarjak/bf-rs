use crate::instructions::Op;
use crate::tokens::Token;

#[must_use]
pub fn parse(tokens: &[Token]) -> Vec<Op> {
    let (mut loop_stack, mut instructions) = (Vec::new(), Vec::new());

    for token in tokens {
        match token {
            Token::Add | Token::Sub => {
                instructions.push(Op::Add(if *token == Token::Add { 1 } else { u8::MAX }));
            }

            Token::MoveRight | Token::MoveLeft => {
                instructions.push(Op::Move(if *token == Token::MoveRight { 1 } else { -1 }));
            }

            Token::LoopStart => {
                instructions.push(Op::Loop(Vec::new()));
                loop_stack.push(instructions.len());
            }

            Token::LoopEnd => {
                let loop_start = loop_stack.pop().expect("unmatched `]`");

                let loop_body = instructions.split_off(loop_start);
                *instructions.last_mut().unwrap() = Op::Loop(loop_body);
            }

            Token::PrintChar => instructions.push(Op::PrintChar),
            Token::ReadChar => instructions.push(Op::ReadChar),
        }
    }

    if !loop_stack.is_empty() {
        panic!("unmatched `[`");
    }

    instructions
}
