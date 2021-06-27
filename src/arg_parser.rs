use clap::{App, Arg};

pub fn process_args() -> String {
    let matches = App::new("bf")
        .version("0.4.5")
        .author("Johannes Barjak. <johannesbarja@protonmail.com>")
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
