// Rust's uninitialized variable analysis knows about loops.

// error-pattern: use of moved value: `x`


fn f() -> bool { true }
fn g(_: Vec<i32>) {}

fn main() {
    let x = vec![10, 20, 30];
    while f() {
        g(x); // bad: after first iteration, x is uninitialized
    }
}
