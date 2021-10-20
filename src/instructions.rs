#[derive(PartialEq, Debug, Clone)]
pub enum Op {
    Add(u8, isize),
    Move(isize),
    Loop(Vec<Op>),
    PrintChar(isize),
    ReadChar(isize),
    Clear(isize),
    Mul(isize, u8),
    Set(u8, isize),
    Shift(isize),
}
