mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{data::*, message::MessageData};

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>) -> MessageData {
    let text = read_file("CSML/foreach.csml".to_owned()).unwrap();

    let context = gen_context(
        serde_json::json!({}),
        serde_json::json!({}),
        serde_json::json!({}),
        0,
        false,
    );

    interpret(&text, "start", context, &event, None, None, None)
}

#[test]
fn ok_foreach() {
    let data = r#"{"messages":[ {"content": { "text": "1" } ,"content_type":"text"}, {"content": { "text": "2" } ,"content_type":"text"}, {"content": { "text": "3" } ,"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None);

    let v1: serde_json::Value = message_to_jsonvalue(msg);
    let v2: serde_json::Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}
