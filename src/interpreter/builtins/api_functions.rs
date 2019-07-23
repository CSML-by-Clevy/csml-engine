use serde_json::Value;
use std::collections::HashMap;
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{data::Data, builtins::*};
use crate::parser::{ast::Literal};

// default #############################################################################

fn parse_api(mut args: Vec<Literal>, data: &mut Data) -> Result<(String, HashMap<String, Value>), ErrorInfo> {
    let mut map: HashMap<String, Value> = HashMap::new();

    if let Some(Literal::StringLiteral{value: fn_id, ..}) = Literal::search_in_obj(&args, "fn_id") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    } else if args.len() >= 1 {
        if let Literal::StringLiteral{value: fn_id, ..} = &args[0] {
            map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
            args.reverse();
            args.pop();
            args.reverse();
        }
    }

    let sub_map = create_submap(&["fn_id"], &args)?;
    let client = client_to_json(&data.memory.client);

    map.insert("data".to_owned(), Value::Object(sub_map));
    map.insert("client".to_owned(), Value::Object(client));
    Ok((format!("{}", data.memory.fn_endpoint), map))
}

pub fn api(args: &Vec<Literal>, interval: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(args.clone(), data)?;

    println!("http call {:?}", http_arg);
    println!("map {:?}", serde_json::to_string(&map).unwrap());
    match reqwest::Client::new().post(&http_arg).json(&map).send() {
        Ok(ref mut arg) => match &arg.text() {
            Ok(text) => {
                println!("reqwest post ok: ");
                let json: serde_json::Value = serde_json::from_str(&text).unwrap();
                if let Some(Value::String(val)) = json.get("data") {
                    Ok(Literal::string(val.to_string(), None))
                } else {
                    Ok(Literal::null())
                }
            }
            Err(e) => {
                println!("error in parsing reqwest result: {:?}", e);
                Err(ErrorInfo{
                    message: "Error in parsing reqwest result".to_owned(),
                    interval
                })
            }
        },
        Err(e) => {
            println!("error in reqwest post {:?}", e);
            Err(ErrorInfo{
                message: "Error in reqwest post".to_owned(),
                interval
            })
        }
    }
}
