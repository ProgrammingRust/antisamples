// Attempt 1: Insufficient bounds on N.
//
// error-pattern: mismatched types
// error-pattern: binary operation `*` cannot be applied to type `N`
// error-pattern: binary operation `+` cannot be applied to type `N`

fn dot<N>(v1: &[N], v2: &[N]) -> N {
    let mut total: N = 0;
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

fn main() {}
