// `Rc<String>` can't be shared across threads because it's not `Send`.
// error-pattern: `std::rc::Rc<std::string::String>` cannot be sent between threads safely

use std::thread::spawn;
use std::rc::Rc;

fn main() {
    let rc1 = Rc::new("hello threads".to_string());
    let rc2 = rc1.clone();
    spawn(move || {  // error
        rc2.clone();
    });
    rc1.clone();
}
