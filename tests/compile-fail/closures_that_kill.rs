// Closures that drop values can't be called more than once.

fn main() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);

    f();  // ok
    f();  // error: use of moved value
    //~^ ERROR: use of moved value
}
