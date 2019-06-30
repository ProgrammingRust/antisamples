// Statics can be marked `mut`, but as discussed in Chapter "References and
// borrowing", Rust has no way to enforce its rules about exclusive access on
// `mut` statics.  They are, therefore, inherently non-thread-safe, and safe
// code can't use them at all:

fn main() {
    static mut PACKETS_SERVED: usize = 0;

    println!("{} served", PACKETS_SERVED);  // error: use of mutable static
    //~^ ERROR: use of mutable static is unsafe and requires unsafe function or block
}
