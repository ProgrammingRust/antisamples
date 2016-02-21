fn main() {
    println!("{}", -100u32);  // error: unary negation of unsigned integer
    //~^ ERROR: unary negation of unsigned integer
}
