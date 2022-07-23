#[derive(PartialEq, Debug, Clone)]
pub enum Op {
    Off(isize, OffOp),
    Move(isize),
    Loop(Vec<Op>),
    Mul(isize, u8),
    Shift(isize),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OffOp {
    Add(u8),
    PrintChar,
    ReadChar,
    Clear,
    Set(u8),
}
