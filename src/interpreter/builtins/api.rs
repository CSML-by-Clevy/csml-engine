use crate::data::primitive::{null::PrimitiveNull, PrimitiveType};
use crate::data::{ast::Interval, tokens::*, ApiInfo, Client, Data, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::{builtins::tools::*, json_to_literal};

use curl::{
    easy::{Easy, List},
    Error,
};
use std::{collections::HashMap, env, io::Read};

fn parse_api(
    args: &HashMap<String, Literal>,
    client: Client,
    fn_endpoint: String,
) -> Result<(String, String), ErrorInfo> {
    let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

    if let Some(literal) = args.get("fn_id") {
        if literal.primitive.get_type() == PrimitiveType::PrimitiveString {
            let fn_id = Literal::get_value::<String>(&literal.primitive).unwrap();
            map.insert(
                "function_id".to_owned(),
                serde_json::Value::String(fn_id.to_owned()),
            );
        }
    } else if let Some(literal) = args.get(DEFAULT) {
        if literal.primitive.get_type() == PrimitiveType::PrimitiveString {
            let fn_id = Literal::get_value::<String>(&literal.primitive).unwrap();
            map.insert(
                "function_id".to_owned(),
                serde_json::Value::String(fn_id.to_owned()),
            );
        }
    }

    let sub_map = create_submap(&["fn_id", DEFAULT], &args)?;
    let client = client_to_json(&client);

    map.insert("data".to_owned(), serde_json::Value::Object(sub_map));
    map.insert("client".to_owned(), serde_json::Value::Object(client));
    Ok((fn_endpoint, serde_json::to_string(&map).unwrap()))
}

fn init(easy: &mut Easy) -> Result<(), Error> {
    let mut list = List::new();
    list.append("Accept: application/json")?;
    list.append("Content-Type: application/json")?;
    match env::var("FN_X_API_KEY") {
        Ok(key) => list.append(&format!("X-Api-Key: {}", key))?,
        Err(_e) => list.append("X-Api-Key: PoePoe")?,
    };

    easy.http_headers(list)
}

fn format_and_transfer(
    easy: &mut Easy,
    result: &mut Vec<u8>,
    http_arg: &str,
    mut data: &[u8],
) -> Result<(), Error> {
    easy.url(http_arg)?;
    easy.post(true)?;
    easy.post_field_size(data.len() as u64)?;
    init(easy)?;
    let mut transfer = easy.transfer();

    transfer.read_function(|buf| Ok(data.read(buf).unwrap_or(0)))?;
    transfer.write_function(|new_data| {
        result.extend_from_slice(new_data);
        Ok(new_data.len())
    })?;

    transfer.perform()
}

pub fn api(
    args: HashMap<String, Literal>,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    let (client, fn_endpoint) = match &data.context.api_info {
        Some(ApiInfo {
            client,
            fn_endpoint,
        }) => (client.to_owned(), fn_endpoint.to_owned()),
        None => {
            return Err(ErrorInfo {
                message: "fn call can not be make because fn_endpoint is not set".to_owned(),
                interval,
            })
        }
    };

    let (http_arg, map) = parse_api(&args, client, fn_endpoint)?;
    let data_bytes = map.as_bytes();
    let mut result = Vec::new();

    match format_and_transfer(&mut data.curl, &mut result, &http_arg, data_bytes) {
        Ok(_) => (),
        Err(err) => {
            return Err(ErrorInfo {
                message: format!("{}", err),
                interval,
            })
        }
    };

    let json: serde_json::Value = serde_json::from_str(&String::from_utf8_lossy(&result)).unwrap();
    if let Some(value) = json.get("data") {
        json_to_literal(value, interval)
    } else {
        Ok(PrimitiveNull::get_literal(interval))
    }
}
