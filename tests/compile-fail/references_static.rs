// error-pattern: explicit lifetime required in the type of `p`

static mut STASH: &i32 = &128;
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
