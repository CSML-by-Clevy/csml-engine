mod support;

use csmlinterpreter::data::context::ContextJson;
use csmlinterpreter::data::event::Event;

use crate::support::tools::format_message;
use crate::support::tools::message_to_json_value;

use serde_json::Value;

#[test]
fn ok_addition() {
    let data = r#"{"memories":[], "messages":[{"content":{"text":"5"},"content_type":"text"}]}"#;
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
        "./CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_array_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_array_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_array_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_array_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_array_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_array_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_array_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_array_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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

////////////////////////////////////////////////////////////////////////////////
/// BOOLEAN
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_boolean_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_0",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_boolean_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_boolean_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_boolean_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_boolean_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_boolean_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_boolean_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_boolean_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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

////////////////////////////////////////////////////////////////////////////////
/// FLOAT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_float_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "array_find_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_float_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_float_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_float_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_float_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn addition_float_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_float_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn addition_float_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_float_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_float_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_float_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_float_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_float_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

/////////////////////////////////////////////////////////////////////////////////
/// INT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_int_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "array_find_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_int_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_int_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_int_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_int_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn addition_int_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_int_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn addition_int_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_int_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_int_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_int_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_int_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_int_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

////////////////////////////////////////////////////////////////////////////////
/// NULL
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_null_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "array_find_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_null_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_null_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_null_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_null_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_null_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_null_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_null_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_null_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_null_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_null_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_null_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_null_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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

////////////////////////////////////////////////////////////////////////////////
/// OBJECT
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_object_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "array_find_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_object_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_object_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_object_step_2() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_object_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_object_step_3() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_object_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_object_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_object_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_object_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_object_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_object_step_6() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_object_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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

/////////////////////////////////////////////////////////////////////////////////
/// STRING
////////////////////////////////////////////////////////////////////////////////

#[test]
fn addition_string_step_0() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "array_find_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_string_step_1() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_string_step_1",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_string_step_2() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_string_step_2",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn addition_string_step_3() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_string_step_3",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn addition_string_step_4() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_string_step_4",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_string_step_5() {
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_string_step_5",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

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
fn addition_string_step_6() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "2"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(
        Event::new("payload", "", serde_json::json!({})),
        ContextJson::new(
            serde_json::json!({}),
            serde_json::json!({}),
            None,
            None,
            "addition_string_step_6",
            "flow",
        ),
        "CSML/basic_test/numerical_operation/addition.csml",
    );

    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
