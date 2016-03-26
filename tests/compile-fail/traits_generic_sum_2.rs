// Attempt 2: Did not specify output type of Add
//
// error-pattern: mismatched types

use std::ops::Add;

fn sum<N: Add + Default>(values: &Vec<N>) -> N {
    let mut total = N::default();
    for v in values {
        total = total + *v;
    }
    total
}

fn main() {
}
