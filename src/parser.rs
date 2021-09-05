use crate::instructions::Opcode;
use crate::tokens::Token;

#[must_use]
pub fn parse(tokens: &[Token]) -> Vec<Opcode> {
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

                instructions.push(Opcode::Add(value));
                i -= 1;
            }

            Token::MoveRight | Token::MoveLeft => {
                let mut value = 0;

                while matches!(tokens.get(i), Some(Token::MoveRight | Token::MoveLeft)) {
                    value += if tokens[i] == Token::MoveRight { 1 } else { -1 };
                    i += 1;
                }

                instructions.push(Opcode::Move(value));
                i -= 1;
            }

            Token::LoopStart => {
                instructions.push(Opcode::Loop(Vec::new()));
                loop_stack.push(instructions.len());
            }

            Token::LoopEnd => {
                let loop_start = loop_stack.pop().expect("unmatched `[`");

                let loop_body = instructions.split_off(loop_start);
                *instructions.last_mut().unwrap() = Opcode::Loop(loop_body);
            }

            Token::PrintChar => instructions.push(Opcode::PrintChar),
            Token::ReadChar => instructions.push(Opcode::ReadChar),
        }

        i += 1;
    }

    instructions
}
