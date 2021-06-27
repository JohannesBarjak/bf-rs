use std::process;

pub fn process_args(args: &[String]) -> String {
    let first = args.get(1).expect("Please provide a argument");
    if first == "-h" || first == "--help" {
        show_help();
        process::exit(0);
    } else if first == "-v" || first == "--version" {
        println!("bf-rs v0.4.5");
        process::exit(0);
    }

    first.to_string()
}

fn show_help() {
    println!(
        "Usage:
    bf [file]

    -h, --help        show help
    -v, --version     show bf-rs version"
    );
}
