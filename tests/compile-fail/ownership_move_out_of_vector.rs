// You can't move values out of a vector, slice, or array.

// error-pattern: cannot move out of indexed content

#[allow(unused_variables)]
fn main() {
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // Pull out random elements from the vector.
    let third = v[2];
    let fifth = v[4];
}
