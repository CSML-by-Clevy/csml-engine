use std::hash::BuildHasher;
use reqwest::{ClientBuilder, header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE}};
use serde_json::{Value, map::Map};
use std::{env, collections::HashMap};
use crate::parser::{ast::Literal};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{data::Data, builtins::*};

// default #############################################################################

fn parse_api<S: BuildHasher>(args: &HashMap<String, Literal, S>, data: &mut Data) -> Result<(String, String), ErrorInfo> {
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

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let api_key = match env::var("FN_X_API_KEY") {
        Ok(key) => HeaderValue::from_str(&key).unwrap(),
        Err(_e) => HeaderValue::from_str("PoePoe").unwrap()
    };

    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
    headers.insert("X-Api-Key", api_key);
    headers
}

pub fn api(args: HashMap<String, Literal>, interval: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(&args, data)?;
    let client = ClientBuilder::new()
            .use_rustls_tls()
            // .danger_accept_invalid_certs(true)
            .build().unwrap();

    // println!("http call {:?}", http_arg);
    // println!("map {:?}", serde_json::to_string(&map).unwrap());
    match client.post(&http_arg)
        .headers(construct_headers())
        .json(&map).send() {

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
                Err(ErrorInfo{
                    message: "Error in parsing reqwest result".to_owned(),
                    interval
                })
            }
        },
        Err(_e) => {
            Err(ErrorInfo{
                message: "Error in reqwest post".to_owned(),
                interval
            })
        }
    }
}
