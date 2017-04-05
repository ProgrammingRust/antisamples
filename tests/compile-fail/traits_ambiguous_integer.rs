// .abs() can't be used on an int literal, because the type can't be inferred.

fn main() {
    let zero = 0;  // type unspecified; could be `i8`, `u8`, ...

    let _ =
        zero.abs();  // error: method `abs` not found
    //~^ ERROR: no method named `abs` found for type `{integer}` in the current scope
}
