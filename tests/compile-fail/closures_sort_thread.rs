// You can't just fire off a thread that operates on a data structure stored in
// a local argument binding. That would be wildly unsafe.

// error-pattern: closure may outlive the current function, but it borrows `stat`, which is owned by the current function

struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Statistic {
    Population,
    MonsterAttackRisk
}

impl City {
    fn get_statistic(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::Population => self.population,
            Statistic::MonsterAttackRisk => self.monster_attack_risk as i64
        }
    }
}

use std::thread;

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
    -> thread::JoinHandle<Vec<City>>
{
    let key_fn = |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(|| {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    let jh = start_sorting_thread(vec![], Statistic::MonsterAttackRisk);
    jh.join().unwrap();
}

