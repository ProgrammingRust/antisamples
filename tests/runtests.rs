extern crate compiletest_rs as compiletest;
use compiletest::common::Mode;
use std::path::PathBuf;

fn run_tests(mode: Mode, dir: &str) {
    let mut config = compiletest::default_config();
    config.mode = mode;
    config.src_base = PathBuf::from(dir);
    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_tests(Mode::CompileFail, "tests/compile-fail");
    run_tests(Mode::ParseFail, "tests/parse-fail");
    run_tests(Mode::RunFail, "tests/run-fail");
}
