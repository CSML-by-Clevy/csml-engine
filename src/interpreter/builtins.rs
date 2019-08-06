pub mod api_functions;
pub mod reserved_functions;

use crate::error_format::data::ErrorInfo;
use crate::parser::ast::*;
use crate::interpreter::json_to_rust::*;

use serde_json::{Map, Value};

pub fn create_submap(keys: &[&str], args: &[Literal]) -> Result<Map<String, Value>, ErrorInfo> {
    let mut map = Map::new();

    for elem in args.iter() {
        if keys.iter().find(|&&x| 
            if let Some(value) = elem.get_name() {
                x == value
            } else {
                false
            }
        ).is_none() {
            map.insert(
                if let Some(name) = elem.get_name() {name.to_owned()} else {"default".to_owned()}, 
                Value::String(elem.to_string())
            );
        }
    }

    Ok(map)
}

fn client_to_json(client: &Client) -> Map<String, Value> {
    let mut map = Map::new();

    map.insert("bot_id".to_owned(), Value::String(client.bot_id.to_owned()));
    map.insert("channel_id".to_owned(), Value::String(client.channel_id.to_owned()));
    map.insert("user_id".to_owned(), Value::String(client.user_id.to_owned()));

    map
}
