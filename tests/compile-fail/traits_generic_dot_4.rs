use std::ops::{Add, Mul};

fn dot<N>(v1: &[N], v2: &[N]) -> N
    where N: Add<Output=N> + Mul<Output=N> + Default
{
    let mut total = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
        //~^ ERROR: cannot move out of type `[N]`, a non-copy array
        //~| ERROR: cannot move out of type `[N]`, a non-copy array
    }
    total
}

fn main() {}
