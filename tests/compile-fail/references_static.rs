// error-pattern: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements

static mut STASH: &'static i32 = &128;
fn f(p: &i32) { // still not good enough
    unsafe {
        STASH = p;
    }
}



#[allow(dead_code)]
mod alternate_wording {
    // The signature of `f` as written
    // above is actually shorthand for the following:
    fn f<'a>(p: &'a i32) { () }
}

fn main() {}
