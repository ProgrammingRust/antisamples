// You can't take a value out of a field of a struct in a vector.

fn main() {
    struct Person { name: Option<String>, birth: i32 }
    let mut composers = Vec::new();
    composers.push(Person { name: Some("Palestrina".to_string()),
                            birth: 1525 });

    // You can't do this:
    let first_name = composers[0].name;
    //~^ ERROR: cannot move out of index of `std::vec::Vec<main::Person>`

    let _ = first_name;
}
