use crate::tokens::Token;

use Token::*;

#[must_use]
pub fn tokenize(source: &str) -> Vec<Token> {
    source
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Plus),
            '-' => Some(Minus),
            '>' => Some(Right),
            '<' => Some(Left),
            '[' => Some(OpenBracket),
            ']' => Some(CloseBracket),
            '.' => Some(Dot),
            ',' => Some(Coma),
            _ => None,
        })
        .collect()
}
