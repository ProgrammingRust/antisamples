fn main() {
    println!("{}", -100u32);  // error: can't apply unary `-` to type `u32`
    //~^ ERROR: 2:20: 2:27: cannot apply unary operator `-` to type `u32`
}
