fn main() {
    println!("{}", -100u32);  // error: unary negation of unsigned integer
    //~^ ERROR: 2:20: 2:27: constant evaluation error [E0080]
    //~| NOTE: in this expansion of println!
    //~| NOTE: unary negation of unsigned integer
}
