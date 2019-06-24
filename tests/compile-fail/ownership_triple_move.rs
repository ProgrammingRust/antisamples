// Rust's uninitialized variable analysis is flow-sensitive.

fn f(_: Vec<i32>) { }
fn g(_: Vec<i32>) { }
fn h(_: Vec<i32>) { }

fn main() {
    let c = true;

    let x = vec![10, 20, 30];
    if c {
        f(x); // ... ok to move from x here
    } else {
        g(x); // ... and ok to also move from x here
    }
    h(x) // bad: x is uninitialized here if either path uses it
    //~^ ERROR: use of moved value: `x`
}
