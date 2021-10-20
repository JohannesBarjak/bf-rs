use crate::instructions::Op;
use crate::MEMORY_SIZE;

#[must_use]
pub fn transpile(instructions: Vec<Op>) -> String {
    let mut output = c_header();

    transpile_instructions(instructions, &mut output);
    output.push_str("}\n");

    output
}

fn c_header() -> String {
    format!(
        "{}{}{}{}",
        "#include<stdio.h>\n\nint main() {\n",
        format!("    char memory[{}] = {{0}};\n", MEMORY_SIZE),
        "    char *ptr = memory;\n",
        format!("    ptr += {};\n\n", MEMORY_SIZE / 2)
    )
}

fn transpile_instructions(instructions: Vec<Op>, output: &mut String) {
    for op in instructions {
        match op {
            Op::Add(n, offset) => {
                output.push_str(format!("    *(ptr + {}) += {};\n", offset, n).as_str())
            }
            Op::Move(n) => output.push_str(format!("    ptr += {};\n", n).as_str()),

            Op::Loop(body) => {
                output.push_str("    while(*ptr) {\n");
                transpile_instructions(body, output);
                output.push_str("    }\n");
            }

            Op::PrintChar(offset) => {
                output.push_str(format!("    putchar(*(ptr + {}));\n", offset).as_str())
            }

            Op::ReadChar(offset) => {
                output.push_str(format!("    *(ptr + {}) = getchar();\n", offset).as_str())
            }

            Op::Clear(offset) => {
                output.push_str(format!("    *(ptr + {}) = 0;\n", offset).as_str())
            }

            Op::Mul(offset, mul) => {
                output.push_str(format!("    *(ptr + {}) += *ptr * {};\n", offset, mul).as_str());
            }

            Op::Set(n, offset) => {
                output.push_str(format!("    *(ptr + {}) = {};\n", offset, n).as_str());
            }

            Op::Shift(n) => {
                output.push_str(format!("    while (*ptr) ptr += {};\n", n).as_str());
            }
        }
    }
}
