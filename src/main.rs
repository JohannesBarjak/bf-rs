use bf::interpreter::Tape;
use bf::{interpreter, optimizer, parser, tokenizer, transpiler};
use clap::clap_app;

use std::fs;
use std::path::Path;

fn main() {
    let matches = clap_app!(bf =>
        (version: "0.9.0")
        (author: "Johannes B. <johannesbarja@protonmail.com>")
        (about: "A brainfuck interpreter")
        (@arg INPUT: +required "Brainfuck program file")
        (@arg interpret: -i "Interpret the program instead of transpiling it")
    )
    .get_matches();

    match matches.value_of("INPUT") {
        Some(file) => {
            let mut instructions =
                parser::parse(&tokenizer::tokenize(&fs::read_to_string(file).unwrap()));

            for _ in 0..2 {
                instructions = optimizer::optimize(instructions);
            }

            if matches.is_present("interpret") {
                interpreter::interpret(&instructions, &mut Tape::new());
            } else {
                fs::write(
                    format!(
                        "{}.c",
                        Path::new(file).file_stem().unwrap().to_str().unwrap()
                    ),
                    transpiler::transpile(instructions),
                )
                .unwrap();
            }
        }

        None => unreachable!(),
    }
}
