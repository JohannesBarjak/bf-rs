use fnv::FnvHashMap;
use itertools::Itertools;

use std::mem;

use crate::instructions::Op;

pub fn optimize(mut instructions: Vec<Op>) -> Vec<Op> {
    let optimizer_pass = |instructions| {
        simplify_code(remove_dead_code(convert_simple_loops(reorder_offsets(
            calculate_offsets(compress_instructions(instructions)),
        ))))
    };

    let mut optimized_instructions = optimizer_pass(instructions.clone());

    while optimized_instructions != instructions {
        instructions = optimized_instructions;
        optimized_instructions = optimizer_pass(instructions.clone());
    }

    optimized_instructions
}

fn simplify_code(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .map(|op| {
            if let Op::Set(0, offset) = op {
                Op::Clear(offset)
            } else {
                op
            }
        })
        .collect()
}

fn remove_dead_code(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Loop(_), Op::Loop(_))
            | (Op::Loop(_), Op::Set(0, 0))
            | (Op::Set(0, 0), Op::Loop(_)) => Ok(op1),

            (Op::Set(n1, offset1), Op::Add(n2, offset2) | Op::Set(n2, offset2))
                if offset1 == offset2 =>
            {
                Ok(if let Op::Add(..) = op2 {
                    Op::Set(n1 + n2, *offset1)
                } else {
                    Op::Set(*n2, *offset1)
                })
            }

            _ => Err((op1, op2)),
        })
        .map(|op| match op {
            Op::Loop(body) => Op::Loop(remove_dead_code(body)),
            _ => op,
        })
        .collect()
}

fn reorder_offsets(mut instructions: Vec<Op>) -> Vec<Op> {
    let mut new_instructions: Vec<Op> = Vec::with_capacity(instructions.len());
    let mut i = 0;

    while i < instructions.len() {
        match &mut instructions[i] {
            Op::Add(..) | Op::Set(..) | Op::Clear(_) => {
                let mut block = Vec::new();

                while let Some(Op::Add(_, offset) | Op::Set(_, offset) | Op::Clear(offset)) =
                    instructions.get(i)
                {
                    block.push((offset, instructions[i].clone()));
                    i += 1;
                }

                i -= 1;

                block.sort_by(|(offset1, _), (offset2, _)| offset1.cmp(offset2));
                new_instructions.append(&mut block.into_iter().map(|(_, op)| op).collect());
            }

            Op::Loop(body) => new_instructions.push(Op::Loop(reorder_offsets(mem::take(body)))),
            _ => new_instructions.push(mem::replace(&mut instructions[i], Op::Clear(0))),
        }

        i += 1;
    }

    new_instructions
}

fn calculate_offsets(mut instructions: Vec<Op>) -> Vec<Op> {
    let mut new_instructions = Vec::with_capacity(instructions.len());
    let mut instructions = instructions.iter_mut().peekable();

    while let Some(op) = instructions.next() {
        match op {
            Op::Add(..)
            | Op::Set(..)
            | Op::Clear(_)
            | Op::PrintChar(_)
            | Op::ReadChar(_)
            | Op::Move(_) => {
                let mut block = vec![op];
                let mut new_block = Vec::new();
                let mut offset = 0;

                block.append(
                    &mut instructions
                        .peeking_take_while(|op| {
                            matches!(
                                op,
                                Op::Add(..)
                                    | Op::Set(..)
                                    | Op::Clear(_)
                                    | Op::PrintChar(_)
                                    | Op::ReadChar(_)
                                    | Op::Move(_)
                            )
                        })
                        .collect_vec(),
                );

                for op in block {
                    match op {
                        Op::Add(n, off) => new_block.push(Op::Add(*n, *off + offset)),
                        Op::Set(n, off) => new_block.push(Op::Set(*n, *off + offset)),

                        Op::Clear(off) => new_block.push(Op::Clear(*off + offset)),

                        Op::PrintChar(off) => new_block.push(Op::PrintChar(*off + offset)),
                        Op::ReadChar(off) => new_block.push(Op::ReadChar(*off + offset)),

                        Op::Move(off) => offset += *off,

                        _ => unreachable!(),
                    }
                }

                if offset != 0 {
                    new_block.push(Op::Move(offset));
                }

                new_instructions.append(&mut new_block);
            }

            Op::Loop(body) => new_instructions.push(Op::Loop(calculate_offsets(mem::take(body)))),
            _ => new_instructions.push(mem::replace(op, Op::Clear(0))),
        }
    }

    new_instructions
}

fn compress_instructions(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Add(n1, offset1), Op::Add(n2, offset2)) if offset1 == offset2 => {
                Ok(Op::Add(n1 + n2, *offset1))
            }

            (Op::Move(n1), Op::Move(n2)) => Ok(Op::Move(n1 + n2)),

            _ => Err((op1, op2)),
        })
        .filter_map(|op| match op {
            Op::Loop(body) => Some(Op::Loop(compress_instructions(body))),
            Op::Add(0, _) | Op::Move(0) => None,

            _ => Some(op),
        })
        .collect()
}

fn convert_simple_loops(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .flat_map(|op| {
            if let Op::Loop(body) = op {
                match body[..] {
                    [Op::Add(1 | u8::MAX, offset)] => vec![Op::Set(0, offset)],

                    [Op::Move(step)] => vec![Op::Shift(step)],

                    _ if (&body[..])
                        .iter()
                        .all(|op| matches!(op, Op::Add(_, _) | Op::Move(_))) =>
                    {
                        let mut offset = 0;
                        let mut tape_map = FnvHashMap::default();

                        for op in &body {
                            if let Op::Move(n) = op {
                                offset += n;
                            } else if let Op::Add(mul, off) = op {
                                tape_map.insert(
                                    offset + off,
                                    mul + tape_map.get(&(offset + off)).unwrap_or(&0),
                                );
                            };
                        }

                        if offset == 0 && tape_map.get(&0) == Some(&u8::MAX) {
                            tape_map.remove(&0);

                            let mut replacement = Vec::new();

                            for (offset, mul) in tape_map {
                                replacement.push(Op::Mul(offset, mul));
                            }

                            replacement.push(Op::Set(0, 0));
                            replacement
                        } else {
                            vec![Op::Loop(body)]
                        }
                    }

                    _ => vec![Op::Loop(convert_simple_loops(body))],
                }
            } else {
                vec![op]
            }
        })
        .collect()
}
