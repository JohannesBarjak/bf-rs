use bf::interpreter;
use bf::parser;
use bf::tokenizer;
use bf::transpiler;
use clap::{App, Arg};

use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("bf")
        .version("0.7.0")
        .author("Johannes B. <johannesbarja@protonmail.com>")
        .about("A brainfuck interpreter")
        .arg(
            Arg::new("INPUT")
                .about("Brainfuck program file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("interpret")
                .short('i')
                .about("Interpret the program instead of transpiling it"),
        )
        .get_matches();

    match matches.value_of("INPUT") {
        Some(file) => {
            let instructions =
                parser::parse(&tokenizer::tokenize(&fs::read_to_string(file).unwrap()));

            if matches.is_present("interpret") {
                interpreter::interpret(instructions);
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
