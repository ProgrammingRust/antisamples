// error-pattern: cannot assign to `x` because it is borrowed

fn main() {
    let mut x = 42;         // non-const i32 variable
    let p = &x;             // shared reference to i32
    assert_eq!(*p, 42);
    x += 1;                 // error: cannot assign to x because it is borrowed
    assert_eq!(*p, 42);     // if you take out the assignment, this is true
}
