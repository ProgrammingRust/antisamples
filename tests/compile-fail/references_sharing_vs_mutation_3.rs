// error-pattern: cannot assign to `x` because it is borrowed
// error-pattern: cannot borrow `x` as mutable because it is also borrowed as immutable
// error-pattern: cannot borrow `y` as mutable more than once at a time
// error-pattern: cannot use `y` because it was mutably borrowed

fn main() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;     // ok: multiple shared borrows permitted
    x += 10;         // error: cannot assign to `x` because it is borrowed
    let m = &mut x;  // error: cannot borrow `x` as mutable because it is
                     // also borrowed as immutable

    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y;  // error: cannot borrow as mutable more than once
    let z = y;        // error: cannot use `y` because it was mutably borrowed
}
