// It's easy to get many non-`mut` references into an array, slice, or vector at once:
// Getting multiple `mut` references is not so easy.
//
// error-pattern: cannot borrow `v` as mutable more than once at a time

fn multiple_mut_refs(i: usize, j: usize) {

    let mut v = vec![0, 1, 2, 3];
    let a = &mut v[i];
    let b = &mut v[j];  // error: cannot borrow `v` as mutable
                        //        more than once at a time
}


fn main() {
    multiple_mut_refs(0, 0);
}

