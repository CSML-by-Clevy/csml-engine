pub mod support;

use csmlinterpreter::data::{Event, Message, MessageData};
use csmlinterpreter::interpret;
use serde_json::Value;
use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

fn format_message(event: Event, name: &str, step: &str) -> MessageData {
    let file = format!("CSML/basic_test/numerical_operations/{}", name);
    let text = read_file(file).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    interpret(&text, step, context, &event, None)
}

#[test]
fn ok_division() {
    let data = r#"{"messages":[ {"content":{"text":"2"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "division.csml", "start");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn ok_division_2() {
    let data = r#"{"messages":[ {"content":{"text":"21.333333333333332"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(gen_event(""), "division.csml", "div2");

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

fn check_error_component(vec: &[Message]) -> bool {
    let comp = &vec[0];

    return comp.content.is_object();
}

#[test]
fn ok_division_3() {
    let file = format!("CSML/basic_test/numerical_operations/{}", "division.csml");
    let text = read_file(file).unwrap();

    let context = gen_context(serde_json::json!({}), serde_json::json!({}));

    match &interpret(&text, "div3", context, &gen_event(""), None) {
        MessageData {
            memories: None,
            messages: vec,
            next_flow: None,
            next_step: None,
            hold: None,
            ..
        } if vec.len() == 1 && check_error_component(&vec) => {}
        e => panic!("Error in div by 0 {:?}", e),
    }
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_array_step() {
    let vector = vec![
        format_message(gen_event(""), "division.csml", "division_array_step_0"),
        format_message(gen_event(""), "division.csml", "division_array_step_1"),
        format_message(gen_event(""), "division.csml", "division_array_step_2"),
        format_message(gen_event(""), "division.csml", "division_array_step_3"),
        format_message(gen_event(""), "division.csml", "division_array_step_4"),
        format_message(gen_event(""), "division.csml", "division_array_step_5"),
        format_message(gen_event(""), "division.csml", "division_array_step_6"),
    ];

    for msg in vector.iter() {
        let value: Value = message_to_json_value(msg.to_owned());

        if let Some(value) = value.get("messages") {
            if let Some(value) = value.get(0) {
                if let Some(value) = value.get("content_type") {
                    if value == "error" {
                        continue;
                    }
                }
            }
        }

        println!("{:#?}", value);

        return assert!(false);
    }

    assert!(true)
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION BOOLEAN
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_boolean_step() {
    let vector = vec![
        format_message(gen_event(""), "division.csml", "division_boolean_step_0"),
        format_message(gen_event(""), "division.csml", "division_boolean_step_1"),
        format_message(gen_event(""), "division.csml", "division_boolean_step_2"),
        format_message(gen_event(""), "division.csml", "division_boolean_step_3"),
        format_message(gen_event(""), "division.csml", "division_boolean_step_4"),
        format_message(gen_event(""), "division.csml", "division_boolean_step_5"),
        format_message(gen_event(""), "division.csml", "division_boolean_step_6"),
    ];

    for msg in vector.iter() {
        let value: Value = message_to_json_value(msg.to_owned());

        if let Some(value) = value.get("messages") {
            if let Some(value) = value.get(0) {
                if let Some(value) = value.get("content_type") {
                    if value == "error" {
                        continue;
                    }
                }
            }
        }

        println!("{:#?}", value);

        return assert!(false);
    }

    assert!(true)
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION FLOAT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_float_step_0() {
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_0");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_float_step_1() {
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_1");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_float_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_2");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn division_float_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_3");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn division_float_step_4() {
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_4");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_float_step_5() {
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_5");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_float_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_float_step_6");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION INT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_int_step_0() {
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_0");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_int_step_1() {
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_1");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_2");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn division_int_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_3");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn division_int_step_4() {
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_4");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_int_step_5() {
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_5");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_int_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_int_step_6");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION NULL
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_null_step() {
    let vector = vec![
        format_message(gen_event(""), "division.csml", "division_null_step_0"),
        format_message(gen_event(""), "division.csml", "division_null_step_1"),
        format_message(gen_event(""), "division.csml", "division_null_step_2"),
        format_message(gen_event(""), "division.csml", "division_null_step_3"),
        format_message(gen_event(""), "division.csml", "division_null_step_4"),
        format_message(gen_event(""), "division.csml", "division_null_step_5"),
        format_message(gen_event(""), "division.csml", "division_null_step_6"),
    ];

    for msg in vector.iter() {
        let value: Value = message_to_json_value(msg.to_owned());

        if let Some(value) = value.get("messages") {
            if let Some(value) = value.get(0) {
                if let Some(value) = value.get("content_type") {
                    if value == "error" {
                        continue;
                    }
                }
            }
        }

        println!("{:#?}", value);

        return assert!(false);
    }

    assert!(true)
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION OBJECT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_object_step() {
    let vector = vec![
        format_message(gen_event(""), "division.csml", "division_object_step_0"),
        format_message(gen_event(""), "division.csml", "division_object_step_1"),
        format_message(gen_event(""), "division.csml", "division_object_step_2"),
        format_message(gen_event(""), "division.csml", "division_object_step_3"),
        format_message(gen_event(""), "division.csml", "division_object_step_4"),
        format_message(gen_event(""), "division.csml", "division_object_step_5"),
        format_message(gen_event(""), "division.csml", "division_object_step_6"),
    ];

    for msg in vector.iter() {
        let value: Value = message_to_json_value(msg.to_owned());

        if let Some(value) = value.get("messages") {
            if let Some(value) = value.get(0) {
                if let Some(value) = value.get("content_type") {
                    if value == "error" {
                        continue;
                    }
                }
            }
        }

        println!("{:#?}", value);

        return assert!(false);
    }

    assert!(true)
}

////////////////////////////////////////////////////////////////////////////////
/// DIVISION STRING
////////////////////////////////////////////////////////////////////////////////

#[test]
fn division_string_step_0() {
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_0");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_string_step_1() {
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_1");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_string_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_2");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn division_string_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_3");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn division_string_step_4() {
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_4");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_string_step_5() {
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_5");

    let value: Value = message_to_json_value(msg.to_owned());

    if let Some(value) = value.get("messages") {
        if let Some(value) = value.get(0) {
            if let Some(value) = value.get("content_type") {
                if value == "error" {
                    return assert!(true);
                }
            }
        }
    }

    println!("{:#?}", value);

    assert!(false)
}

#[test]
fn division_string_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "1"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "division.csml", "division_string_step_6");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}