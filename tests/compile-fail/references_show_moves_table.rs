// Passing a non-copyable value to a non-reference parameter moves it.

use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    //...
}

fn main() {
    let mut table = Table::new();
    show(table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    //~^ ERROR: use of moved value: `table`
}
