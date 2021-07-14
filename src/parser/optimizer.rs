use crate::instructions::Opcode;
use crate::parser::{is_loop_end, is_loop_start};

pub fn optimize(input: &mut Vec<Opcode>) {
    let mut i = 0;

    while i < input.len() - 2 {
        if is_loop_start(&input, &i) && is_loop_end(&input, &(i + 2)) {
            let midpoint = &input[i + 1];

            if *midpoint == Opcode::Substract(1) || *midpoint == Opcode::Add(1) {
                input.splice(i..i + 3, [Opcode::SetToZero].iter().cloned());
            }
        }

        i += 1;
    }
}
