mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_random() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "start",
            "flow",
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
