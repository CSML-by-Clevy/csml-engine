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
/// EQUAL ARRAY
////////////////////////////////////////////////////////////////////////////////

#[test]
fn equal_array_step_0() {
    let data = r#"{
        "memories":[
        ],
        "messages":[
            {"content":{"text": "true"}, "content_type":"text"}
        ],
        "next_flow":null,
        "next_step":null}"#;
    let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_0");


    let v1: Value = message_to_json_value(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}

#[test]
fn equal_array_step() {
    let vector = vec![
        format_message(gen_event(""), "equal.csml", "equal_array_step_0"),
        format_message(gen_event(""), "equal.csml", "equal_array_step_1"),
        format_message(gen_event(""), "equal.csml", "equal_array_step_2"),
        format_message(gen_event(""), "equal.csml", "equal_array_step_3"),
        format_message(gen_event(""), "equal.csml", "equal_array_step_4"),
        format_message(gen_event(""), "equal.csml", "equal_array_step_5"),
        format_message(gen_event(""), "equal.csml", "equal_array_step_6"),
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


// #[test]
// fn equal_boolean_step() {
//     let vector = vec![
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_0"),
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_1"),
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_2"),
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_3"),
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_4"),
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_5"),
//         format_message(gen_event(""), "equal.csml", "equal_boolean_step_6"),
//     ];

//     for msg in vector.iter() {
//         let value: Value = message_to_json_value(msg.to_owned());

//         if let Some(value) = value.get("messages") {
//             if let Some(value) = value.get(0) {
//                 if let Some(value) = value.get("content_type") {
//                     if value == "error" {
//                         continue;
//                     }
//                 }
//             }
//         }

//         println!("{:#?}", value);

//         return assert!(false);
//     }

//     assert!(true)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// EQUAL FLOAT
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn equal_float_step_0() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_0");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_float_step_1() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_1");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_float_step_2() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_2");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn equal_float_step_3() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_3");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn equal_float_step_4() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_4");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_float_step_5() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_5");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_float_step_6() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_float_step_6");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// EQUAL INT
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn equal_int_step_0() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_0");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_int_step_1() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_1");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_int_step_2() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_2");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn equal_int_step_3() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_3");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn equal_int_step_4() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_4");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_int_step_5() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_5");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_int_step_6() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_int_step_6");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// EQUAL NULL
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn equal_null_step() {
//     let vector = vec![
//         format_message(gen_event(""), "equal.csml", "equal_null_step_0"),
//         format_message(gen_event(""), "equal.csml", "equal_null_step_1"),
//         format_message(gen_event(""), "equal.csml", "equal_null_step_2"),
//         format_message(gen_event(""), "equal.csml", "equal_null_step_3"),
//         format_message(gen_event(""), "equal.csml", "equal_null_step_4"),
//         format_message(gen_event(""), "equal.csml", "equal_null_step_5"),
//         format_message(gen_event(""), "equal.csml", "equal_null_step_6"),
//     ];

//     for msg in vector.iter() {
//         let value: Value = message_to_json_value(msg.to_owned());

//         if let Some(value) = value.get("messages") {
//             if let Some(value) = value.get(0) {
//                 if let Some(value) = value.get("content_type") {
//                     if value == "error" {
//                         continue;
//                     }
//                 }
//             }
//         }

//         println!("{:#?}", value);

//         return assert!(false);
//     }

//     assert!(true)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// EQUAL OBJECT
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn equal_object_step() {
//     let vector = vec![
//         format_message(gen_event(""), "equal.csml", "equal_object_step_0"),
//         format_message(gen_event(""), "equal.csml", "equal_object_step_1"),
//         format_message(gen_event(""), "equal.csml", "equal_object_step_2"),
//         format_message(gen_event(""), "equal.csml", "equal_object_step_3"),
//         format_message(gen_event(""), "equal.csml", "equal_object_step_4"),
//         format_message(gen_event(""), "equal.csml", "equal_object_step_5"),
//         format_message(gen_event(""), "equal.csml", "equal_object_step_6"),
//     ];

//     for msg in vector.iter() {
//         let value: Value = message_to_json_value(msg.to_owned());

//         if let Some(value) = value.get("messages") {
//             if let Some(value) = value.get(0) {
//                 if let Some(value) = value.get("content_type") {
//                     if value == "error" {
//                         continue;
//                     }
//                 }
//             }
//         }

//         println!("{:#?}", value);

//         return assert!(false);
//     }

//     assert!(true)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// EQUAL STRING
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn equal_string_step_0() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_0");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_string_step_1() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_1");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_string_step_2() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_2");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn equal_string_step_3() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_3");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn equal_string_step_4() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_4");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_string_step_5() {
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_5");

//     let value: Value = message_to_json_value(msg.to_owned());

//     if let Some(value) = value.get("messages") {
//         if let Some(value) = value.get(0) {
//             if let Some(value) = value.get("content_type") {
//                 if value == "error" {
//                     return assert!(true);
//                 }
//             }
//         }
//     }

//     println!("{:#?}", value);

//     assert!(false)
// }

// #[test]
// fn equal_string_step_6() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "1"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "equal.csml", "equal_string_step_6");


//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }