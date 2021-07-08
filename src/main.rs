mod clap;

use bf::Tape;
use bf::MEMORY_SIZE;

fn main() {
    let tape = Tape {
        memory: [0; MEMORY_SIZE],
        ptr: MEMORY_SIZE / 2,
    };

    let file = clap::get_file();
    bf::run(tape, file);
}
