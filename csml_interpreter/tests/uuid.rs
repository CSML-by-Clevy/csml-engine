mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_v1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "v1",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/uuid.csml",
    );

    let v: Value = message_to_json_value(msg);

    let _uuid = v["messages"][0]["content"]["text"]
        .as_str()
        .unwrap()
        .parse::<String>()
        .unwrap();
}

#[test]
fn ok_v4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "v4",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/uuid.csml",
    );

    let v: Value = message_to_json_value(msg);

    let _uuid = v["messages"][0]["content"]["text"]
        .as_str()
        .unwrap()
        .parse::<String>()
        .unwrap();
}

#[test]
fn ok_v4_no_arg() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "v4_no_arg",
            "flow",
            None,
        ),
        "CSML/basic_test/built-in/uuid.csml",
    );

    let v: Value = message_to_json_value(msg);

    let _uuid = v["messages"][0]["content"]["text"]
        .as_str()
        .unwrap()
        .parse::<String>()
        .unwrap();
}
