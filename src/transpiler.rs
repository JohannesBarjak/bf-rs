use crate::instructions::Opcode;
use crate::MEMORY_SIZE;

#[must_use]
pub fn transpile(instructions: Vec<Opcode>) -> String {
    let mut output = format!(
        "{}{}{}{}",
        "#include<stdio.h>\n\nint main() {\n",
        format!("    char memory[{}] = {{0}};\n", MEMORY_SIZE),
        "    char *ptr = memory;\n",
        format!("    ptr += {};\n\n", MEMORY_SIZE / 2)
    );

    for opcode in instructions {
        match opcode {
            Opcode::Add(n) => output.push_str(format!("    *ptr += {};\n", n as u8).as_str()),
            Opcode::Move(n) => output.push_str(format!("    ptr += {};\n", n).as_str()),

            Opcode::LoopStart(_) => output.push_str("    while(*ptr) {\n"),
            Opcode::LoopEnd(_) => output.push_str("    }\n"),

            Opcode::PrintChar => output.push_str("    putchar(*ptr);\n"),
            Opcode::ReadChar => output.push_str("    *ptr = getchar();\n"),

            Opcode::Clear => output.push_str("    *ptr = 0;\n"),
        }
    }

    output.push_str("}\n");

    output
}
