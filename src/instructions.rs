#[derive(PartialEq, Debug, Clone)]
pub enum Opcode {
    Add(isize),
    Move(isize),
    Loop(Vec<Opcode>),
    PrintChar,
    ReadChar,
    Clear,
}
