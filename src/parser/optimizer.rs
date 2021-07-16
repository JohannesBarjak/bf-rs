use crate::instructions::Opcode;

pub fn optimize(input: &mut Vec<Opcode>) {
    let mut i = 0;

    while i < input.len() - 2 {
        if input[i] == Opcode::LoopStartPlaceholder && input[i + 2] == Opcode::LoopEndPlaceholder {
            let midpoint = &input[i + 1];

            if *midpoint == Opcode::Substract(1) || *midpoint == Opcode::Add(1) {
                input.splice(i..i + 3, [Opcode::SetToZero].iter().cloned());
            }
        }

        i += 1;
    }
}
