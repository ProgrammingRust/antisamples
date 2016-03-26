use std::ops::Add;

fn sum<N: Add<Output=N> + Default>(values: &Vec<N>) -> N {
    let mut total = N::default();
    for v in values {
        total = total + *v;
        //~^ ERROR: cannot move out of borrowed content
    }
    total
}

fn main() {}
