use bf::Tape;
use std::env;

fn main() {
    let tape = Tape {
        cell: [0; 180_000],
        stack: Vec::new(),
        ptr: 90_000,
    };

    let args: Vec<String> = env::args().collect();

    bf::run(tape, args);
}
