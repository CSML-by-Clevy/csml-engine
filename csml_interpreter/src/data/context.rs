use crate::data::{
    primitive::{PrimitiveObject, PrimitiveType},
    Client, Hold, Interval, Literal,
};

use crate::interpreter::{json_to_literal, memory_to_literal};

use nom::lib::std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct ApiInfo {
    pub client: Client,
    pub fn_endpoint: String,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub current: HashMap<String, Literal>,
    pub metadata: HashMap<String, Literal>,
    pub api_info: Option<ApiInfo>,
    pub hold: Option<Hold>,
    pub step: String,
    pub flow: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_hashmap_from_mem(lit: &serde_json::Value) -> HashMap<String, Literal> {
    match memory_to_literal(
        lit,
        Interval {
            line: 0,
            column: 0,
            offset: 0,
        },
    ) {
        Ok(vars) if vars.primitive.get_type() == PrimitiveType::PrimitiveObject => {
            match vars.primitive.as_any().downcast_ref::<PrimitiveObject>() {
                Some(map) => map.value.clone(),
                None => HashMap::new(),
            }
        }
        _ => HashMap::new(),
    }
}

pub fn get_hashmap_from_json(lit: &serde_json::Value) -> HashMap<String, Literal> {
    match json_to_literal(
        lit,
        Interval {
            line: 0,
            column: 0,
            offset: 0,
        },
    ) {
        Ok(vars) if vars.primitive.get_type() == PrimitiveType::PrimitiveObject => {
            match vars.primitive.as_any().downcast_ref::<PrimitiveObject>() {
                Some(map) => map.value.clone(),
                None => HashMap::new(),
            }
        }
        _ => HashMap::new(),
    }
}

impl Context {
    pub fn new(
        current: HashMap<String, Literal>,
        metadata: HashMap<String, Literal>,
        api_info: Option<ApiInfo>,
        hold: Option<Hold>,
        step: &str,
        flow: &str,
    ) -> Self {
        Self {
            current,
            metadata,
            api_info,
            hold,
            step: step.to_owned(),
            flow: flow.to_owned(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_hashmap(lit: &serde_json::Value) -> HashMap<String, Literal> {
    match json_to_literal(
        lit,
        Interval {
            line: 0,
            column: 0,
            offset: 0,
        },
    ) {
        Ok(vars) if vars.primitive.get_type() == PrimitiveType::PrimitiveObject => {
            match vars.primitive.as_any().downcast_ref::<PrimitiveObject>() {
                Some(map) => map.value.clone(),
                None => HashMap::new(),
            }
        }
        _ => HashMap::new(),
    }
}
