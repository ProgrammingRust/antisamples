use std::ops::{Add, Mul};

fn dot<N: Add<Output=N> + Mul<Output=N> + Default>(v1: &[N], v2: &[N]) -> N
{
    let mut total = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
        //~^ ERROR: cannot move out of type `[N]`, a non-copy slice
        //~| NOTE: cannot move out of here
        //~| NOTE: move occurs because `v1[_]` has type `N`, which does not implement the `Copy` trait
        //~| ERROR: cannot move out of type `[N]`, a non-copy slice
        //~| NOTE: cannot move out of here
        //~| NOTE: move occurs because `v2[_]` has type `N`, which does not implement the `Copy` trait
    }
    total
}

fn main() {}
