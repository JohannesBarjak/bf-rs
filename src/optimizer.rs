use fnv::FnvHashMap;
use itertools::Itertools;

use crate::instructions::Op;

pub fn optimize(instructions: Vec<Op>) -> Vec<Op> {
    simplify_code(remove_dead_code(set_optimization(optimize_loops(
        compress_instructions(instructions),
    ))))
}

fn simplify_code(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .map(|op| if op == Op::Set(0) { Op::Clear } else { op })
        .collect()
}

fn remove_dead_code(instructions: Vec<Op>) -> Vec<Op> {
    let instructions = match instructions[..] {
        [Op::Loop(_), ..] => instructions[1..].to_vec(),
        _ => instructions,
    };

    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Loop(_), Op::Loop(_)) => Ok(op1),
            (Op::Loop(_), Op::Set(0)) => Ok(op1),

            (Op::Set(_), Op::Set(n2)) => Ok(Op::Set(*n2)),

            _ => Err((op1, op2)),
        })
        .collect()
}

fn compress_instructions(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .coalesce(|op1, op2| match (&op1, &op2) {
            (Op::Add(n1), Op::Add(n2)) => Ok(Op::Add(n1 + n2)),
            (Op::Move(n1), Op::Move(n2)) => Ok(Op::Move(n1 + n2)),

            _ => Err((op1, op2)),
        })
        .filter_map(|op| match op {
            Op::Loop(body) => Some(Op::Loop(compress_instructions(body))),
            Op::Add(0) | Op::Move(0) => None,

            _ => Some(op),
        })
        .collect()
}

fn optimize_loops(instructions: Vec<Op>) -> Vec<Op> {
    instructions
        .into_iter()
        .flat_map(|op| {
            if let Op::Loop(body) = &op {
                match body[..] {
                    [Op::Add(1 | u8::MAX)] => vec![Op::Set(0)],

                    [Op::Move(step)] => vec![Op::Shift(step)],

                    _ if (&body[..])
                        .iter()
                        .all(|op| matches!(op, Op::Add(_) | Op::Move(_))) =>
                    {
                        let mut offset = 0;
                        let mut tape_map = FnvHashMap::default();

                        for op in body {
                            if let Op::Move(n) = op {
                                offset += n;
                            } else if let Op::Add(mul) = op {
                                tape_map.insert(offset, mul + tape_map.get(&offset).unwrap_or(&0));
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
                            vec![op]
                        }
                    }

                    _ => vec![Op::Loop(optimize_loops(body.clone()))],
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
            (Op::Set(n1), Op::Add(n2)) => Ok(Op::Set(n1 + n2)),

            _ => Err((op1, op2)),
        })
        .map(|op| match op {
            Op::Loop(body) => Op::Loop(set_optimization(body)),
            _ => op,
        })
        .collect()
}
