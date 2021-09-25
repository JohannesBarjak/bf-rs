use fnv::FnvHashMap;

use crate::instructions::Op;

pub fn optimize(instructions: &mut Vec<Op>) {
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
