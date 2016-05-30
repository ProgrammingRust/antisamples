// error-pattern: closure may outlive the current function, but it borrows `stat`, which is owned by the current function

struct City {
    name: String,
    population: i64,
    country: String
}

enum Statistic {
    Population
}

impl City {
    fn get_statistic(&self, stat: &Statistic) -> i64 {
        match *stat {
            Statistic::Population => self.population
        }
    }
}

use std::thread;

fn start_sorting_thread(cities: Vec<City>, stat: Statistic)
    -> thread::JoinHandle<Vec<City>>
{
    let key_fn = |city: &City| -> i64 { city.get_statistic(&stat) };

    thread::spawn(|| {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {}
