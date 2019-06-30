// The `?` operator checks that the error type matches the function's return type.
// error-pattern: `?` couldn't convert the error to `std::io::Error`

use std::io::{self, BufRead};

/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(file: &mut dyn BufRead) -> Result<Vec<i64>, io::Error> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;         // reading lines can fail
        numbers.push(line.parse()?);     // parsing integers can fail
    }
    Ok(numbers)
}

fn main() {}
