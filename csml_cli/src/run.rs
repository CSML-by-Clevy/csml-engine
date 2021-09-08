use csml_engine::data::CsmlRequest;
use csml_interpreter::{
    data::{csml_bot::CsmlBot, csml_flow::CsmlFlow, Client},
    load_components,
};

use std::error::Error;
// use git2::Error;

use crate::init_package::Manifest;

use serde_json::json;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn init_request(string: &str, metadata: Option<serde_json::Value>) -> CsmlRequest {
    CsmlRequest {
        request_id: "request".to_owned(),
        client: Client {
            user_id: "user".to_owned(),
            bot_id: "botid".to_owned(),
            channel_id: "CLI".to_owned(),
        },
        callback_url: None,
        payload: json!({
            "content_type": "text",
            "content": {"text": string},
        }),
        metadata: match metadata {
            Some(metadata) => metadata,
            None => json!({}),
        },
    }
}

pub fn init_request_flow_trigger(flow_id: &str, step_id: Option<&str>) -> CsmlRequest {
    CsmlRequest {
        request_id: "_".to_owned(),
        client: Client {
            user_id: "user".to_owned(),
            bot_id: "botid".to_owned(),
            channel_id: "CLI".to_owned(),
        },
        callback_url: None,
        payload: json!({
            "content_type": "flow_trigger",
            "content": {
                "flow_id": flow_id,
                "step_id": step_id
            }
        }),
        metadata: json!({}),
    }
}

pub fn load_info(directory_name: &str) -> Result<CsmlBot, Box<dyn Error>> {
    let file = File::open(&format!("{}/manifest.yaml", directory_name))?;

    let manifest: Manifest = serde_yaml::from_reader(file)?;

    let mut flows = vec![];

    let paths = fs::read_dir(format!("{}/src", directory_name))?;

    for path in paths {
        if let Ok(dir) = path {
            let mut file = File::open(dir.path())?;

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let name = dir.path().display().to_string();
            let vec = name.split('/').collect::<Vec<&str>>();
            if vec.is_empty() {
                continue;
            }
            let len = vec.len() - 1;
            let file_name = vec[len].split('.').collect::<Vec<&str>>();

            flows.push(CsmlFlow {
                id: file_name[0].to_owned(),
                name: file_name[0].to_owned(),
                content: contents,
                commands: vec![], // link commands
            });
        }
    }

    Ok(CsmlBot {
        id: manifest.name.clone(),
        name: manifest.name.clone(),
        fn_endpoint: None,
        flows,
        native_components: Some(load_components().unwrap()),
        custom_components: None,
        default_flow: manifest.default_flow.clone(),
        bot_ast: None,
        env: None,
    })
}

pub fn search_csml_bot_folders() -> Vec<(String, CsmlBot)> {
    let paths = fs::read_dir("./").unwrap();
    let mut vec = vec![];

    let result = load_info("./");

    match result {
        Ok(csml) => {
            vec.push(("./".to_owned(), csml));
        }
        Err(_) => {}
    }

    for entry in paths {
        let p = entry.unwrap().path();
        if p.is_dir() {
            let path = format!("{}", p.display());

            let result = load_info(&path);

            match result {
                Ok(csml) => {
                    vec.push((path, csml));
                }
                Err(_) => {}
            }
        }
    }

    vec
}
