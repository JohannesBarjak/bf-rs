pub mod instructions;
pub mod interpreter;
pub mod optimizer;
pub mod parser;
pub mod tokenizer;
pub mod tokens;
pub mod transpiler;

pub const MEMORY_SIZE: usize = u16::MAX as usize;
