// You can't use methods on floating-point values unless they're unambiguously
// `f32` or `f64`.

// error-pattern: no method named `sqrt` found for type `{float}`

fn main() {
    println!("{}", (2.0).sqrt());
}
