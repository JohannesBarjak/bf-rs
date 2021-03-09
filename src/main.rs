use std::io::Write;
use std::io::Read;

struct Tape {
    cell: [u8; 60_000],
    stack: Vec<usize>,
    ptr: usize
}

fn main() {
    let tape = Tape {
        cell: [0; 60_000],
        stack: Vec::new(),
        ptr: 30_000
    };

    let args: Vec<String> = std::env::args().collect();
    (args.len() == 1).then(|| panic!("Enter a bf file"));

    let input = &args[1];
    let input = std::fs::read_to_string(input).unwrap();
    interpreter(input, tape);
}

fn interpreter(input: String, mut tape: Tape) {
    let mut i = 0;
    let mut loop_count = 0;

    let input_char = |i| input.chars().nth(i).unwrap();

    while i < input.len() {
        match input_char(i) {
            '+' => tape.cell[tape.ptr] += 1,
            '>' => tape.ptr += 1,
            '<' => tape.ptr -= 1,

            '[' => if tape.cell[tape.ptr] != 0 { tape.stack.push(i); } else {
                       loop_count += 1;
                       while loop_count != 0 {
                           i += 1;
                           if input_char(i) == '[' { loop_count += 1; }
                           else if input_char(i) == ']' { loop_count -= 1; }
                       }
                   },

            ']' => if tape.cell[tape.ptr] != 0 { i = *tape.stack.last().unwrap(); } else {
                       tape.stack.pop().unwrap();
                   },

            '-' => tape.cell[tape.ptr] -= 1,

            '.' => { print!("{}", tape.cell[tape.ptr] as char);
                     std::io::stdout().flush().unwrap() },

            ',' => tape.cell[tape.ptr] = {
                       let pg_input = std::io::stdin()
                           .bytes().next().unwrap();
                       pg_input.unwrap() as u8
                   },

            _ => ()
        }
        i += 1;
    }
}
