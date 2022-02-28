mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_time() {
    let data =
        r#"
        {"messages":[ 
            {"content":{"text": "true"},"content_type":"text"},
            {"content":{"text": "true"},"content_type":"text"},
            {"content":{"text": "2014-10-20T01:00:00.000Z"},"content_type":"text"},
            {"content":{"text": "2014"},"content_type":"text"}
        ],
        "memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "start", "flow"),
        "CSML/basic_test/built-in/time.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn ok_time_parse_1_args() {
    let data =
        r#"
        {"messages":[ 
            {"content":{"text": "1983-08-13T00:00:00.000Z"},"content_type":"text"}
        ],
        "memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "parse_1_arg", "flow"),
        "CSML/basic_test/built-in/time.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn ok_time_parse_2_args() {
    let data =
        r#"
        {"messages":[ 
            {"content":{"text": "1983-08-13T12:09:14.274Z"},"content_type":"text"}
        ],
        "memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "parse_2_arg", "flow"),
        "CSML/basic_test/built-in/time.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_parse_timezone() {
    let data =
        r#"
        {"messages":[ 
            {"content":{"text": "2014-11-28T21:00:09.000+09:00"},"content_type":"text"}
        ],
        "memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "parse_timezone", "flow"),
        "CSML/basic_test/built-in/time.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}


#[test]
fn ok_with_timezone() {
    let data =
        r#"
        {"messages":[ 
            {"content":{"text": "2014-11-28T21:00:09.000+01:00"},"content_type":"text"}
        ],
        "memories":[]
        }"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(HashMap::new(), HashMap::new(), None, None, "with_timezone", "flow"),
        "CSML/basic_test/built-in/time.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

