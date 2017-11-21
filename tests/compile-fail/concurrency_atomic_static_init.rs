// Rust does not currently allow function calls in global `static` initializers.

use std::sync::Mutex;

static HOSTNAME: Mutex<String> =
    Mutex::new(String::new());  // error: function call in static
//~^ ERROR: calls in statics are limited to constant functions, struct and enum constructors
//~| ERROR: calls in statics are limited to constant functions, struct and enum constructors

fn main() {}
