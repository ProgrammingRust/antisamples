// In the book, we claim that integers can't be autoconverted to bool.
// Check that this is really true.
//
// error-pattern: mismatched types

fn main() {
    let v =
        if 1 { 2 } else { 3 }
    ;
    println!("{}", v);
}
