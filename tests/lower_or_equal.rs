pub mod support;

use csmlinterpreter::data::{Event, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;
use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, name: &str, step: &str) -> MessageData {
    let file = format!("CSML/basic_test/numerical_operations/{}", name);
    let text = read_file(file).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_array_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_array_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_array_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_array_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_array_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_array_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_array_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_array_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_boolean_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_boolean_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_boolean_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_boolean_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_boolean_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_boolean_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_boolean_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_boolean_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL FLOAT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_float_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_float_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_float_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_float_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_float_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_float_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_float_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_float_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL INT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_int_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_int_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_int_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_int_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_int_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_int_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL NULL
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_null_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_null_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_null_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_null_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_null_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_null_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_null_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_null_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL OBJECT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_object_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_object_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_object_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_object_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_object_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_object_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_object_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_object_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// LOWER_OR_EQUAL STRING
////////////////////////////////////////////////////////////////////////////////

#[test]
fn lower_or_equal_string_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_int_step_0",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_string_step_1() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_string_step_1",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_string_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_string_step_2",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_string_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_string_step_3",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_string_step_4() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_string_step_4",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_string_step_5() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "false"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_string_step_5",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn lower_or_equal_string_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        gen_event(""),
        "lower_or_equal.csml",
        "lower_or_equal_string_step_6",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
