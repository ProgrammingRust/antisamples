pub struct RefAndBool<'a, T: 'a> {
    //~^ ERROR: parameter `'a` is never used
    //~| ERROR: parameter `T` is never used
    //~| HELP: consider removing `T` or using a marker such as `std::marker::PhantomData`
    //~| HELP: consider removing `'a` or using a marker such as `std::marker::PhantomData`
    ptr_and_bit: usize
}
