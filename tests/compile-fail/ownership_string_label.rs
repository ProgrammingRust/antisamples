// `#[derive(Copy)]` does not work on structs with non-copyable fields.

// error-pattern: the trait `Copy` may not be implemented for this type

#![allow(dead_code)]

#[derive(Copy, Clone)]
struct StringLabel { name: String }

fn main() {}
