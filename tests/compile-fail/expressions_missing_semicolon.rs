// Odd error message when you leave out a semicolon by accident.
// error-pattern: mismatched types

struct Prefs;
impl Prefs {
    fn changed(&self) -> bool { true }
}

struct Page;
impl Page {
    fn compute_size(&mut self) -> (u32, u32) { (0, 0) }
}

fn main() {
    let preferences = Prefs;
    let mut page = Page;

    if preferences.changed() {
        page.compute_size()  // oops, missing semicolon
    }

    println!("done");
}
