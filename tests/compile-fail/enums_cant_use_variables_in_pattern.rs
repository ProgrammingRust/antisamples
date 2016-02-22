// error-pattern: unreachable pattern

fn print_if_equal(x: i32, y: i32) {
    match x {
        y =>  // trying to match only if x == y
              // (it doesn't work: see explanation below)
            println!("{} == {}", x, y),
        _ =>  // error: unreachable pattern
            println!("not equal")
    }
}

// This fails because identifiers introduce *new* bindings.
// The pattern `y` here creates a new local variable `y`,
// shadowing the argument `y`.

fn main() {
    print_if_equal(2, 4);
}
