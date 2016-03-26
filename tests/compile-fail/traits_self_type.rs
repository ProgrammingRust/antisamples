// Traits that use the Self type are incompatible with trait objects.

// error-pattern: the trait `Spliceable` cannot be made into an object

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

struct CherryTree;
struct Mammoth;

impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self { CherryTree }
}

impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self { Mammoth }
}

// error: the trait `Spliceable` cannot be made into an object
fn splice_anything(left: &Spliceable, right: &Spliceable) {
    let combo = left.splice(right);
    drop(combo);
}

fn main() {
    splice_anything(&Mammoth, &CherryTree);
}
