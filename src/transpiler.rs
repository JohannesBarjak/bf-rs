use crate::instructions::Op;
use crate::MEMORY_SIZE;

use Op::*;

#[must_use]
pub fn transpile(instructions: Vec<Op>) -> String {
    let mut gen_code = c_header();

    transpile_instructions(instructions, &mut gen_code);
    gen_code
}

fn c_header() -> String {
    format!(
        "{}{}{}{}{}{}{}{}",
        "#include<stdio.h>\n\nint main() {\n",
        "    char memory[",
        MEMORY_SIZE,
        "] = {0};\n",
        "    char *ptr = memory;\n",
        "    ptr += ",
        MEMORY_SIZE / 2,
        ";\n\n",
    )
}

fn transpile_instructions(instructions: Vec<Op>, gen_code: &mut String) {
    for op in instructions {
        match op {
            Add(n, offset) => add_line(gen_code, format!("*(ptr + {}) += {};", offset, n)),
            Move(n) => add_line(gen_code, format!("ptr += {};", n)),

            Loop(body) => {
                add_line(gen_code, "while(*ptr) {".to_owned());
                transpile_instructions(body, gen_code);
            }

            PrintChar(offset) => add_line(gen_code, format!("putchar(*(ptr + {}));", offset)),
            ReadChar(offset) => add_line(gen_code, format!("*(ptr + {}) = getchar();", offset)),

            Clear(offset) => add_line(gen_code, format!("*(ptr + {}) = 0;", offset)),

            Mul(offset, mul) => {
                add_line(gen_code, format!("*(ptr + {}) += *ptr * {};", offset, mul))
            }

            Set(n, offset) => add_line(gen_code, format!("*(ptr + {}) = {};", offset, n)),
            Shift(n) => add_line(gen_code, format!("while (*ptr) ptr += {};", n)),
        }
    }

    add_line(gen_code, "}".to_owned());
}

fn add_line(gen_code: &mut String, line: String) {
    gen_code.push_str(format!("    {}\n", line).as_str());
}
