// error-pattern: wrong number of lifetime parameters: expected 1, found 0

struct S<'a> {
    r: &'a i32
}

struct T {
    s: S  // not adequate
}
