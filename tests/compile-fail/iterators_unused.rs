// Rust warns about unused iterators.
//
// error-pattern: which must be used: iterator adaptors are lazy and do nothing unless consumed

#[deny(unused_must_use)]
fn main() {
    ["earth", "water", "air", "fire"]
        .iter().map(|elt| println!("{}", elt));
}
