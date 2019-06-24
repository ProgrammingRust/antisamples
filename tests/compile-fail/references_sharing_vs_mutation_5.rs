// error-pattern: cannot use `v.1` because it was mutably borrowed

fn main() {
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;      // ok: reborrowing mutable from mutable
    *m0 = 137;
    let r1 = &m.1;          // ok: reborrowing shared from mutable,
                            // and doesn't overlap with m0
    v.1;                    // error: access through other paths still forbidden
}
