#[derive(PartialEq, Debug, Clone)]
pub enum Op {
    Add(isize),
    Move(isize),
    Loop(Vec<Op>),
    PrintChar,
    ReadChar,
    Clear,
    Mul(isize, isize),
    Shift(isize),
}
