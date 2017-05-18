// Even if a struct is declared `pub`, its fields can be private.

mod img {
    /// A rectangle of eight-bit grayscale pixels.
    pub struct GrayscaleMap {
        pixels: Vec<u8>,
        size: (usize, usize)
    }
}

use img::GrayscaleMap;

// Other modules can use this struct and any public methods it might have,
// but can't access the private fields by name...
fn width(map: &GrayscaleMap) -> usize {
    map.size.0
    //~^ ERROR: field `size` of struct `img::GrayscaleMap` is private
}

fn main() {}
