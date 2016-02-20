// error-pattern: match arms have incompatible types

enum Pet {
    RedPanda, Buffalo, Orca
}

enum Element {
    Fire, Air, Water, Earth, Time, Strange, Charm, BlackBile, Havers
}

use self::Element::*;

fn main() {
    struct Favorites { element: Element }
    let favorites = Favorites { element: Strange };

    let suggested_pet =
        match favorites.element {
            Fire => Pet::RedPanda,
            Air => Pet::Buffalo,
            Water => Pet::Orca,
            _ => None  // error: incompatible types
        };
}
