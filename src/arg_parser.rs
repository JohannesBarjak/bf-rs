use std::process;

pub fn process_args(input: &str) {
    if input == "-h" || input == "--help" {
        show_help();
        process::exit(0);
    } else if input == "-v" || input == "--version" {
        println!("bf-rs v0.4.5");
        process::exit(0);
    }
}

fn show_help() {
    println!(
        "Usage:
    bf [file]

    -h, --help        show help
    -v, --version     show bf-rs version"
    );
}
