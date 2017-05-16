// Rust won't move a non-copyable value to two different places.
// After you move it the first time, it's not there anymore!

// error-pattern: use of moved value: `s`

fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = s;
}
