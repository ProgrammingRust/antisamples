// Closures that drop values do not implement a `Fn` trait.

// error-pattern: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`

fn call_twice<F>(closure: F) where F: Fn () {
    closure();
    closure();
}

fn main() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);
    call_twice(f);
}
