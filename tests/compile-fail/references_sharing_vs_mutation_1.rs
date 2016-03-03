// error-pattern: cannot move out of `v` because it is borrowed

fn main() {
    // So far, we've discussed how Rust ensures no reference will ever point to a
    // variable that has gone out of scope. But there are other ways to introduce
    // dangling pointers. Here's an easy case:

    let v = vec![4, 8, 19, 27, 34, 10];
    let r = &v;
    let aside = v;  // move vector to aside
    r[0];           // bad: uses `v`, which is now uninitialized

    // The assignment to `aside` moves the vector, leaving `v` uninitialized, turning
    // `r` into a dangling pointer.
    // 
    // The problem here is not that `v` goes out of scope while `r` still refers to it,
    // but rather that `v`'s value gets moved elsewhere, leaving `v` uninitialized.
    // Naturally, Rust catches the error.
}
