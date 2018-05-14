fn main() {
    let is_even = |x: u64| -> bool x % 2 == 0;  // error
    //~^ ERROR: expected one of `!`, `(`, `+`, `::`, `<`, or `{`, found `x`
}
