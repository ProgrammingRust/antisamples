struct Point3d(f32, f32, f32);
const ORIGIN: Point3d = Point3d(0.0, 0.0, 0.0);

enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Box { point1: Point3d, point2: Point3d }
}

fn main() {
    let shape = Shape::Sphere { center: ORIGIN, radius: 1.0 };
    let r = shape.radius;  // error: no field `radius` on type `Shape`
    //~^ERROR: no field `radius` on type `Shape`
}
