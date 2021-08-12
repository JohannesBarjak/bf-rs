#[derive(PartialEq)]
pub enum Token {
    Add,
    Sub,
    MoveLeft,
    MoveRight,
    LoopStart,
    LoopEnd,
    PrintChar,
    ReadChar,
}
