struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64
}

fn main() {
    let my_cities: Vec<City> = vec![];

    /// Given a list of cities and a test function,
    /// return how many cities pass the test.
    fn count_selected_cities(cities: &Vec<City>,
                             test_fn: fn(&City) -> bool) -> usize
    {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1;
            }
        }
        count
    }

    let n = count_selected_cities(
        &my_cities,
        |city| city.monster_attack_risk > 0.0);  // error: type mismatch
    //~^ ERROR: the type of this value must be known in this context
    //~| ERROR: mismatched types
    //~| NOTE: expected fn pointer, found closure
    //~| NOTE: expected type `fn(&City) -> bool`
    //~| NOTE: found type `[closure
}
