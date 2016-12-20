// "`String`, however, does *not* implement `Write`."

use std::io::prelude::*;

fn main() {
    let mut s = String::new();
    s.write_all(b"hello world");
    //~^ ERROR: no method named `write_all` found for type `std::string::String` in the current scope
}
