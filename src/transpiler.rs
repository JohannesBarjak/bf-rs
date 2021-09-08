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
    for opcode in instructions {
        match opcode {
            Op::Add(n) => output.push_str(format!("    *ptr += {};\n", n as u8).as_str()),
            Op::Move(n) => output.push_str(format!("    ptr += {};\n", n).as_str()),

            Op::Loop(loop_body) => {
                output.push_str("    while(*ptr) {\n");
                transpile_instructions(loop_body, output);
                output.push_str("    }\n");
            }

            Op::PrintChar => output.push_str("    putchar(*ptr);\n"),
            Op::ReadChar => output.push_str("    *ptr = getchar();\n"),

            Op::Clear => output.push_str("    *ptr = 0;\n"),

            Op::Mul(offset, mul) => {
                output.push_str(format!("    *(ptr + {}) += *ptr * {};\n", offset, mul).as_str())
            }
        }
    }
}
