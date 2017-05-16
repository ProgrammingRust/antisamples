// error-pattern: missing lifetime specifier


fn main() {
    // This does not compile.
    struct S {
        r: &i32
    }

    let s;
    {
        let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10); // bad: reads from dropped `x`
}
