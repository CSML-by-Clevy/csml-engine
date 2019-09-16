use csmlinterpreter::{interpret, parse_file};
use csmlinterpreter::interpreter::{json_to_rust::*};
use csmlinterpreter::parser::{ast::*}; //, is_trigger
use multimap::MultiMap;

// use serde::{Deserialize, Serialize};
// use serde_json::json;

use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// fn match parse_file<'a>(text: &'a [u8]) -> Flow {
//     match Parser::parse_flow(text) {
//         Ok(flow) => flow,
//         Err(e) => panic!("error parse file {:?}", e),
//     }
// }

fn interpret_flow(flow: &Flow, step_name: &str) {
    let event = 
    None;
    // Some(Event{
    //     payload: PayLoad {
    //         content_type: "text".to_owned(),
    //         content: PayLoadContent {
    //             text: "4".to_owned()
    //         }
    //     }
    // });
    // Some(Event{
    //     literal: Literal::string("42".to_owned())
    // });

    let mut metadata = MultiMap::new();

    metadata.insert("firstname".to_owned(), 
        MemoryType {
            created_at: "Today".to_owned(),
            step_id: None,
            flow_id: None,
            conversation_id: None,
            key: "firstname".to_owned(),
            value: serde_json::Value::String("Alexis".to_owned()),
        }
    );

    metadata.insert("tutu".to_owned(), 
        MemoryType {
            created_at: "Today".to_owned(),
            step_id: None,
            flow_id: None,
            conversation_id: None,
            key: "tutu".to_owned(),
            value: serde_json::Value::String("toto".to_owned()),
        }
    );

    let memory = Context {
        past: MultiMap::new(),
        current: MultiMap::new(),
        metadata: metadata,
        retries: 42,
        is_initial_step: false,
        client : Client{bot_id: "1".to_owned(), channel_id: "2".to_owned(), user_id: "3".to_owned()},
        fn_endpoint: "toto".to_owned()
    };

    match interpret(flow, step_name, &memory, &event) {
        Ok(msg) => dbg!(msg),
        Err(e) => panic!("error: {:?}", e),
    };
}

// use std::{env, io::Read};
// use curl::easy::{Easy, List};

fn main() {
    let text = read_file("CSML/test.csml".to_owned()).unwrap();

    let flow = match parse_file(text) {
        Ok(flow) => flow,
        Err(e) => panic!("{:?}", e),
    };

    interpret_flow(&flow, "start");

    // println!("flow -> {:?}", flow);
    // println!("n-------------------------------------njson -> {:?}", serde_json::to_string(&flow).unwrap() );
}

// ((1 + 3) > 6) && var

// if (true == (7 && (7 - (3+(2*1)) == 2)))) {
//     goto toto
// }

// List(1, 2) => [1, 2]

// "{{(1 + 3)}} {{var}}"

// ((1 + 3) > 6) + var
