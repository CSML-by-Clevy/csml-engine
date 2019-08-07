use serde_json::Value;
use multimap::MultiMap;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::parser::{ast::Literal, tokens::*};

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

// pub fn extract_form_object(literal: Literal) -> Literal {
//     if let Literal::ObjectLiteral{properties, ..} = &literal {
//         if properties.len() == 1 {
//             properties[0].clone()
//         } else {
//             literal.clone()
//         }
//     } else {
//         literal
//     }
// }

pub fn json_to_literal(literal: &serde_json::Value) -> Result<Literal, String> {

    match literal.get("type") {
        Some(Value::String(val)) if val == "string"  => {
            match literal.get("value") {
                Some(Value::String(string)) => {
                    Ok(Literal::string(string.to_owned()))
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "int"     => {
            match literal.get("value") {
                Some(Value::Number(literal)) => {
                    if let Some(int) = literal.as_i64() {
                        Ok(Literal::int(int))
                    } else {
                        Err(format!("value {} is not of type int", literal))
                    }
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "float"   => {
            match literal.get("value") {
                Some(Value::Number(float)) => {
                    if let Some(literal) = float.as_f64() {
                        Ok(Literal::float(literal))
                    } else {
                        Err(format!("value {} is not of type float", float))
                    }
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "bool"    => {
            match literal.get("value") {
                Some(Value::Bool(boolean)) => {
                    Ok(Literal::boolean(*boolean))
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "array"   => {
            match literal.get("items") {
                Some(Value::Array(val)) => {
                    let mut array: Vec<Literal> = vec![];

                    for literal in val.iter() {
                        array.push(json_to_literal(literal)?);
                    }
                    Ok(Literal::array(array))
                }
                _ => Err(format!("Array of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "object"  => {
            match literal.get("properties") {
                Some(Value::Object(val)) => {
                    let mut obj: HashMap<String, Literal> = HashMap::new();

                    for (key, val) in val.iter() {
                        obj.insert(key.to_owned(), json_to_literal(val)?);
                    }
                    Ok(Literal::object(obj))
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == NULL    => {
            Ok(Literal::null())
        },
        e => Err(format!("bad format {:?}", e))
    }
}