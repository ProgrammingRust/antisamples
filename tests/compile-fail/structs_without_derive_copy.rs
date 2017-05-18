// By default, structs are not copyable.
struct Point {
    x: f64,
    y: f64
}

fn f(_p: Point) {}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    f(p);
    f(p);
    //~^ ERROR: use of moved value: `p`
}
