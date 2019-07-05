struct Account { name: String, language: String }
struct UI;
impl UI {
    fn greet(&self, name: &String, language: &String) {
    }
    fn show_settings(&self, account: &Account) {
    }
}

fn main() {
    let account = Account {
        name: "Jim Blandy".to_string(),
        language: "Rust".to_string()
    };
    let ui = UI;

    match account {
        Account { name, language, .. } => {
        //~^ NOTE: value moved here
            ui.greet(&name, &language);
            ui.show_settings(&account);  // error: borrow of moved value: `account`
            //~^ ERROR: borrow of moved value: `account`
            //~| NOTE: value borrowed here after partial move
            //~| NOTE: move occurs because `account.language` has type `std::string::String`, which does not implement the `Copy` trait
        }
    }
}
