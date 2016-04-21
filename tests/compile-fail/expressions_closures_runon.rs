fn main() {
    let is_even = |x: u64| -> bool x % 2 == 0;  // error
    //~^ ERROR: expected `{`, found `x`
}
