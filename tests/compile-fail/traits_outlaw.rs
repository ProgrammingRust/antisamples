// Normal method call syntax can be confounded by naming collisions involving traits.

trait Visible {
    fn draw(&self);
}
trait HasPistol {
    fn draw(&self);
}

struct Outlaw {}
impl Visible for Outlaw {
    fn draw(&self) {}
}
impl HasPistol for Outlaw {
    fn draw(&self) {}
}

fn main() {
    let mut outlaw = Outlaw {};

    outlaw.draw();  // error: draw on screen or draw pistol?
    //~^ ERROR: multiple applicable items in scope
}
