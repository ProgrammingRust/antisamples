// If the generic function you're calling doesn't have any arguments that provide useful clues,
// you may have to spell out the type parameters.

// error-pattern: type annotations needed

fn main() {
    // calling a generic method collect<C>() that takes no arguments
    let v1 = (0 .. 1000).collect();  // error: can't infer type
    let v2 = (0 .. 1000).collect::<Vec<i32>>(); // ok
}
