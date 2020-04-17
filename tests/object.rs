// mod support;

// use csmlinterpreter::data::context::ContextJson;
// use csmlinterpreter::data::event::Event;
// use csmlinterpreter::data::message_data::MessageData;

// use crate::support::tools::format_message;
// use crate::support::tools::message_to_json_value;

// use serde_json::Value;

// fn check_error_component(vec: &MessageData) -> bool {
//     let comp = &vec.messages[0];
//     comp.content_type == "error"
// }

// #[test]
// fn ok_object_step1() {
//     let data = r#"{"messages":[ {"content":{"text":"1"},"content_type":"text"} ],"memories":[]}"#;
//     let msg = format_message(
//         Event::new("payload", "", serde_json::json!({})),
//         ContextJson::new(
//             serde_json::json!({}),
//             serde_json::json!({}),
//             None,
//             None,
//             "step1",
//             "flow",
//         ),
//         "CSML/basic_test/object.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_object_step2() {
//     let data = r#"{"messages":[ {"content":{"text":"4"},"content_type":"text"} ],"memories":[]}"#;
//     let msg = format_message(
//         Event::new("payload", "", serde_json::json!({})),
//         ContextJson::new(
//             serde_json::json!({}),
//             serde_json::json!({}),
//             None,
//             None,
//             "step2",
//             "flow",
//         ),
//         "CSML/basic_test/object.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_object_step3() {
//     let data =
//         r#"{"messages":[ {"content":{"text":"true"},"content_type":"text"} ],"memories":[]}"#;
//     let msg = format_message(
//         Event::new("payload", "", serde_json::json!({})),
//         ContextJson::new(
//             serde_json::json!({}),
//             serde_json::json!({}),
//             None,
//             None,
//             "step3",
//             "flow",
//         ),
//         "CSML/basic_test/object.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_object_step4() {
//     let msg = format_message(
//         Event::new("payload", "", serde_json::json!({})),
//         ContextJson::new(
//             serde_json::json!({}),
//             serde_json::json!({}),
//             None,
//             None,
//             "step4",
//             "flow",
//         ),
//         "CSML/basic_test/object.csml",
//     );
//     let res = check_error_component(&msg);

//     assert_eq!(res, false)
// }

// #[test]
// fn ok_object_step5() {
//     let msg = format_message(
//         Event::new("payload", "", serde_json::json!({})),
//         ContextJson::new(
//             serde_json::json!({}),
//             serde_json::json!({}),
//             None,
//             None,
//             "step5",
//             "flow",
//         ),
//         "CSML/basic_test/object.csml",
//     );
//     let v: Value = message_to_json_value(msg);

//     let int = v["messages"][0]["content"]["text"]
//         .as_str()
//         .unwrap()
//         .parse::<i64>()
//         .unwrap();

//     if int < 1 && int > 5 {
//         panic!("Random fail {}", int);
//     }
// }
