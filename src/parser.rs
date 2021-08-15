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
                if tokens.get(i + 2) == Some(&Token::LoopEnd)
                    && matches!(tokens[i + 1], Token::Add | Token::Sub)
                {
                    instructions.push(Opcode::Clear);
                    i += 2;
                } else {
                    loop_stack.push(instructions.len());
                    instructions.push(Opcode::LoopStart(0));
                }
            }

            Token::LoopEnd => {
                let loop_start = loop_stack.pop().expect("unmatched `[`");

                instructions[loop_start] = Opcode::LoopStart(instructions.len());
                instructions.push(Opcode::LoopEnd(loop_start));
            }

            Token::PrintChar => instructions.push(Opcode::PrintChar),
            Token::ReadChar => instructions.push(Opcode::ReadChar),
        }

        i += 1;
    }

    instructions
}
