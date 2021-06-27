pub fn cleanup_input(program: String) -> String {
    program
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}

pub fn optimize_brainfuck(mut program: Vec<u8>) -> Vec<u8> {
    let char_mapping = [(b'+', 'a'), (b'-', 's'), (b'>', 'r'), (b'<', 'l')];
    let mut i = 0;

    while i < program.len() {
        for c in &char_mapping {
            if program[i] == c.0 {
                let start = i;

                while i < program.len() && program[i] == c.0 {
                    i += 1;
                }

                let size = i - start;

                if size > 1 {
                    let optim_str;
                    if size < 10 {
                        optim_str = format!("{}f{}", c.1, size);
                    } else {
                        optim_str = format!("{}{}", c.1, size);
                    }
                    program.splice(start..i, optim_str.as_bytes().iter().cloned());
                }

                i = start;
            }
        }

        if i < program.len() - 2 && program[i] == b'[' && program[i + 2] == b']' {
            let midpoint = program[i + 1];

            if midpoint == b'-' || midpoint == b'+' {
                program.splice(i..i + 3, [b'z'].iter().cloned());
            } else if midpoint == b'>' || midpoint == b'<' {
                program.splice(
                    i..i + 3,
                    [b'm', if midpoint == b'>' { b'r' } else { b'l' }]
                        .iter()
                        .cloned(),
                );
            }
        }

        i += 1;
    }

    program
}
