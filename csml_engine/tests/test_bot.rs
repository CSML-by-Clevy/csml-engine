use csml_engine::{
    data::{BotOpt, CsmlRequest},
    start_conversation,
};
use csml_interpreter::data::{csml_bot::CsmlBot, csml_flow::CsmlFlow, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FlowInfo {
    name: String,
    description: Option<String>,
    commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BotInfo {
    id: String,
    name: String,
    description: Option<String>,
    default_flow: String,
    flows: Vec<FlowInfo>,

    files: Vec<String>,
    functions: Vec<String>,
    apps: Vec<String>,
}

fn get_commands(name: &str, bot_info: &BotInfo) -> Vec<String> {
    for flow in &bot_info.flows {
        if flow.name == name {
            return flow.commands.to_owned();
        }
    }

    vec![]
}

fn init_bot(bot_name: &str) -> Result<CsmlBot, std::io::Error> {
    let mut bot_flows = vec![];

    let tmp = format!("CSML/test/{}/flows", bot_name);
    let flows_path = Path::new(&tmp);
    let mut file = fs::File::open(&format!("CSML/test/{}/bot.json", bot_name))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let bot_info: BotInfo = serde_json::from_str(&contents).unwrap();

    for flow in fs::read_dir(flows_path)? {
        let flow = flow?;

        let mut flow_file = fs::File::open(flow.path().to_str().unwrap())?;
        let mut flow_content = String::new();
        flow_file.read_to_string(&mut flow_content)?;

        let name = flow
            .file_name()
            .to_str()
            .unwrap()
            .trim_end_matches(".csml")
            .to_owned();
        let commands = get_commands(&name, &bot_info);
        bot_flows.push(CsmlFlow {
            id: name.to_owned(),
            name: name.to_owned(),
            commands,
            content: flow_content,
        });
    }

    let bot = CsmlBot {
        id: bot_info.name.clone(),
        name: bot_info.name.clone(),
        fn_endpoint: None,
        flows: bot_flows,
        native_components: None,
        custom_components: None,
        default_flow: bot_info.default_flow.clone(),
        bot_ast: None,
        no_interruption_delay: None,
        env: Some(serde_json::json!({
            "random": "value",
            "toto": "key",
        })),
    };

    Ok(bot)
}

fn init_request(string: &str, bot_id: String, channel_id: String) -> CsmlRequest {
    CsmlRequest {
        request_id: "tmp".to_owned(),
        client: Client {
            user_id: "test".to_owned(),
            bot_id,
            channel_id,
        },
        callback_url: Some("http://httpbin.org/post".to_owned()),
        payload: json!({
            "content_type": "text",
            "content": { "text": string},
        }),
        metadata: json!({"some": "custom-value"}),
    }
}

#[test]
fn ok_test_hold() {
    let bot = init_bot("goto_flow").unwrap();

    let events = &["start", "hold", "event1", "event2", "event3"];

    let messages = &[
        "start",
        "hold",
        "start[1]",
        "end[1]:event1",
        "start[2]",
        "end[2]:event2",
        "start[3]",
        "end[3]:event3",
    ];

    let channel_id = Uuid::new_v4().to_string();
    let bot_id = match std::env::var("GITHUB_SHA") {
        Ok(mut value) => {
            let id = Uuid::new_v4().to_string();
            value.push_str(&id);
            value
        }
        Err(..) => Uuid::new_v4().to_string(),
    };

    let mut output_message = vec![];

    for event in events.iter() {
        match start_conversation(
            init_request(event, bot_id.clone(), channel_id.clone()),
            BotOpt::CsmlBot(bot.to_owned()),
        ) {
            Ok(obj) => {
                let messages = obj["messages"].as_array().unwrap();
                for message in messages.iter() {
                    output_message.push(
                        message["payload"]["content"]["text"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );
                }

                if obj["conversation_end"].as_bool().unwrap() {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    if output_message != messages {
        panic!("\noutput {:?}\n message {:?}", output_message, messages);
    }
}

#[test]
fn ok_test_import() {
    let bot = init_bot("goto_flow").unwrap();

    let events = &["goto flow3"];

    let messages = &["hello from fn"];

    let channel_id = Uuid::new_v4().to_string();
    let bot_id = match std::env::var("GITHUB_SHA") {
        Ok(mut value) => {
            let id = Uuid::new_v4().to_string();
            value.push_str(&id);
            value
        }
        Err(..) => Uuid::new_v4().to_string(),
    };

    let mut output_message = vec![];

    for event in events.iter() {
        match start_conversation(
            init_request(event, bot_id.clone(), channel_id.clone()),
            BotOpt::CsmlBot(bot.to_owned()),
        ) {
            Ok(obj) => {
                let messages = obj["messages"].as_array().unwrap();
                for message in messages.iter() {
                    output_message.push(
                        message["payload"]["content"]["text"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );
                }

                if obj["conversation_end"].as_bool().unwrap() {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    if output_message != messages {
        panic!("\noutput {:?}\n message {:?}", output_message, messages);
    }
}

#[test]
fn ok_test_commands() {
    let bot = init_bot("goto_flow").unwrap();

    let events = &["/flow4"];

    let messages = &["flow4"];

    let channel_id = Uuid::new_v4().to_string();
    let bot_id = match std::env::var("GITHUB_SHA") {
        Ok(mut value) => {
            let id = Uuid::new_v4().to_string();
            value.push_str(&id);
            value
        }
        Err(..) => Uuid::new_v4().to_string(),
    };

    let mut output_message = vec![];

    for event in events.iter() {
        match start_conversation(
            init_request(event, bot_id.clone(), channel_id.clone()),
            BotOpt::CsmlBot(bot.to_owned()),
        ) {
            Ok(obj) => {
                let messages = obj["messages"].as_array().unwrap();
                for message in messages.iter() {
                    output_message.push(
                        message["payload"]["content"]["text"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );
                }

                if obj["conversation_end"].as_bool().unwrap() {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    if output_message != messages {
        panic!("\noutput {:?}\n message {:?}", output_message, messages);
    }
}

#[test]
fn ok_test_goto_var() {
    let bot = init_bot("goto_flow").unwrap();

    let events = &["/flow5"];

    let messages = &["flow5 start", "flow5 step1", "flow4"];

    let channel_id = Uuid::new_v4().to_string();
    let bot_id = match std::env::var("GITHUB_SHA") {
        Ok(mut value) => {
            let id = Uuid::new_v4().to_string();
            value.push_str(&id);
            value
        }
        Err(..) => Uuid::new_v4().to_string(),
    };

    let mut output_message = vec![];

    for event in events.iter() {
        match start_conversation(
            init_request(event, bot_id.clone(), channel_id.clone()),
            BotOpt::CsmlBot(bot.to_owned()),
        ) {
            Ok(obj) => {
                let messages = obj["messages"].as_array().unwrap();
                for message in messages.iter() {
                    output_message.push(
                        message["payload"]["content"]["text"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );
                }

                if obj["conversation_end"].as_bool().unwrap() {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    if output_message != messages {
        panic!("\noutput {:?}\n message {:?}", output_message, messages);
    }
}

#[test]
fn ok_test_memory() {
    let bot = init_bot("goto_flow").unwrap();

    let events = &["/flow6"];
    let messages = &[
        "flow6 start",
        "{\"val\":1}",
        "message",
        "4",
        "4.2",
        "[21,42,84]",
    ];

    let channel_id = Uuid::new_v4().to_string();
    let bot_id = match std::env::var("GITHUB_SHA") {
        Ok(mut value) => {
            let id = Uuid::new_v4().to_string();
            value.push_str(&id);
            value
        }
        Err(..) => Uuid::new_v4().to_string(),
    };

    let mut output_message = vec![];

    for event in events.iter() {
        match start_conversation(
            init_request(event, bot_id.clone(), channel_id.clone()),
            BotOpt::CsmlBot(bot.to_owned()),
        ) {
            Ok(obj) => {
                let messages = obj["messages"].as_array().unwrap();
                for message in messages.iter() {
                    output_message.push(
                        message["payload"]["content"]["text"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    );
                }

                if obj["conversation_end"].as_bool().unwrap() {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }

    if output_message != messages {
        panic!("\noutput {:?}\n messages {:?}", output_message, messages);
    }
}
