// error-pattern: expected one of `!`, `(`, `::`, `<`, or `=`, found `;`

// This code has several problems, and doesn't compile.
static mut STASH: &i32;
fn f(p: &i32) { STASH = p; }
