// error-pattern: expected type, found `440.0`

macro_rules! json {
    (null) => {
        Json::Null
    };
    ([ $( $element:expr ),* ]) => {
        Json::Array(vec![ $( $element ),* ])
    };
}


macro_rules! strip_test {
    (#[test] $( $rest:tt )*) => {
        $( $rest )*
    }
}

strip_test! {
    #[test]
    fn json_array_with_json_element() {
        let macro_generated_value = json!(
            [
                // valid JSON that doesn't match `$element:expr`
                {
                    "pitch": 440.0
                }
            ]
        );
        let hand_coded_value =
            Json::Array(vec![
                Json::Object(Box::new(vec![
                    ("pitch".to_string(), Json::Number(440.0))
                ].collect()))
            ]);
        assert_eq!(macro_generated_value, hand_coded_value);
    }
}

fn main() {
    json_array_with_json_element();
}
