#![deny(dead_code)]
// error-pattern: function is never used: `roughly_equal`

fn roughly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn trig_works() {
    assert!(roughly_equal(std::f64::consts::PI.cos(), -1.0));
}

fn main() {
}
