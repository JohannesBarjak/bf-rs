mod clap;

use bf::tape::Tape;

fn main() {
    let tape = Tape::new();

    let file = clap::get_file();
    bf::run(tape, file);
}
