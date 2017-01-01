// `main()` can't use `?` because its return type is not `Result`.

// error-pattern: the trait bound `(): std::ops::Carrier` is not satisfied

use std::io;

fn calculate_tides() -> io::Result<()> {
    Ok(())
}

fn main() {
    calculate_tides()?;  // error: can't pass the buck any further
}
