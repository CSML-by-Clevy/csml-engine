// pub mod support;

// use csmlinterpreter::data::{Event, Message, MessageData};
// use csmlinterpreter::interpret;
// use serde_json::Value;
// use support::tools::{gen_context, gen_event, message_to_json_value, read_file};

// fn format_message(event: Event, name: &str, step: &str) -> MessageData {
//     let file = format!("CSML/basic_test/{}", name);
//     let text = read_file(file).unwrap();

//     let context = gen_context(serde_json::json!({}), serde_json::json!({}));

//     interpret(&text, step, context, &event, None)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// ADDITION ARRAY
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_array_step() {
//     let vector = vec![
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_0"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_1"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_2"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_3"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_4"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_5"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_array_step_6"),
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
// /// ADDITION BOOLEAN
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_boolean_step() {
//     let vector = vec![
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_0"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_1"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_2"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_3"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_4"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_5"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_boolean_step_6"),
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
// /// ADDITION FLOAT
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_float_step_0() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_0");

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
// fn addition_float_step_1() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_1");

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
// fn addition_float_step_2() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_2");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn addition_float_step_3() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_3");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn addition_float_step_4() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_4");

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
// fn addition_float_step_5() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_5");

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
// fn addition_float_step_6() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_float_step_6");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// ADDITION INT
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_int_step_0() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_0");

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
// fn addition_int_step_1() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_1");

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
// fn addition_int_step_2() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_2");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn addition_int_step_3() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_3");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn addition_int_step_4() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_4");

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
// fn addition_int_step_5() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_5");

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
// fn addition_int_step_6() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_int_step_6");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// ADDITION NULL
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_null_step() {
//     let vector = vec![
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_0"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_1"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_2"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_3"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_4"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_5"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_null_step_6"),
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
// /// ADDITION OBJECT
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_object_step() {
//     let vector = vec![
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_0"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_1"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_2"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_3"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_4"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_5"),
//         format_message(gen_event(""), "numerical_operation.csml", "addition_object_step_6"),
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
// /// ADDITION STRING
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn addition_string_step_0() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_0");

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
// fn addition_string_step_1() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_1");

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
// fn addition_string_step_2() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_2");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn addition_string_step_3() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_3");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn addition_string_step_4() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_4");

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
// fn addition_string_step_5() {
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_5");

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
// fn addition_string_step_6() {
//     let data = r#"{
//         "memories":[
//         ],
//         "messages":[
//             {"content":{"text": "2"}, "content_type":"text"}
//         ],
//         "next_flow":null,
//         "next_step":null}"#;
//     let msg = format_message(gen_event(""), "numerical_operation.csml", "addition_string_step_6");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// ////////////////////////////////////////////////////////////////////////////////
// /// AND ARRAY
// ////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn and_array_step() {
//     let vector = vec![
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_0"),
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_1"),
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_2"),
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_3"),
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_4"),
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_5"),
//         format_message(gen_event(""), "numerical_operation.csml", "and_array_step_6"),
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

// // #[test]
// // fn ok_multiplication() {
// //     let data = r#"{"messages":[ {"content":{"text":"8"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
// //     let msg = format_message(gen_event(""), "multiplication.csml", "start");

// //     let v1: Value = message_to_json_value(msg);
// //     let v2: Value = serde_json::from_str(data).unwrap();

// //     assert_eq!(v1, v2)
// // }

// // #[test]
// // fn ok_remainder() {
// //     let data = r#"{"messages":[ {"content":{"text":"2"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
// //     let msg = format_message(gen_event(""), "remainder.csml", "start");

// //     let v1: Value = message_to_json_value(msg);
// //     let v2: Value = serde_json::from_str(data).unwrap();

// //     assert_eq!(v1, v2)
// // }

// // #[test]
// // fn ok_string_to_numeric() {
// //     let data = r#"{"messages":[ {"content":{"text":"2.5"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
// //     let msg = format_message(gen_event(""), "string_to_numeric.csml", "start");

// //     let v1: Value = message_to_json_value(msg);
// //     let v2: Value = serde_json::from_str(data).unwrap();

// //     assert_eq!(v1, v2)
// // }
