fn main() {
    fn f(p: &'static i32) { assert_eq!(*p, 10); }

    let x = 10;
    f(&x);
    //~^ ERROR: `x` does not live long enough
}
