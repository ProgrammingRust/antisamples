// Printing a value requires a bound (for Display or Debug, depending on the format string)

// error-pattern: `<I as std::iter::Iterator>::Item` doesn't implement `std::fmt::Debug`

/// Print out all the values produced by an iterator
fn dump<I>(iter: I)
    where I: Iterator
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);   // error
    }
}

fn main() {}
