// Illustration of the problem of collecting `BufRead::lines()`.

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // ok, but not what you want
    let results: Vec<io::Result<String>> = reader.lines().collect();

    // error: can't convert collection of Results to Vec<String>
    let lines: Vec<String> = reader.lines().collect();
    //~^ ERROR: the trait bound `std::vec::Vec<std::string::String>: std::iter::FromIterator<std::result::Result<std::string::String, std::io::Error>>` is not satisfied
}
