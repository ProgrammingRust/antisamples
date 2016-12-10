// Attempt 2: Did not specify output type of Add or Mul
//
// error-pattern: mismatched types
// error-pattern: mismatched types

use std::ops::{Add, Mul};

fn dot<N: Add + Mul + Default>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn main() {
}
