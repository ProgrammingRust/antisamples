// Nonempty JSON objects aren't Rust expressions.

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:expr ),* ]) => {
        Json::Array(vec![ $( $element ),* ])
    };
}

fn main() {
    let students = json!([
        {
            "name": "Jim Blandy",
            //~^ ERROR: expected type, found `"Jim Blandy"`
            "class_of": 1931,
            "major": "Tibetan throat singing"
        },
        {
            "name": "Jason Orendorff",
            //~^ ERROR: expected type, found `"Jason Orendorff"`
            "class_of": 1702,
            "major": "Knots"
        }
    ]);
}
