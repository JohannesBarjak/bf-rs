use fnv::FnvHashMap;
use itertools::Itertools;

use crate::instructions::*;

use OffOp::*;
use Op::*;

pub fn optimize(mut instructions: Vec<Op>) -> Vec<Op> {
    let optimizer_pass = |instructions| {
        simplify_code(remove_dead_code(convert_simple_loops(reorder_offsets(
            &calculate_offsets(&compress_instructions(instructions)),
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
            if let Off(offset, Set(0)) = op {
                Off(offset, Clear)
            } else {
                op
            }
        })
        .map(|op| match op {
            Loop(body) => Loop(simplify_code(body)),
            _ => op,
        })
        .collect()
}

fn remove_dead_code(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Loop(_), Loop(_)) | (Loop(_), Off(0, Set(0))) | (Off(0, Set(0)), Loop(_)) => Ok(op1),

            (Off(off, Set(n1)), Off(off2, Add(n2) | Set(n2))) if off == off2 => {
                Ok(if let Off(_, Add(_)) = op2 {
                    Off(*off, Set(n1 + n2))
                } else {
                    Off(*off, Set(*n2))
                })
            }

            _ => Err((op1, op2)),
        })
        .map(|op| match op {
            Loop(body) => Loop(remove_dead_code(body)),
            _ => op,
        })
        .collect()
}

fn reorder_offsets(instructions: &[Op]) -> Vec<Op> {
    let mut new_instructions: Vec<Op> = Vec::new();
    let mut instructions = instructions.iter().peekable();

    let valid_op = |op: &Op| match op {
        Off(off, offop @ (Add(_) | Set(_) | Clear)) => Some((*off, *offop)),
        _ => None,
    };

    while let Some(op) = instructions.next() {
        if let Some(first) = valid_op(op) {
            let mut block = vec![first];

            block.extend(
                // Cannot find a peeking_map_while function to rule this out
                instructions
                    .peeking_take_while(|op| valid_op(op).is_some())
                    .map(|op| valid_op(op).unwrap()),
            );

            block.sort_by(|(off, _), (off2, _)| off.cmp(off2));
            new_instructions.append(&mut block.into_iter().map(|(off, op)| Off(off, op)).collect());
        } else if let Loop(body) = op {
            new_instructions.push(Loop(reorder_offsets(body)));
        } else {
            new_instructions.push(op.clone());
        }
    }

    new_instructions
}

fn calculate_offsets(instructions: &[Op]) -> Vec<Op> {
    let mut new_instructions = Vec::new();
    let mut instructions = instructions.iter().peekable();

    let valid_op = |op: &Op| {
        matches!(
            op,
            Off(_, Add(_) | Set(_) | Clear | PrintChar | ReadChar) | Move(_)
        )
    };

    while let Some(op) = instructions.next() {
        if valid_op(op) {
            let mut block = vec![op];
            let mut new_block = Vec::new();
            let mut offset = 0;

            block.extend(instructions.peeking_take_while(|op| valid_op(op)));

            for op in block {
                match op {
                    Off(off, offop) => new_block.push(Off(off + offset, *offop)),
                    Move(off) => offset += *off,

                    _ => unreachable!(),
                }
            }

            if offset != 0 {
                new_block.push(Move(offset));
            }

            new_instructions.append(&mut new_block);
        } else if let Loop(body) = op {
            new_instructions.push(Loop(calculate_offsets(body)));
        } else {
            new_instructions.push(op.clone());
        }
    }

    new_instructions
}

fn compress_instructions(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Off(off, Add(n1)), Off(off2, Add(n2))) if off == off2 => Ok(Off(*off, Add(n1 + n2))),

            (Move(n1), Move(n2)) => Ok(Move(n1 + n2)),

            _ => Err((op1, op2)),
        })
        .filter_map(|op| match op {
            Loop(body) => Some(Loop(compress_instructions(body))),
            Off(_, Add(0)) | Move(0) => None,

            _ => Some(op),
        })
        .collect()
}

fn convert_simple_loops(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .flat_map(|op| {
            if let Loop(body) = op {
                match body[..] {
                    [Off(off, Add(1 | u8::MAX))] => vec![Off(off, Set(0))],

                    [Move(step)] => vec![Shift(step)],

                    _ if body.iter().all(|op| matches!(op, Off(_, Add(_)) | Move(_))) => {
                        let mut tape_map = FnvHashMap::default();

                        let offset = body.iter().fold(0, |acc, op| match op {
                            Move(n) => acc + n,
                            Off(off, Add(mul)) => {
                                tape_map.insert(
                                    acc + off,
                                    mul + tape_map.get(&(acc + off)).unwrap_or(&0),
                                );
                                acc
                            }
                            _ => acc,
                        });

                        if offset == 0 && tape_map.get(&0) == Some(&u8::MAX) {
                            tape_map.remove(&0);

                            let mut replacement = Vec::new();

                            for (offset, mul) in tape_map {
                                replacement.push(Mul(offset, mul));
                            }

                            replacement.push(Off(0, Set(0)));
                            replacement
                        } else {
                            vec![Loop(body)]
                        }
                    }

                    _ => vec![Loop(convert_simple_loops(body))],
                }
            } else {
                vec![op]
            }
        })
        .collect()
}
