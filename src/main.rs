use clap::{App, Arg};

use std::fs;
use std::path::Path;

use bf::interpreter::Tape;
use bf::{interpreter, optimizer, parser, tokenizer, transpiler};

fn main() {
    let matches = App::new("bf")
        .version("0.9.0")
        .author("Johannes B. <johannesbarja@protonmail.com>")
        .about("A brainfuck interpreter")
        .arg(
            Arg::new("INPUT")
                .required(true)
                .about("Brainfuck program file"),
        )
        .arg(
            Arg::new("interpret")
                .short('i')
                .about("Interpret the program instead of transpiling it"),
        )
        .get_matches();

    match matches.value_of("INPUT") {
        Some(file) => {
            let mut instructions =
                parser::parse(&tokenizer::tokenize(&fs::read_to_string(file).unwrap()));

            instructions = optimizer::optimize(instructions);

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
