#[derive(PartialEq, Debug)]
pub enum Opcode {
    Add(isize),
    Move(isize),
    Loop(Vec<Opcode>),
    PrintChar,
    ReadChar,
    Clear,
}
