use crate::instructions::Op;

pub fn optimize(instructions: &mut Vec<Op>) {
    let mut i = 0;

    while i < instructions.len() {
        if let Op::Loop(loop_body) = &mut instructions[i] {
            match loop_body[..] {
                [Op::Add(1 | -1)] => instructions[i] = Op::Clear,
                _ => optimize(loop_body),
            }
        }

        i += 1;
    }
}
