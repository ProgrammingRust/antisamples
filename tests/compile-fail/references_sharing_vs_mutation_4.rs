// error-pattern: cannot borrow field `r.1` of immutable binding as mutable

fn main() {
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;         // okay: reborrowing shared as shared
    let m1 = &mut r.1;     // error: can't reborrow shared as mutable
}
