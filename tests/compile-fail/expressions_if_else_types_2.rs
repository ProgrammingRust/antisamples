// `if`-expressions that produce a value other than `()` must have an `else` block.

fn is_hockey_season() -> bool { true }

fn main() {
    #[allow(unused_variables)]
    let best_sports_team =
        if is_hockey_season() { "Predators" };  // error
    //~^ ERROR: if may be missing an else clause

    // (The last example is an error because in July,
    // the result would be `()`.)
}

