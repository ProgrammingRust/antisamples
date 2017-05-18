// By default, structs do not support the == or != operators.
struct Point {
    x: f64,
    y: f64
}

fn f(_p: Point) {}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    if p == (Point { x: 3.0, y: 4.0 }) {
    //~^ ERROR: binary operation `==` cannot be applied to type `Point`
        println!("ok");
    }
}
