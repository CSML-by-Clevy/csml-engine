// mod support;

// use csml_interpreter::data::event::Event;

// use crate::support::tools::format_message;
// use crate::support::tools::message_to_json_value;

// use serde_json::Value;

// fn format_message(event: Event, step: &str) -> MessageData {
//     let text = read_file("CSML/basic_test/if_statement.csml".to_owned()).unwrap();

//     let mut current = serde_json::map::Map::new();
//     current.insert("var10".to_owned(), serde_json::json!(10));
//     current.insert("var5".to_owned(), serde_json::json!(5));
//     current.insert("bool".to_owned(), serde_json::json!(false));

//     let context = gen_context(serde_json::json!(current), serde_json::json!({}));

//     interpret(&text, step, context, &event, None)
// }

// #[test]
// fn ok_equal_20() {
//     let data = r#"{"messages":[{"content":{"text":"event == 20"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
//     let msg = format_message(gen_event("20"), "start");

//     let msg = format_message(
//         Event::new("payload", "20", serde_json::json!({})),
//         "CSML/basic_test/if_statement.csml",
//         "start",
//     );

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_greater_20() {
//     let data = r#"{"messages":[{"content":{"text":"event > 20 && event < 40"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
//     let msg = format_message(gen_event("22"), "start");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_greater_equal_50() {
//     let data = r#"{"messages":[{"content":{"text":"event >= 50"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
//     let msg = format_message(gen_event("50"), "start");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_less_20() {
//     let data = r#"{"messages":[{"content":{"text":"event < 20"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
//     let msg = format_message(gen_event("4"), "start");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_less_equal_45() {
//     let data = r#"{"messages":[{"content":{"text":"event <= 45"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;
//     let msg = format_message(gen_event("42"), "start");

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_not_int1() {
//     let msg = format_message(gen_event("plop"), "start");
//     let data = r#"{"messages":[{"content":{"text":"event is not int"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_not_int2() {
//     let msg = format_message(gen_event("plop"), "start");
//     let data = r#"{"messages":[{"content":{"text":"event is not int"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_var_to_numeric_comparison() {
//     let msg = format_message(gen_event(""), "step1");
//     let data = r#"{"messages":[{"content":{"text":"var10 > 9"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_var_to_var_comparison() {
//     let msg = format_message(gen_event(""), "step2");
//     let data = r#"{"messages":[{"content":{"text":"var10 > var5"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_var9_to_var9_comparison() {
//     let msg = format_message(gen_event(""), "step3");
//     let data = r#"{"messages":[{"content":{"text":"var9 > var9"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_if_func_in_condition_true() {
//     let msg = format_message(gen_event(""), "step4");
//     let data = r#"{"messages":[{"content":{"text":"quoi"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_if_ident_bool_condition() {
//     let msg = format_message(gen_event(""), "step5");
//     let data = r#"{"messages":[{"content":{"text":"OK"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_if_func_in_condition_false() {
//     let msg = format_message(gen_event(""), "step6");
//     let data = r#"{"messages":[{"content":{"text":"pas OK"},"content_type":"text"}],"next_flow":null,"memories":[],"next_step":"end"}"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }

// #[test]
// fn ok_if_path_argument() {
//     let msg = format_message(gen_event(""), "step7");
//     let data = r#"{
//         "messages":[
//             {"content":{"text":"YES"},"content_type":"text"},
//             {"content":{"text":"YES"},"content_type":"text"}
//         ],
//         "next_flow":null,
//         "memories":[],
//         "next_step":"end"
//     }"#;

//     let v1: Value = message_to_json_value(msg);
//     let v2: Value = serde_json::from_str(data).unwrap();

//     assert_eq!(v1, v2)
// }
