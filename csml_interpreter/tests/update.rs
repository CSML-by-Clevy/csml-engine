// mod support;

// use csml_interpreter::data::context::ContextJson;
// use csml_interpreter::data::event::Event;

// use crate::support::tools::format_message;
// use crate::support::tools::message_to_json_value;

// use serde_json::Value;

// #[test]
// fn ok_update_step1() {
//     let data = r#"
//         {
//             "messages":[
//                 {
//                     "content":{"text":"1"},"content_type":"text"
//                 },
//                 {
//                     "content":{"text":"4"},"content_type":"text"
//                 }
//             ],
//             "memories":[]
//         }"#;
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
//         "CSML/basic_test/update.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_update_step2() {
//     let data = r#"{"messages":[ {"content": [{"test": 1}, 2, 3, 4, 5], "content_type":"array"}, {"content": [1, 2, 3, 4, 5], "content_type":"array"} ],"memories":[]}"#;
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
//         "CSML/basic_test/update.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_update_step3() {
//     let data = r#"{"messages":[ {"content": [1], "content_type":"array"}, {"content": [2], "content_type":"array"} ],"memories":[]}"#;
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
//         "CSML/basic_test/update.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_update_step4() {
//     let data = r#"{"messages":[ {"content": [1], "content_type":"array"}, {"content": [1, 2], "content_type":"array"} ],"memories":[]}"#;
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
//         "CSML/basic_test/update.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_update_step5() {
//     let data = r#"{"messages":[ {"content": [1, 2], "content_type":"array"}, {"content": [1], "content_type":"array"} ],"memories":[]}"#;
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
//         "CSML/basic_test/update.csml",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }
