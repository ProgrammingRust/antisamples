fn error_messages() -> Vec<String> { vec![] }

fn main() {
    let strings: Vec<String> = error_messages();
    for s in strings {                  // each String is moved into s here...
        println!("{}", s);
    }                                   // ...and dropped here
    println!("{} error(s)", strings.len()); // error: use of moved value
    //~^ ERROR: borrow of moved value: `strings`
}
