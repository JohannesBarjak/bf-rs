pub fn cleanup_input(input: String) -> String {
    input
        .chars()
        .filter(|i| "+-><[].,".chars().any(|c| c == *i))
        .collect()
}

pub fn optimize_brainfuck(mut input: Vec<u8>) -> Vec<u8> {
    let char_mapping = [(b'+', 'a'), (b'-', 's'), (b'>', 'r'), (b'<', 'l')];
    let mut i = 0;

    while i < input.len() {
        for c in &char_mapping {
            if input[i] == c.0 {
                let start = i;

                while i < input.len() && input[i] == c.0 {
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
                    input.splice(start..i, optim_str.as_bytes().iter().cloned());
                }

                i = start;
            }
        }

        if i < input.len() - 2 && input[i] == b'[' && input[i + 2] == b']' {
            let midpoint = input[i + 1];

            if midpoint == b'-' || midpoint == b'+' {
                input.splice(i..i + 3, [b'z'].iter().cloned());
            } else if midpoint == b'>' || midpoint == b'<' {
                input.splice(
                    i..i + 3,
                    [b'm', if midpoint == b'>' { b'r' } else { b'l' }]
                        .iter()
                        .cloned(),
                );
            }
        }

        i += 1;
    }

    input
}
