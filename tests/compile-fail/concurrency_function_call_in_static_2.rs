// Function calls aren't allowed in static initialiers, take 2.

use std::sync::atomic::{AtomicUsize, Ordering};

static PACKETS_SERVED: AtomicUsize =
    AtomicUsize::new(0);  // error: function call in static
//~^ ERROR: `std::sync::atomic::AtomicUsize::new` is not yet stable as a const fn
//~| HELP: in Nightly builds, add `#![feature(const_atomic_usize_new)]` to the crate attributes to enable

fn main() {
    assert_eq!(PACKETS_SERVED.load(Ordering::SeqCst), 0);
}
