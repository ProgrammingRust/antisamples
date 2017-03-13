// error-pattern: unreachable pattern
#![deny(unreachable_patterns)]

mod game {
    pub type Error = &'static str;
    pub type Result<T> = ::std::result::Result<T, Error>;
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Point(i32, i32);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Hex(i32, i32);

fn point_to_hex(Point(x, y): Point) -> Option<Hex> {
    Some(Hex(x, y))
}

fn check_move(current_hex: Hex, click: Point) -> game::Result<Hex> {
    match point_to_hex(click) {
        None =>
            Err("That's not a game space."),
        Some(current_hex) =>  // try to match if user clicked the current_hex
                              // (it doesn't work: see explanation below)
            Err("You are already there! You must click somewhere else."),
        Some(other_hex) =>
            Ok(other_hex)
    }
}

// This fails because identifiers introduce *new* bindings...

fn main() {
    let _ = check_move(Hex(3, 3), Point(0, 9));
}
