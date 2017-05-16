// error-pattern: expected lifetime parameter

struct S<'a> {
    r: &'a i32
}

struct T {
    s: S  // not adequate
}
