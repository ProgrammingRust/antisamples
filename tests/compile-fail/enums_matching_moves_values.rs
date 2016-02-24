// error-pattern: use of partially moved value: `account`

struct Account { name: String, language: String }
impl Account {
    fn show_hats(&self, ui: UI) {
    }
}

struct UI;
impl UI {
    fn greet(&self, name: &String, language: &String) {
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
            account.show_hats(ui);  // error: use of moved value
        }
    }
}
