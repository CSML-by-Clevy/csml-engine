use crate::data::primitive::{PrimitiveObject, PrimitiveType};
use crate::data::Client;
use crate::data::Interval;
use crate::data::Literal;
use crate::interpreter::json_to_literal;

use nom::lib::std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ContextJson {
    pub current: serde_json::Value,
    pub metadata: serde_json::Value,
    pub retries: i64,
    pub is_initial_step: bool,
    pub client: Client,
    pub fn_endpoint: String,
}
#[derive(Debug, Clone)]
pub struct Context {
    pub current: HashMap<String, Literal>,
    pub metadata: HashMap<String, Literal>,
    pub retries: i64,
    pub is_initial_step: bool,
    pub client: Client,
    pub fn_endpoint: String,
}

pub fn get_hashmap(lit: &serde_json::Value) -> HashMap<String, Literal> {
    match json_to_literal(lit, Interval { line: 0, column: 0 }) {
        Ok(vars) if vars.primitive.get_type() == PrimitiveType::PrimitiveObject => {
            match vars.primitive.as_any().downcast_ref::<PrimitiveObject>() {
                Some(map) => map.value.clone(),
                None => HashMap::new(),
            }
        }
        _ => HashMap::new(),
    }
}

impl ContextJson {
    pub fn to_literal(self) -> Context {
        let current = get_hashmap(&self.current);
        let metadata = get_hashmap(&self.metadata);

        Context {
            current,
            metadata,
            retries: self.retries,
            is_initial_step: self.is_initial_step,
            client: self.client,
            fn_endpoint: self.fn_endpoint,
        }
    }
}
