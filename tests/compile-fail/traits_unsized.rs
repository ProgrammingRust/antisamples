// Rust doesn't permit variables of type `Write`.

fn main() {
    use std::io::Write;

    let mut buf: Vec<u8> = vec![];
    let writer: Write = buf;  // error: `Write` does not have a constant size
    //~^ ERROR: mismatched types
    //~| NOTE: expected trait std::io::Write, found struct `std::vec::Vec`
    //~| NOTE: expected type `std::io::Write`
    //~| ERROR: the trait bound `std::io::Write: std::marker::Sized` is not satisfied
    //~| NOTE: `std::io::Write` does not have a constant size known at compile-time
    //~| HELP: the trait `std::marker::Sized` is not implemented for `std::io::Write`
    //~| NOTE: all local variables must have a statically known size

    let _ = writer;
}
