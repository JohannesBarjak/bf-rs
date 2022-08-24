use crate::instructions::*;
use crate::MEMORY_SIZE;

use OffOp::*;
use Op::*;

#[must_use]
pub fn transpile(instructions: &[Op]) -> String {
    _transpile(instructions, c_header())
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

fn _transpile(instructions: &[Op], gen_code: String) -> String {
    instructions.iter().fold(gen_code, |acc, op| {
        add_line(
            acc,
            match op {
                Off(off, Add(n)) => format!("*(ptr + {}) += {};", off, n),
                Move(n) => format!("ptr += {};", n),

                Loop(body) => {
                    "while(*ptr) {\n".to_owned() + _transpile(body, String::new()).as_str()
                }

                Off(off, PrintChar) => format!("putchar(*(ptr + {}));", off),
                Off(off, ReadChar) => format!("*(ptr + {}) = getchar();", off),

                Off(off, Clear) => format!("*(ptr + {}) = 0;", off),

                Mul(offset, mul) => format!("*(ptr + {}) += *ptr * {};", offset, mul),

                Off(off, Set(n)) => format!("*(ptr + {}) = {};", off, n),
                Shift(n) => format!("while (*ptr) ptr += {};", n),
            },
        )
    }) + "    }"
}

fn add_line(code: String, line: String) -> String {
    code + format!("    {}\n", line).as_str()
}
