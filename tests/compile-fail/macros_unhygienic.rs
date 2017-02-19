// This mishmash won't compile.

// error-pattern: the trait bound `Json: std::convert::From<std::boxed::Box<std::collections::HashMap<std::string::String, Json>>>` is not satisfied

use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

fn main() {
    let fields = "Fields, W.C.";
    let role = {
        let mut fields = Box::new(HashMap::new());
        fields.insert("name".to_string(), Json::from("Larson E. Whipsnade"));
        fields.insert("actor".to_string(), Json::from(fields));
        Json::Object(fields)
    };

    let _ = role;
}
