// A value owned by an `Rc` pointer is immutable.

#[allow(unused_variables)]
fn main() {
    use std::rc::Rc;

    // Rust can infer all these types; written out for clarity
    let s : Rc<String> = Rc::new("shirataki".to_string());
    let t : Rc<String> = s.clone();
    let u : Rc<String> = s.clone();

    s.push_str(" noodles");
    //~^ ERROR: cannot borrow immutable borrowed content as mutable
}

