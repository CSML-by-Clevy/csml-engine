use crate::error_format::data::ErrorInfo;
use crate::interpreter::{builtins::*, data::Data};
use crate::parser::{literal::Literal, tokens::*};

use curl::easy::{Easy, List};
use serde_json::{map::Map, Value};
use std::{collections::HashMap, env, io::Read};

fn parse_api(
    args: &HashMap<String, Literal>,
    data: &Data,
) -> Result<(String, String), ErrorInfo> {
    let mut map: Map<String, Value> = Map::new();

    if let Some(Literal::StringLiteral { value: fn_id, .. }) = args.get("fn_id") {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    } else if let Some(Literal::StringLiteral { value: fn_id, .. }) = args.get(DEFAULT) {
        map.insert("function_id".to_owned(), Value::String(fn_id.to_owned()));
    }

    let sub_map = create_submap(&["fn_id", DEFAULT], &args)?;
    let client = client_to_json(&data.memory.client);

    map.insert("data".to_owned(), Value::Object(sub_map));
    map.insert("client".to_owned(), Value::Object(client));
    Ok((
        data.memory.fn_endpoint.to_string(),
        serde_json::to_string(&map).unwrap(),
    ))
}

fn format_and_transfer(mut easy: Easy, result: &mut Vec<u8>, mut data: &[u8]) {
    let mut list = List::new();
    list.append("Accept: application/json").unwrap();
    list.append("Content-Type: application/json").unwrap();
    match env::var("FN_X_API_KEY") {
        Ok(key) => list.append(&format!("X-Api-Key: {}", key)).unwrap(),
        Err(_e) => list.append("X-Api-Key: PoePoe").unwrap(),
    };

    easy.http_headers(list).unwrap();
    let mut transfer = easy.transfer();

    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer
        .write_function(|new_data| {
            result.extend_from_slice(new_data);
            Ok(new_data.len())
        })
        .unwrap();

    transfer.perform().unwrap();
}

pub fn api(
    args: HashMap<String, Literal>,
    interval: Interval,
    data: &Data,
) -> Result<Literal, ErrorInfo> {
    let (http_arg, map) = parse_api(&args, data)?;
    let data = map.as_bytes();
    let mut result = Vec::new();
    let mut easy = Easy::new();

    easy.url(&http_arg).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    format_and_transfer(easy, &mut result, data);

    let json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&result)).unwrap();
    if let Some(value) = json.get("data") {
        json_to_literal(value, interval.clone())
    } else {
        Ok(Literal::null(interval))
    }
}
