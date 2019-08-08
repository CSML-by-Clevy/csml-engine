use serde_json::Value;
use std::collections::HashMap;
use crate::parser::{ast::Literal};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{data::Data, builtins::*};

// default #############################################################################

fn parse_api(args: &HashMap<String, Literal>, data: &mut Data) -> Result<(String, HashMap<String, Value>), ErrorInfo> {
    let mut map: HashMap<String, Value> = HashMap::new();

    if let Some(Literal::StringLiteral{value: fn_id, ..}) = args.get("fn_id") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    } else if let Some(Literal::StringLiteral{value: fn_id, ..}) = args.get("default") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    }

    let sub_map = create_submap(&["fn_id", "default"], &args)?;
    let client = client_to_json(&data.memory.client);

    map.insert("data".to_owned(), Value::Object(sub_map));
    map.insert("client".to_owned(), Value::Object(client));
    Ok((data.memory.fn_endpoint.to_string(), map))
}

pub fn api(args: HashMap<String, Literal>, interval: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(&args, data)?;

    // println!("http call {:?}", http_arg);
    // println!("map {:?}", serde_json::to_string(&map).unwrap());
    match reqwest::Client::new().post(&http_arg).json(&map).send() {
        Ok(ref mut arg) => match &arg.text() {
            Ok(text) => {
                // println!("reqwest post ok: ");
                let json: serde_json::Value = serde_json::from_str(&text).unwrap();
                if let Some(Value::String(val)) = json.get("data") {
                    Ok(Literal::string(val.to_string()))
                } else {
                    Ok(Literal::null())
                }
            }
            Err(_e) => {
                // println!("error in parsing reqwest result: {:?}", e);
                Err(ErrorInfo{
                    message: "Error in parsing reqwest result".to_owned(),
                    interval
                })
            }
        },
        Err(_e) => {
            // println!("error in reqwest post {:?}", e);
            Err(ErrorInfo{
                message: "Error in reqwest post".to_owned(),
                interval
            })
        }
    }
}
