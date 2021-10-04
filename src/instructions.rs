#[derive(PartialEq, Debug, Clone)]
pub enum Op {
    Add(u8),
    Move(isize),
    Loop(Vec<Op>),
    PrintChar,
    ReadChar,
    Clear,
    Mul(isize, u8),
    Set(u8),
    Shift(isize),
}
