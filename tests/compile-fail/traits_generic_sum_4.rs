use std::ops::Add;

fn sum<N>(values: &Vec<N>) -> N
    where N: Add<Output=N> + Default
{
    let mut total = N::default();
    for v in values {
        total = total + *v;  // error: cannot move `*v` out of borrowed content
        //~^ ERROR: cannot move out of borrowed content
    }
    total
}

fn main() {}
