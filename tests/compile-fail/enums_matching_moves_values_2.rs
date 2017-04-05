// Guards are not allowed on patterns that move stuff.
// error-pattern: cannot bind by-move into a pattern guard

struct Point;
struct Lander;
struct Robot;
impl Robot {
    fn last_known_location(&self) -> Option<Point> { Some(Point) }
}
fn short_distance_strategy(_: Point) {}
fn long_distance_strategy(_: Point) {}
fn searching_strategy() {}

impl Lander {
    fn distance_to(&self, _point: Point) -> u64 { 0 }

    fn plan_robot_recovery(&self, robot: &Robot) {
        match robot.last_known_location() {
            Some(point) if self.distance_to(point) < 10 =>
                short_distance_strategy(point),
            Some(point) =>
                long_distance_strategy(point),
            None =>
                searching_strategy()
        }
    }
}

fn main() {}
