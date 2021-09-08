use crate::instructions::Op;

pub fn optimize(instructions: &mut Vec<Op>) {
    let mut i = 0;

    while i < instructions.len() {
        if let Op::Loop(loop_body) = &mut instructions[i] {
            match loop_body[..] {
                [Op::Add(1 | -1)] => instructions[i] = Op::Clear,

                [Op::Add(-1), ..]
                    if (&loop_body[1..])
                        .iter()
                        .all(|op| matches!(op, Op::Add(1) | Op::Move(_))) =>
                {
                    let copy_body = &loop_body[1..];

                    let mut offset = 0;
                    let mut copy_stack = Vec::new();

                    for op in copy_body {
                        if let Op::Move(n) = op {
                            offset += *n
                        } else if let Op::Add(mul) = op {
                            copy_stack.push(Op::Mul(offset, *mul));
                        };
                    }

                    if offset == 0 {
                        copy_stack.push(Op::Clear);
                        instructions.splice(i..i + 1, copy_stack.iter().cloned());
                    }
                }

                _ => optimize(loop_body),
            }
        }

        i += 1;
    }
}
