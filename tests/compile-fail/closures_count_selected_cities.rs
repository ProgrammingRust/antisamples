// Closures don't have function types.

struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64
}

fn main() {
    struct Prefs;
    impl Prefs {
        fn acceptable_monster_risk(&self) -> f64 { 0.0 }
    }

    let my_cities: Vec<City> = vec![];
    let preferences = Prefs;

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

    let limit = preferences.acceptable_monster_risk();
    let n = count_selected_cities(
        &my_cities,
        |city| city.monster_attack_risk > limit);  // error: type mismatch
    //~^ ERROR: mismatched types
    //~| NOTE: expected fn pointer, found closure
    //~| NOTE: expected type `for<'r> fn(&'r City) -> bool`
}
