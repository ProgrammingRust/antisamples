// Rust does not currently allow function calls in global `static` initializers.
#![feature(const_string_new)]

use std::sync::Mutex;

static HOSTNAME: Mutex<String> =
    Mutex::new(String::new());  // error: function call in static
//~^ ERROR: calls in statics are limited to constant functions, tuple structs and tuple variants

fn main() {}
