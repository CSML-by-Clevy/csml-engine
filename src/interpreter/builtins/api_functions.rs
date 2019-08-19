use serde_json::{Value, map::Map};
use std::{env, collections::HashMap, io::Read};
use curl::easy::{Easy, List};
// use reqwest::{ClientBuilder, header::{HeaderMap/, HeaderValue, ACCEPT, CONTENT_TYPE}};
use crate::parser::{ast::Literal};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{data::Data, builtins::*};

// default #############################################################################

fn parse_api(args: &HashMap<String, Literal>, data: &mut Data) -> Result<(String, String), ErrorInfo> {
    let mut map: Map<String, Value> = Map::new();

    if let Some(Literal::StringLiteral{value: fn_id, ..}) = args.get("fn_id") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    } else if let Some(Literal::StringLiteral{value: fn_id, ..}) = args.get("default") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    }

    let sub_map = create_submap(&["fn_id", "default"], &args)?;
    let client = client_to_json(&data.memory.client);

    map.insert("data".to_owned(), Value::Object(sub_map));
    map.insert("client".to_owned(), Value::Object(client));
    Ok((data.memory.fn_endpoint.to_string(), serde_json::to_string(&map).unwrap()))
}

pub fn api(args: HashMap<String, Literal>, _interval: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(&args, data)?;
    let mut data = map.as_bytes();
    // let client = ClientBuilder::new().build().unwrap();

    // match client.post(&http_arg)
    //     .headers(construct_headers())
    //     .json(&map).send() {

    //     Ok(ref mut arg) => match &arg.text() {
    //         Ok(text) => {
    //             // println!("reqwest post ok: ");
    //             let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    //             if let Some(Value::String(val)) = json.get("data") {
    //                 Ok(Literal::string(val.to_string()))
    //             } else {
    //                 Ok(Literal::null())
    //             }
    //         }
    //         Err(_e) => {
    //             Err(ErrorInfo{
    //                 message: "Error in parsing reqwest result".to_owned(),
    //                 interval
    //             })
    //         }
    //     },
    //     Err(_e) => {
    //         Err(ErrorInfo{
    //             message: "Error in reqwest post".to_owned(),
    //             interval
    //         })
    //     }
    // }

    let mut result = Vec::new();
    let mut easy = Easy::new();
    easy.url(&http_arg).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    {
        let mut list = List::new();
        list.append("Accept: application/json").unwrap();
        list.append("Content-Type: application/json").unwrap();
        match env::var("FN_X_API_KEY") {
            Ok(key) => list.append(&format!("X-Api-Key: {}", key) ).unwrap(),
            Err(_e) =>  list.append("X-Api-Key: PoePoe").unwrap()
        };
        
        easy.http_headers(list).unwrap();

        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();

        transfer.write_function(|new_data| {
            result.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();

        transfer.perform().unwrap();
    }

    let json: serde_json::Value = serde_json::from_str(& String::from_utf8_lossy(&result)).unwrap();
    if let Some(Value::String(val)) = json.get("data") {
        Ok(Literal::string(val.to_string()))
    } else {
        Ok(Literal::null())
    }
}
