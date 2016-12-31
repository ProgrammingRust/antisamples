// The `?` operator checks that the error type matches the function's return type.

// error-pattern: the trait bound `std::io::Error: std::convert::From<std::num::ParseIntError>` is not satisfied
// error-pattern: the trait `std::convert::From<std::num::ParseIntError>` is not implemented for `std::io::Error`

use std::io::{self, BufRead};

/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(file: &mut BufRead) -> Result<Vec<i64>, io::Error> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;         // reading lines can fail
        numbers.push(line.parse()?);     // parsing integers can fail
    }
    Ok(numbers)
}

fn main() {}
