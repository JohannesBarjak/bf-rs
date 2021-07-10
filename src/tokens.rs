#[derive(PartialEq, Clone)]
pub enum Token {
    Add(u8),
    Substract(u8),
    MovePtrRight(usize),
    MovePtrLeft(usize),
    LoopStart { loop_end_addr: usize },
    LoopStartPlaceholder,
    LoopEnd { loop_start_addr: usize },
    LoopEndPlaceholder,
    PrintChar,
    ReadChar,
    SetToZero,
}
