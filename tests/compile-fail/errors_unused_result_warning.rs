// The Rust compiler warns about unused `Result` values.

// error-pattern: unused result which must be used

use std::io::{Write, stderr};

#[deny(unused_must_use)]
fn main() {
    let err = "bleen".parse::<u64>().unwrap_err();
    writeln!(stderr(), "error: {}", err);  // warning: unused result
}
