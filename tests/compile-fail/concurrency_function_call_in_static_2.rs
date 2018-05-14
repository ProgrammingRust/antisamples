// (Most) function calls aren't allowed in static initialiers, take 2.

use std::sync::Mutex;

static HOSTNAME: Mutex<String> =
    Mutex::new(String::new());  // error: function call in static
//~^ ERROR: calls in statics are limited to constant functions, struct and enum constructors
//~| ERROR: calls in statics are limited to constant functions, struct and enum constructors

fn main() {
    let guard = HOSTNAME.lock().unwrap();
    assert_eq!(&*guard, "");
}
