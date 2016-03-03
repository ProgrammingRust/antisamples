// error-pattern: cannot borrow `wave` as immutable because it is also borrowed as mutable

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
}

fn main() {
    let mut wave = vec![0.0, 1.0, 0.0, -1.0];

    extend(&mut wave, &wave);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0,
                          0.0, 1.0, 0.0, -1.0]);
}
