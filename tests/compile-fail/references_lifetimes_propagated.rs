// error-pattern: `parabola` does not live long enough

fn smallest(v: &[i32]) -> &i32 {
    v.iter().min().unwrap()
}

fn main() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
    }
    assert_eq!(*s, 0); // bad: points to element of dropped array
}
