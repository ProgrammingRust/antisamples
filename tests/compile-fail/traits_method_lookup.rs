// There is one unusual rule about trait methods:
// the trait itself must be in scope.
// Otherwise all its methods are hidden.

fn say_hello() -> std::io::Result<()> {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello")?;  // error: no method named `write_all`
    //~^ ERROR: no method named `write_all` found for type `std::vec::Vec<u8>` in the current scope

    Ok(())
}
