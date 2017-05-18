// By default, structs are not cloneable.
struct Point {
    x: f64,
    y: f64
}

fn f(_p: Point) {}

fn main() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = p1.clone();
    //~^ ERROR: no method named `clone` found for type `Point` in the current scope
    f(p1);
    f(p2);
}
