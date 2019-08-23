use serde_json::Value;
use multimap::MultiMap;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::parser::{ast::Literal};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryType {
    pub created_at: String,
    pub step_id: Option<String>,
    pub flow_id: Option<String>,
    pub conversation_id: Option<String>,
    pub key: String,
    pub value: Literal
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String
}

//TODO: change to Context
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory {
    pub past: MultiMap<String, MemoryType>,
    pub current: MultiMap<String, MemoryType>,
    pub metadata: MultiMap<String, MemoryType>,
    pub retries: i64,
    pub is_initial_step: bool,
    pub client: Client,
    pub fn_endpoint: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayLoadContent {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayLoad {
    pub content_type: String,
    pub content: PayLoadContent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub payload: PayLoad,
}

pub fn json_to_literal(literal: &serde_json::Value) -> Result<Literal, String> {
    match literal {
        Value::String(val)  => {
            Ok(Literal::string(val.to_owned()))
        },
        Value::Number(val) => {
            if let Some(float) = val.as_f64() {
                Ok(Literal::float(float))
            } else if let Some(int) = val.as_i64() {
                Ok(Literal::int(int))
            } else {
                Err(format!("Number of type {} bad format", val))
            }
        },
        Value::Bool(val)    => {
            Ok(Literal::boolean(val.to_owned()))
        },
        Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(json_to_literal(elem)?);
            }
            Ok(Literal::array(vec))
        },
        Value::Object(val) => {
            let mut map = HashMap::new();

            for (k, v) in val.iter() {
                map.insert(k.to_owned(), json_to_literal(v)?);
            }
            Ok(Literal::object(map))
        },
        Value::Null    => {
            Ok(Literal::null())
        }
    }
}