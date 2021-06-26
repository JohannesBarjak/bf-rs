use bf::optimizers;
use std::fs;

#[test]
fn input_cleanup_test() {
    assert_eq!(
        fs::read_to_string("tests/Cleaned_Hello2.bf").unwrap(),
        optimizers::cleanup_input(fs::read_to_string("tests/Hello2.bf").unwrap())
    );
}

#[test]
fn brainfuck_optimizer_test() {
    assert_eq!(
        fs::read_to_string("tests/Optimized_Hello2.bf")
            .unwrap()
            .as_bytes()
            .to_vec(),
        optimizers::optimize_brainfuck(
            fs::read_to_string("tests/Cleaned_Hello2.bf")
                .unwrap()
                .as_bytes()
                .to_vec()
        )
    );
}
