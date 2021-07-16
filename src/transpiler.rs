use crate::instructions::Opcode;
use crate::MEMORY_SIZE;

pub fn transpile(input: Vec<Opcode>) -> String {
    let mut output = format!(
        "#include<stdio.h>\n\nint main() {{
    char memory[{}] = {{0}};
    char *ptr = memory;
    ptr += {};\n\n",
        MEMORY_SIZE,
        MEMORY_SIZE / 2
    );

    for opcode in input {
        match opcode {
            Opcode::Add(n) => output.push_str(format!("    *ptr += {};\n", n).as_str()),
            Opcode::Substract(n) => output.push_str(format!("    *ptr -= {};\n", n).as_str()),

            Opcode::MovePtrRight(n) => output.push_str(format!("    ptr += {};\n", n).as_str()),
            Opcode::MovePtrLeft(n) => output.push_str(format!("    ptr -= {};\n", n).as_str()),

            Opcode::LoopStart { .. } => output.push_str("    while(*ptr) {\n"),
            Opcode::LoopEnd { .. } => output.push_str("    }\n"),

            Opcode::PrintChar => output.push_str("    putchar(*ptr);\n"),
            Opcode::ReadChar => output.push_str("    *ptr = getchar();\n"),

            Opcode::SetToZero => output.push_str("    *ptr = 0;\n"),

            _ => unreachable!(),
        }
    }

    output.push_str("}\n");

    output
}
