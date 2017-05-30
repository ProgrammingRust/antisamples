// error-pattern: `x` does not live long enough

fn main() {
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy
    }
}
