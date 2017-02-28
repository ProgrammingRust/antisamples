// error-pattern: use of partially moved value: `account`

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
            ui.greet(&name, &language);
            ui.show_settings(&account);  // error: use of moved value `account`
        }
    }
}
