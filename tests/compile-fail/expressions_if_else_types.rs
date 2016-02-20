enum Pet {
    Buzzard, Hyena, Amoeba, Programmer
}

struct User;

impl User {
    fn is_hobbit(&self) -> bool { true }
}

fn main() {
    let with_wings = true;
    let user = User;

    let suggested_pet =
        if with_wings { Pet::Buzzard } else { Pet::Hyena };  // ok

    let favorite_number =
        if user.is_hobbit() { "eleventy-one" } else { 9 };  // error
    //~^ ERROR: if and else have incompatible types
}
