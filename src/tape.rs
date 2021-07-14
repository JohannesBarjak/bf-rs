pub const MEMORY_SIZE: usize = 180_000;

pub struct Tape {
    pub memory: [u8; MEMORY_SIZE],
    pub ptr: usize,
}

impl Tape {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Tape {
    fn default() -> Self {
        Tape {
            memory: [0; MEMORY_SIZE],
            ptr: MEMORY_SIZE / 2,
        }
    }
}
