use std::mem::discriminant;

#[derive(PartialEq, Clone)]
pub enum OpKind {
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

pub fn is_loop_start(input: &[OpKind], i: &usize) -> bool {
    input[*i] == OpKind::LoopStartPlaceholder
        || discriminant(&input[*i]) == discriminant(&OpKind::LoopStart { loop_end_addr: 1 })
}

pub fn is_loop_end(input: &[OpKind], i: &usize) -> bool {
    input[*i] == OpKind::LoopEndPlaceholder
        || discriminant(&input[*i]) == discriminant(&OpKind::LoopEnd { loop_start_addr: 1 })
}
