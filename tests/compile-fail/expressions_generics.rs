// `::<` must be used to introduce type parameters in the middle of an expression.

// These incorrect expressions confuse Rust even more than the text of the book
// suggests; hence all the extra errors after "chained comparison operators require parentheses"
mod space_invader_smiley {
    fn new_vec() -> Vec<i32> {
        return Vec<i32>::with_capacity(1000);  // error: something about chained comparisons
        //~^ ERROR: chained comparison operators require parentheses
        //~| HELP: use `::<...>` instead of `<...>` if you meant to specify type arguments
        //~| ERROR: `Vec` is the name of a struct or struct variant, but this expression uses it like a function name
        //~| HELP did you mean to write: `Vec { /* fields */ }`?
        //~| ERROR: unresolved name `i32`
        //~| ERROR: unresolved name `with_capacity`
        //~| ERROR: mismatched types
        //return Vec::<i32>::with_capacity(1000);  // ok, using ::<
    }

    fn ramp(n: i32) -> Vec<i32> {
        let ramp = (0 .. n).collect<Vec<i32>>();  // same error
        //~^ ERROR: chained comparison operators require parentheses
        //~| ERROR: `Vec` is the name of a struct or struct variant, but this expression uses it like a function name
        //~| HELP did you mean to write: `Vec { /* fields */ }`?
        //~| ERROR: unresolved name `i32`
        //~| ERROR: attempted to take value of method `collect` on type `std::ops::Range<i32>`
        //~| HELP: maybe a `()` to call it is missing? If not, try an anonymous function
        //let ramp = (0 .. n).collect::<Vec<i32>>();  // ok, using ::<
        ramp
        //~^ ERROR: mismatched types
    }
}

fn main() {}
