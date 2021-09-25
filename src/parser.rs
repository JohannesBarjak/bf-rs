use crate::instructions::Op;
use crate::tokens::Token;

#[must_use]
pub fn parse(tokens: &[Token]) -> Vec<Op> {
    let (mut loop_stack, mut instructions) = (Vec::new(), Vec::new());

    let mut i = 0;

    while i < tokens.len() {
        match tokens[i] {
            Token::Add | Token::Sub => {
                let mut value = 0;

                while matches!(tokens.get(i), Some(Token::Add | Token::Sub)) {
                    value += if tokens[i] == Token::Add { 1 } else { -1 };
                    i += 1;
                }

                instructions.push(Op::Add(value as u8));
                i -= 1;
            }

            Token::MoveRight | Token::MoveLeft => {
                let mut value = 0;

                while matches!(tokens.get(i), Some(Token::MoveRight | Token::MoveLeft)) {
                    value += if tokens[i] == Token::MoveRight { 1 } else { -1 };
                    i += 1;
                }

                instructions.push(Op::Move(value));
                i -= 1;
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

        i += 1;
    }

    instructions
}
