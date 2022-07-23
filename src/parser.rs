use crate::instructions::*;
use crate::tokens::Token;

use OffOp::*;
use Op::*;
use Token::*;

#[must_use]
pub fn parse(tokens: &[Token]) -> Vec<Op> {
    let (mut loop_stack, mut instructions) = (Vec::new(), Vec::new());

    for token in tokens {
        match token {
            Plus | Minus => {
                instructions.push(Off(0, Add(if *token == Plus { 1 } else { u8::MAX })));
            }

            Right | Left => {
                instructions.push(Move(if *token == Right { 1 } else { -1 }));
            }

            OpenBracket => {
                instructions.push(Loop(Vec::new()));
                loop_stack.push(instructions.len());
            }

            CloseBracket => {
                let start = loop_stack.pop().expect("unmatched `]`");

                let body = instructions.split_off(start);
                *instructions.last_mut().unwrap() = Loop(body);
            }

            Dot => instructions.push(Off(0, PrintChar)),
            Coma => instructions.push(Off(0, ReadChar)),
        }
    }

    if !loop_stack.is_empty() {
        panic!("unmatched `[`");
    }

    instructions
}
