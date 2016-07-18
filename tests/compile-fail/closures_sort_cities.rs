// Structs are not ordered by default.

// error-pattern: the trait bound `City: std::cmp::Ord` is not satisfied

struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort();  // error: how do you want them sorted?
}
