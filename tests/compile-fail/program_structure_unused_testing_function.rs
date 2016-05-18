#![deny(dead_code)]
// error-pattern: function is never used: `roughly_equal`

fn roughly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn trig_works() {
    use std::f64::consts::PI;
    assert!(roughly_equal(PI.sin(), 0.0));
}

fn main() {
}
