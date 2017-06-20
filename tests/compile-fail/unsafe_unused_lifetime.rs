pub struct RefAndBool<'a, T: 'a> {
    //~^ ERROR: parameter `'a` is never used
    //~| ERROR: parameter `T` is never used
    ptr_and_bit: usize
}
