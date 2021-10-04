use fnv::FnvHashMap;

use crate::instructions::Op;

pub fn optimize(instructions: &mut Vec<Op>) {
    for _ in 0..2 {
        compress_instructions(instructions);
        optimize_loops(instructions);
        set_optimization(instructions);
    }
}

fn compress_instructions(instructions: &mut Vec<Op>) {
    let mut i = 0;

    while i < instructions.len() {
        match &mut instructions[i] {
            Op::Add(_) => {
                let mut value: u8 = 0;
                let start = i;

                while let Some(Op::Add(n)) = instructions.get(i) {
                    value = value.wrapping_add(*n);
                    i += 1;
                }

                if value != 0 {
                    instructions.splice(start..i, [Op::Add(value)].iter().cloned());
                } else {
                    instructions.splice(start..i, [].iter().cloned());
                }

                i = start;
            }

            Op::Move(_) => {
                let mut value: isize = 0;
                let start = i;

                while let Some(Op::Move(n)) = instructions.get(i) {
                    value += *n;
                    i += 1;
                }

                if value != 0 {
                    instructions.splice(start..i, [Op::Move(value)].iter().cloned());
                } else {
                    instructions.splice(start..i, [].iter().cloned());
                }

                i = start;
            }

            Op::Loop(loop_body) => compress_instructions(loop_body),

            _ => (),
        }

        i += 1;
    }
}

fn optimize_loops(instructions: &mut Vec<Op>) {
    let mut i = 0;

    while i < instructions.len() {
        if let Op::Loop(loop_body) = &mut instructions[i] {
            match loop_body[..] {
                [Op::Add(1 | u8::MAX)] => instructions[i] = Op::Clear,

                [Op::Move(step)] => instructions[i] = Op::Shift(step),

                _ if (&loop_body[..])
                    .iter()
                    .all(|op| matches!(op, Op::Add(_) | Op::Move(_))) =>
                {
                    let mut offset = 0;
                    let mut tape_map = FnvHashMap::default();

                    for op in loop_body {
                        if let Op::Move(n) = op {
                            offset += *n;
                        } else if let Op::Add(mul) = op {
                            tape_map.insert(offset, *mul + tape_map.get(&offset).unwrap_or(&0));
                        };
                    }

                    if offset == 0 && tape_map.get(&0) == Some(&u8::MAX) {
                        tape_map.remove(&0);

                        let mut replacement = Vec::new();

                        for (offset, mul) in &tape_map {
                            replacement.push(Op::Mul(*offset, *mul));
                        }

                        replacement.push(Op::Clear);
                        instructions.splice(i..i + 1, replacement.iter().cloned());
                    }
                }

                _ => optimize(loop_body),
            }
        }

        i += 1;
    }
}

fn set_optimization(instructions: &mut Vec<Op>) {
    let mut i = 0;

    while i < instructions.len() {
        match &mut instructions[i] {
            Op::Clear => {
                if let Some(Op::Add(n)) = instructions.get(i + 1) {
                    let n = *n;
                    instructions.splice(i..i + 2, [Op::Set(n)].iter().cloned());
                }
            }

            Op::Loop(loop_body) => set_optimization(loop_body),

            _ => (),
        }

        i += 1;
    }
}
