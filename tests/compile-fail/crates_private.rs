// Code outside a module can access public items but not private ones.

mod cells {
    pub struct Cell {
        x: f64
    }
}

mod spores {
    use cells::Cell;

    /// A cell made by an adult fern. It disperses on the wind as part of
    /// the fern life cycle. A spore grows into a prothallus -- a whole
    /// separate organism, up to 5mm across -- which produces the zygote
    /// that grows into a new fern. (Plant sex is complicated.)
    pub struct Spore {
        size: f64
    }

    /// Simulate the production of a spore by meiosis.
    pub fn produce_spore(factory: &mut Sporangium) -> Spore {
        Spore { size: 1.0 }
    }

    /// Mix genes to prepare for meiosis (part of interphase).
    fn recombine(parent: &mut Cell) {
    }

    pub struct Sporangium;
}

fn main() {
    let mut factory = spores::Sporangium;
    let cell = cells::Cell { x: 0.0 };

    let s = spores::produce_spore(&mut factory);  // ok

    spores::recombine(&mut cell);  // error: `recombine` is private
    //~^ ERROR: function `recombine` is private
}
