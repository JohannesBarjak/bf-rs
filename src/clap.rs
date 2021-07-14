use clap::{App, Arg};

pub fn get_file() -> String {
    let matches = App::new("bf")
        .version("0.6.3")
        .author("Johannes B. <johannesbarja@protonmail.com>")
        .about("A brainfuck interpreter")
        .arg(
            Arg::new("INPUT")
                .about("Brainfuck program file")
                .required(true)
                .index(1),
        )
        .get_matches();

    matches.value_of("INPUT").unwrap().to_string()
}
