// Function calls aren't allowed in static initialiers, take 2.

use std::sync::atomic::{AtomicUsize, Ordering};

static PACKETS_SERVED: AtomicUsize =
    AtomicUsize::new(0);  // error: function call in static
//~^ ERROR: const fns are an unstable feature
//~| HELP: in Nightly builds, add `#![feature(const_fn)]` to the crate attributes to enable

fn main() {
    assert_eq!(PACKETS_SERVED.load(Ordering::SeqCst), 0);
}
