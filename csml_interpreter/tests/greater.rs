mod support;

use csml_interpreter::data::context::Context;
use csml_interpreter::data::event::Event;
use std::collections::HashMap;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////
/// ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_array_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_array_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_array_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_array_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_array_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_array_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_array_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_array_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// BOOLEAN
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_boolean_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_boolean_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_boolean_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_boolean_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_boolean_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_boolean_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_boolean_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_boolean_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// FLOAT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_float_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_float_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_float_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_float_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_float_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_float_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_float_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_float_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// INT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_int_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_int_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_int_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_int_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_int_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_int_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_int_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// NULL
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_null_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_null_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_null_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_null_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_null_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_null_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_null_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_null_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// OBJECT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_object_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_object_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_object_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_object_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_object_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_object_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_object_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_object_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// STRING
////////////////////////////////////////////////////////////////////////////////

#[test]
fn greater_string_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_string_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_string_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_string_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_string_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_string_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn greater_string_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ]}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        Context::new(
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            "greater_string_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/greater.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
