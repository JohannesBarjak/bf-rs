use fnv::FnvHashMap;
use itertools::Itertools;

use std::mem;

use crate::instructions::Op;

pub fn optimize(mut instructions: Vec<Op>) -> Vec<Op> {
    let optimizer_pass = |instructions| {
        simplify_code(remove_dead_code(set_optimization(convert_simple_loops(
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
        .map(|op| if op == Op::Set(0) { Op::Clear } else { op })
        .collect()
}

fn remove_dead_code(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Loop(_), Op::Loop(_)) => Ok(op1),
            (Op::Loop(_), Op::Set(0)) => Ok(op1),
            (Op::Set(0), Op::Loop(_)) => Ok(op1),

            (Op::Set(_), Op::Set(n2)) => Ok(Op::Set(*n2)),

            _ => Err((op1, op2)),
        })
        .map(|op| match op {
            Op::Loop(body) => Op::Loop(remove_dead_code(body)),
            _ => op,
        })
        .collect()
}

fn calculate_offsets(mut instructions: Vec<Op>) -> Vec<Op> {
    let mut new_instructions = Vec::with_capacity(instructions.len());
    let mut instructions = instructions.iter_mut().peekable();

    while let Some(op) = instructions.next() {
        match op {
            Op::Add(..) | Op::Move(_) => {
                let mut part = vec![op];
                let mut reg = Vec::new();
                let mut offset = 0;

                part.append(
                    &mut instructions
                        .peeking_take_while(|op| matches!(op, Op::Add(..) | Op::Move(_)))
                        .collect_vec(),
                );

                for op in part {
                    match op {
                        Op::Add(n, off) => reg.push(Op::Add(*n, *off + offset)),
                        Op::Move(off) => offset += *off,

                        _ => unreachable!(),
                    }
                }

                reg.push(Op::Move(offset));
                new_instructions.append(&mut reg);
            }

            Op::Loop(body) => new_instructions.push(Op::Loop(calculate_offsets(mem::take(body)))),
            _ => new_instructions.push(mem::replace(op, Op::Clear)),
        }
    }

    new_instructions
}

fn compress_instructions(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Add(n1, offset), Op::Add(n2, offset2)) if offset == offset2 => {
                Ok(Op::Add(n1 + n2, *offset))
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
                    [Op::Add(1 | u8::MAX, 0)] => vec![Op::Set(0)],

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

                            replacement.push(Op::Set(0));
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

fn set_optimization(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Set(n1), Op::Add(n2, 0)) => Ok(Op::Set(n1 + n2)),

            _ => Err((op1, op2)),
        })
        .map(|op| match op {
            Op::Loop(body) => Op::Loop(set_optimization(body)),
            _ => op,
        })
        .collect()
}
