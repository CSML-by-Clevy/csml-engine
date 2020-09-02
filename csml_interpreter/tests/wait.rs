mod support;

use csml_interpreter::data::context::ContextJson;
use csml_interpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn wait_0() {
    // let data = r#"{"messages":[ {"content":{"error": "Builtin Wait expect one argument of type int or float | example: Wait(3) at line 5, column 6 in step [wait_0] from flow [flow]"},"content_type":"error"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "wait_0",
            "flow",
        ),
        "CSML/basic_test/built-in/wait.csml",
    );

    if msg.messages[0].content_type == "error" {
        return assert!(true);
    }

    assert!(false);

    // let v1: Value = message_to_json_value(msg);
    // let v2: Value = serde_json::from_str(data).unwrap();

    // assert_eq!(v1, v2)
}

#[test]
fn wait_1() {
    let data =
        r#"{"messages":[ {"content":{"duration": 10},"content_type":"wait"} ],"memories":[]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "wait_1",
            "flow",
        ),
        "CSML/basic_test/built-in/wait.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
