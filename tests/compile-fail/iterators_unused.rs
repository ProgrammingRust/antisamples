// Rust warns about unused iterators.

#[deny(unused_must_use)]
//~^ NOTE: lint level defined here
fn main() {
    ["earth", "water", "air", "fire"]
        .iter().map(|elt| println!("{}", elt));
    //~^^ ERROR: unused `std::iter::Map` that must be used
    //~| NOTE: iterators are lazy and do nothing unless consumed
}
