// `main()` can't use `?` because its return type is not `Result`.

// error-pattern: the `?` operator can only be used in a function that returns `Result`

use std::io;

fn calculate_tides() -> io::Result<()> {
    Ok(())
}

fn main() {
    calculate_tides()?;  // error: can't pass the buck any further
}
