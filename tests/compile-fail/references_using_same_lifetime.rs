// error-pattern: `y` does not live long enough

struct S<'a> {
    x: &'a i32,
    y: &'a i32
}

fn main() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }
}
