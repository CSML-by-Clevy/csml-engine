mod support;

use csmlinterpreter::interpret;
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use multimap::MultiMap;
use serde_json::Value;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>, step: &str) -> MessageData {
    let text = read_file("CSML/built-in/random.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(&text).unwrap();

    let memory = gen_context(MultiMap::new(), MultiMap::new(), MultiMap::new(), 0, false);

    interpret(&flow, step, &memory, &event)
}

#[test]
fn ok_random() {
    let msg = format_message(None, "start");

    let v: Value = message_to_jsonvalue(msg);

    let float = v["messages"][0]["content"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    if float < 0.0 || float > 1.0 {
        panic!("Random fail {}", float);
    }
}
