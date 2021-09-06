use crate::instructions::Opcode;

pub fn optimize(instructions: &mut Vec<Opcode>) {
    let mut i = 0;

    while i < instructions.len() {
        if let Opcode::Loop(loop_body) = &mut instructions[i] {
            match loop_body[..] {
                [Opcode::Add(1 | -1)] => instructions[i] = Opcode::Clear,
                _ => optimize(loop_body),
            }
        }

        i += 1;
    }
}
