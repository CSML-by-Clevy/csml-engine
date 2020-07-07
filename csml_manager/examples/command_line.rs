use csmlinterpreter::data::{csml_bot::CsmlBot, csml_flow::CsmlFlow, Client};
use csmlrustmanager::{data::*, start_conversation};
use serde_json::{json, Value};
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;

fn get_flow(name: &str) -> Result<String, ::std::io::Error> {
    let file_path = format!("CSML/{}.csml", name);

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Some(
//   "http://localhost:3010/websockets/send/demo_internal?client_id=00f0bd10-5e34-4abc-ae8c-10a6bdacc01d%2523979d8f0d-b30e-4f7e-b95b-72c55c23faf7%2523demo_internal&request_id=04f3df27-3d8c-4154-b7e4-7cade03df222".to_owned()
//),

fn tmp_init_event(event: &str) -> Value {
    json!({
        "request_id": "tmp",
        "client": { "user_id": "alexis", "bot_id": "42", "channel_id": "1" },
        "callback_url": "http://httpbin.org/post",
        "payload": {
            "content_type": "text",
            "content": { "text": event},
            "metadata": { "test": 42},
        },
        "metadata": {},
        "sync": false
    })
}

fn init_data() -> CsmlData {
    CsmlData {
        request_id: "tmp".to_owned(),
        client: Client::new("alexis".to_owned(), "42".to_owned(), "1".to_owned()),
        callback_url: Some("http://httpbin.org/post".to_owned()),
        payload: json!({
            "content_type": "text",
            "content": {
                 "text": "hola"
            },
            "metadata": {
            "title": "title of button",
            "type": "jpg",
            }
        }),
        bot: CsmlBot {
            id: "botid".to_owned(),
            name: "plop".to_owned(),
            fn_endpoint: Some("endpoint".to_owned()),
            flows: vec![
                CsmlFlow {
                    id: "flowid".to_owned(),
                    name: "flow".to_owned(),
                    content: get_flow("flow").expect("error in reading flow"),
                    commands: vec!["/plop".to_owned()],
                },
                CsmlFlow {
                    id: "2".to_owned(),
                    name: "flow2".to_owned(),
                    content: get_flow("flow2").expect("error in reading flow"),
                    commands: vec!["/random".to_owned()],
                },
            ],
            header: serde_json::json!({}),
            default_flow: "flowid".to_owned(),
        },
        metadata: json!({}),
    }
}

fn main() {
    let mut line: String = String::new();
    // let mut context = init_context();
    loop {
        let data = init_data();
        stdin()
            .read_line(&mut line)
            .ok()
            .expect("Failed to read line :)");
        if line.trim().is_empty() {
            continue;
        }
        let event = line.trim().to_owned();
        if event == "exit" {
            break;
        }
        match start_conversation(tmp_init_event(&event), data) {
            Ok(obj) => {
                if obj["conversation_end"].as_bool().unwrap() {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
        line.clear();
    }
}
