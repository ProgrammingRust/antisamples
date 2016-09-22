// Function calls aren't allowed in static initialiers.

#[allow(dead_code)]
fn screen_area() -> usize {
    640 * 480
}

const  NBADGERS: usize = 8;                    // ok, 8 is a constant
static NBUNNIES: usize = NBADGERS * 50;        // ok, constant expression
static NCARROTS: usize = screen_area() / 100;  // error: function call in static
//~^ ERROR: calls in statics are limited to constant functions, struct and enum constructors

fn main() {
    assert_eq!(NBADGERS, 8);
    assert_eq!(NBUNNIES, 400);
    assert_eq!(NCARROTS, 64 * 48);
}
