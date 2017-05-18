// Even if a struct is declared `pub`, its fields can be private.

mod img {
    /// A rectangle of eight-bit grayscale pixels.
    pub struct GrayscaleMap {
        pixels: Vec<u8>,
        size: (usize, usize)
    }
}

use img::GrayscaleMap;

fn main() {
    // ...or use struct expressions to create new `GrayscaleMap` values.
    let map = GrayscaleMap {
        pixels: vec![0; 120000],
        //~^ ERROR: field `pixels` of struct `img::GrayscaleMap` is private
        size: (400, 300)
        //~^ ERROR: field `size` of struct `img::GrayscaleMap` is private
    };
}
