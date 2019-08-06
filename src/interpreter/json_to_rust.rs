use serde::{Deserialize, Serialize};
use crate::parser::{ast::Literal, tokens::*};
use multimap::MultiMap;
use serde_json::Value;

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

pub fn extract_form_object(literal: Literal) -> Literal {
    if let Literal::ObjectLiteral{properties, name: None, ..} = &literal {
        if properties.len() == 1 {
            properties[0].clone()
        } else {
            literal.clone()
        }
    } else {
        literal
    }
}

pub fn json_to_literal(literal: &serde_json::Value) -> Result<Literal, String> {

    match literal.get("type") {
        Some(Value::String(val)) if val == "string"  => {
            match (literal.get("value"), literal.get("name")) {
                (Some(Value::String(string)), Some(Value::String(name))) => {
                    Ok(Literal::string(string.to_owned(), Some(name.to_owned())))
                }
                (Some(Value::String(string)), None) => {
                    Ok(Literal::string(string.to_owned(), None))
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "int"     => {
            match (literal.get("value"), literal.get("name")) {
                (Some(Value::Number(literal)), Some(Value::String(name))) => {
                    if let Some(int) = literal.as_i64() {
                        Ok(Literal::int(int, Some(name.to_owned())))
                    } else {
                        Err(format!("value {} is not of type int", literal))
                    }
                }
                (Some(Value::Number(literal)), None) => {
                    if let Some(int) = literal.as_i64() {
                        Ok(Literal::int(int, None))
                    } else {
                        Err(format!("value {} is not of type int", literal))
                    }
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "float"   => {
            match (literal.get("value"), literal.get("name")) {
                (Some(Value::Number(float)), Some(Value::String(name))) => {
                    if let Some(literal) = float.as_f64() {
                        Ok(Literal::float(literal, Some(name.to_owned())))
                    } else {
                        Err(format!("value {} is not of type float", float))
                    }
                }
                (Some(Value::Number(float)), None) => {
                    if let Some(literal) = float.as_f64() {
                        Ok(Literal::float(literal, None))
                    } else {
                        Err(format!("value {} is not of type float", float))
                    }
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "bool"    => {
            match (literal.get("value"), literal.get("name")) {
                (Some(Value::Bool(boolean)), Some(Value::String(name))) => {
                    Ok(Literal::boolean(*boolean, Some(name.to_owned())))
                }
                (Some(Value::Bool(boolean)), None) => {
                    Ok(Literal::boolean(*boolean, None))
                }
                _ => Err(format!("Object of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "array"   => {
            
            match (literal.get("items"), literal.get("name")) {
                (Some(Value::Array(val)), Some(Value::String(name))) => {
                    let mut array: Vec<Literal> = vec![];

                    for literal in val.iter() {
                        array.push(json_to_literal(literal)?);
                    }
                    Ok(Literal::array(array, Some(name.to_owned())))
                }
                (Some(Value::Array(val)), None) => {
                    let mut array: Vec<Literal> = vec![];

                    for literal in val.iter() {
                        array.push(json_to_literal(literal)?);
                    }
                    Ok(Literal::array(array, None))
                }
                _ => Err(format!("Array of type {} bad format", val))
            }
        },
        Some(Value::String(val)) if val == "object"  => {

            match (literal.get("properties"), literal.get("name")) {
                (Some(Value::Array(val)), Some(Value::String(name))) => {
                    let mut array: Vec<Literal> = vec![];

                    for literal in val.iter() {
                        array.push(json_to_literal(literal)?);
                    }
                    Ok(Literal::object(array, Some(name.to_owned())))
                }
                (Some(Value::Array(val)), None) => {
                    let mut array: Vec<Literal> = vec![];

                    for literal in val.iter() {
                        array.push(json_to_literal(literal)?);
                    }
                    Ok(Literal::array(array, None))
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