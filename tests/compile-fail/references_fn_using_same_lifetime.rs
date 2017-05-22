// error-pattern: `y` does not live long enough

#![allow(unused_variables, unused_assignments)]

fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 { r } // perhaps too tight

fn main() {
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = f(&x, &y);
            r = s;
        }
    }
}
