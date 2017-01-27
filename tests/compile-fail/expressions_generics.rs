// `::<` must be used to introduce type parameters in the middle of an expression.

// These incorrect expressions confuse Rust even more than the text of the book
// suggests; hence all the extra errors after "chained comparison operators require parentheses"
mod space_invader_smiley {
    fn new_vec() -> Vec<i32> {
        return Vec<i32>::with_capacity(1000);  // error: something about chained comparisons
        //~^ ERROR: chained comparison operators require parentheses
        //~| ERROR: expected value, found struct `Vec`
        //~| ERROR: expected value, found builtin type `i32`
        //~| ERROR: cannot find function `with_capacity`
        //~| ERROR: mismatched types
        //return Vec::<i32>::with_capacity(1000);  // ok, using ::<
    }

    fn ramp(n: i32) -> Vec<i32> {
        let ramp = (0 .. n).collect<Vec<i32>>();  // same error
        //~^ ERROR: chained comparison operators require parentheses
        //~| ERROR: expected value, found struct `Vec`
        //~| ERROR: expected value, found builtin type `i32`
        //~| ERROR: attempted to take value of method `collect` on type `std::ops::Range<i32>`
        //let ramp = (0 .. n).collect::<Vec<i32>>();  // ok, using ::<
        ramp
        //~^ ERROR: mismatched types
    }
}

fn main() {}
