use crate::tokens::Token;

#[must_use]
pub fn tokenize(source: &str) -> Vec<Token> {
    source
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Token::Add),
            '-' => Some(Token::Sub),
            '>' => Some(Token::MoveRight),
            '<' => Some(Token::MoveLeft),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            '.' => Some(Token::PrintChar),
            ',' => Some(Token::ReadChar),
            _ => None,
        })
        .collect()
}
