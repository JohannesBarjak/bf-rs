use crate::tokenizer::{is_loop_end, is_loop_start};
use crate::tokens::Token;

pub fn optimize(input: &mut Vec<Token>) {
    let mut i = 0;

    while i < input.len() - 2 {
        if is_loop_start(&input, &i) && is_loop_end(&input, &(i + 2)) {
            let midpoint = &input[i + 1];

            if *midpoint == Token::Substract(1) || *midpoint == Token::Add(1) {
                input.splice(i..i + 3, [Token::SetToZero].iter().cloned());
            }
        }

        i += 1;
    }
}
