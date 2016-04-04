// Printing a value requires a bound (for Display or Debug, depending on the format string)
//
// NOTE: The exact error message below appears in the book, so if the compiler
// changes, we should update the book.
//
// error-pattern: the trait `core::fmt::Display` is not implemented for the type `<I as core::iter::Iterator>::Item`


/// Print out all the values produced by an iterator
fn dump<I: Iterator>(iter: I) {
    let mut index: usize = 0;
    for value in iter {
        println!("{}: {}", index, value);   // error
        index += 1;
    }
}
