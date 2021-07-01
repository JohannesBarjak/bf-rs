use bf::Tape;

const MEMORY_SIZE: usize = 180_000;

fn main() {
    let tape = Tape {
        memory: [0; MEMORY_SIZE],
        stack: Vec::new(),
        ptr: MEMORY_SIZE / 2,
    };

    bf::run(tape);
}
