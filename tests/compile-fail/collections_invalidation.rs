// Rust rules out iterator invalidation errors at compile time.
// error-pattern: can't borrow `my_vec` as mutable

fn main() {
    let mut my_vec = vec![1, 3, 5, 7, 9];

    for (index, &val) in my_vec.iter().enumerate() {
        if val > 4 {
            my_vec.remove(index);  // error: can't borrow `my_vec` as mutable
        }
    }
    println!("{:?}", my_vec);
}
