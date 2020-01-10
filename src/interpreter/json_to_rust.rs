use crate::error_format::data::ErrorInfo;
use crate::parser::{ast::Interval, literal::Literal};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

impl Client {
    pub fn new(bot_id: String, channel_id: String, user_id: String) -> Self {
        Self {
            bot_id,
            channel_id,
            user_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub past: HashMap<String, Literal>,
    pub current: HashMap<String, Literal>,
    pub metadata: HashMap<String, Literal>,
    pub retries: i64,
    pub is_initial_step: bool,
    pub client: Client,
    pub fn_endpoint: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct PayLoadContent {
//     pub text: String,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct PayLoad {
//     pub content_type: String,
//     pub content: PayLoadContent,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub payload: String,
}

pub fn json_to_literal(
    literal: &serde_json::Value,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    match literal {
        Value::String(val) => Ok(Literal::string(val.to_owned(), interval)),
        Value::Number(val) => {
            if let Some(float) = val.as_f64() {
                Ok(Literal::float(float, interval))
            } else if let Some(int) = val.as_i64() {
                Ok(Literal::int(int, interval))
            } else {
                Err(ErrorInfo {
                    message: format!("Number of type {} bad format", val),
                    interval,
                })
            }
        }
        Value::Bool(val) => Ok(Literal::boolean(val.to_owned(), interval)),
        Value::Array(val) => {
            let mut vec = vec![];

            for elem in val {
                vec.push(json_to_literal(elem, interval.clone())?);
            }
            Ok(Literal::array(vec, interval))
        }
        Value::Object(val) => {
            let mut map = HashMap::new();

            for (k, v) in val.iter() {
                map.insert(k.to_owned(), json_to_literal(v, interval.clone())?);
            }
            Ok(Literal::object(map, interval))
        }
        Value::Null => Ok(Literal::null(interval)),
    }
}
