// You can't borrow a car and then scrap it for parts.

// error-pattern: cannot move out of a shared reference

struct Engine {}

struct Car {
    engine: Engine
}

static MORTIS: Car = Car {
    engine: Engine {}
};

struct Friend {}

impl Friend {
    fn borrow_car(&self) -> Option<&'static Car> {
        Some(&MORTIS)
    }
}

fn sell<T>(item: T) {}
fn go_to_vegas() {}

fn main() {
    let friend = Friend {};

    match friend.borrow_car() {
        Some(&Car { engine, .. }) =>  // error: can't move out of borrow
            {
                sell(engine);
                go_to_vegas();
            }
        None => {}
    }
}
