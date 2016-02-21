// error-pattern:arithmetic operation overflowed
fn main() {
    let x = 0x7fffffffi32 + 1;  // panic: arithmetic operation overflowed
}
