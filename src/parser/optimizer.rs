use crate::opcodes::*;

pub fn optimize_input(mut input: Vec<OpKind>) -> Vec<OpKind> {
    let mut i = 0;

    while i < input.len() - 2 {
        if is_loop_start(&input, &i) && is_loop_end(&input, &(i + 2)) {
            let midpoint = &input[i + 1];

            if *midpoint == OpKind::Substract(1) || *midpoint == OpKind::Add(1) {
                input.splice(i..i + 3, [OpKind::SetToZero].iter().cloned());
            }
        }

        i += 1;
    }

    input
}
