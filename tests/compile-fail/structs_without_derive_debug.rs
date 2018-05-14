// By default, structs are not printable.
struct Point {
    x: f64,
    y: f64
}

fn f(_p: Point) {}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("{:?}", p);
    //~^ ERROR: `Point` doesn't implement `std::fmt::Debug`
}
