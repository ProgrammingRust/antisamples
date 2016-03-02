// error-pattern: the type `str` cannot be mutably indexed
// error-pattern: no method named `push` found for type `&str`

fn main() {
    let mut s = "hello";
    s[0] = 'c';    // error: the type `str` cannot be mutably indexed
    s.push('\n');  // error: no method named `push` found for type `&str`
}
