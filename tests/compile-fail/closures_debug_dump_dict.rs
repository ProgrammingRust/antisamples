// Closures can move values.

use std::collections::HashMap;

fn produce_glossary() -> HashMap<String, String> {
    HashMap::new()
}

fn main() {
    let dict = produce_glossary();
    let debug_dump_dict = || {
        for (key, value) in dict {  // oops!
            println!("{:?} - {:?}", key, value);
        }
    };

    debug_dump_dict();
    debug_dump_dict();
    //~^ ERROR: use of moved value: `debug_dump_dict`
}
