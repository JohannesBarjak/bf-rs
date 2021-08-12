#[derive(PartialEq)]
pub enum Opcode {
    Add(isize),
    Move(isize),
    LoopStart(usize),
    LoopEnd(usize),
    PrintChar,
    ReadChar,
}
