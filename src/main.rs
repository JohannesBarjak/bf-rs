use std::io::Write;
use std::io::Read;

struct Tape {
    cell: [u8; 30000],
    stack: Vec<usize>,
    ptr: usize
}

fn main() {
    let tape = Tape {
        cell: [0; 30000],
        stack: Vec::new(),
        ptr: 0
    };

    let args: Vec<String> = std::env::args().collect();
    (args.len() == 1).then(|| panic!("Enter a bf file"));

    let input = &args[1];
    let input = std::fs::read_to_string(input).unwrap();
    interpreter(input, tape);
}

fn interpreter(input: String, mut tape: Tape) {
    let mut i = 0;

    while i < input.len() {
        match input.chars().nth(i).unwrap() {
            '+' => tape.cell[tape.ptr] += 1,
            '>' => tape.ptr += 1,
            '<' => tape.ptr -= 1,

            '[' => tape.stack.push(i),
            ']' => if tape.cell[tape.ptr] != 0 { i = *tape.stack.last().unwrap(); }
                   else { tape.stack.pop().unwrap(); },

            '-' => tape.cell[tape.ptr] -= 1,

            '.' => { print!("{}", tape.cell[tape.ptr] as char);
                std::io::stdout().flush().unwrap() },

            ',' => tape.cell[tape.ptr] = {
                let pg_input = std::io::stdin()
                    .bytes().next().unwrap();
                pg_input.unwrap() as u8 },

            _ => ()
        }
        i += 1;
    }
}
