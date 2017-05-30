// Since Vegetables come in all different sizes, you can't have a vector of them.

// error-pattern: `Vegetable + 'static` does not have a constant size

trait Vegetable {}

struct Salad {
    veggies: Vec<Vegetable>  // error: `Vegetable` does not have
                             //        a constant size
}

fn main() {}
