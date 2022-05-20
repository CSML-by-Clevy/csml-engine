mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_random() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "start",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/random.csml",
    );

    let v: Value = message_to_json_value(msg);

    let float = v["messages"][0]["content"]["text"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    if float < 0.0 || float > 1.0 {
        panic!("Random fail {}", float);
    }
}
