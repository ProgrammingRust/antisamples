// error-pattern: cannot borrow `r.1` as mutable, as it is behind a `&` reference

fn main() {
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;         // ok: reborrowing shared as shared
    let m1 = &mut r.1;     // error: can't reborrow shared as mutable
}
