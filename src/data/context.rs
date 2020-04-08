use crate::data::{
    primitive::{PrimitiveObject, PrimitiveType},
    Client, Hold, Interval, Literal,
};

use crate::interpreter::json_to_literal;

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
pub struct ContextJson {
    pub current: serde_json::Value,
    pub metadata: serde_json::Value,
    pub api_info: Option<ApiInfo>,
    pub hold: Option<Hold>,
    pub step: String,
    pub flow: String,
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

impl Context {
    pub fn new(
        current: HashMap<String, Literal>,
        metadata: HashMap<String, Literal>,
        api_info: Option<ApiInfo>,
        hold: Option<Hold>,
        step: String,
        flow: String,
    ) -> Self {
        Self {
            current,
            metadata,
            api_info,
            hold,
            step,
            flow,
        }
    }
}

impl ContextJson {
    pub fn new(
        current: serde_json::Value,
        metadata: serde_json::Value,
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
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl ContextJson {
    pub fn to_literal(&self) -> Context {
        let current = get_hashmap(&self.current);
        let metadata = get_hashmap(&self.metadata);

        Context::new(
            current,
            metadata,
            self.api_info.to_owned(),
            self.hold.to_owned(),
            self.step.to_owned(),
            self.flow.to_owned(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

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
