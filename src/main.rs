use bf::Tape;

fn main() {
    let tape = Tape {
        cell: [0; 180_000],
        stack: Vec::new(),
        ptr: 90_000,
    };

    bf::run(tape);
}
