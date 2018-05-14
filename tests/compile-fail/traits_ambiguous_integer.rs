// .abs() can't be used on an int literal, because the type can't be inferred.

fn main() {
    let zero = 0;  // type unspecified; could be `i8`, `u8`, ...

    let _ =
        zero.abs();  // error: method `abs` not found
    //~^ ERROR: can't call method `abs` on ambiguous numeric type `{integer}`
}
