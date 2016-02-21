extern crate compiletest_rs as compiletest;
use compiletest::common::Mode;
use std::path::PathBuf;

#[test]
fn compile_test() {
    let mut config = compiletest::default_config();
    config.mode = Mode::CompileFail;
    config.src_base = PathBuf::from("tests/compile-fail");
    compiletest::run_tests(&config);

    config.mode = Mode::ParseFail;
    config.src_base = PathBuf::from("tests/parse-fail");
    compiletest::run_tests(&config);
}
