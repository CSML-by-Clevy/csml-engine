use crate::data::{CsmlFlow, Position};
use crate::error_format::*;
use crate::Interval;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializeCsmlBot {
    pub id: String,
    pub name: String,
    pub flows: Vec<CsmlFlow>,
    pub native_components: Option<String>, // serde_json::Map<String, serde_json::Value>
    pub custom_components: Option<String>, // serde_json::Value
    pub default_flow: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsmlBot {
    pub id: String,
    pub name: String,
    pub fn_endpoint: Option<String>,
    pub flows: Vec<CsmlFlow>,
    pub native_components: Option<serde_json::Map<String, serde_json::Value>>, //
    pub custom_components: Option<serde_json::Value>,                          // serde_json::Value
    pub default_flow: String,
    pub bot_ast: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoBot {
    pub id: String,
    pub name: String,
    pub custom_components: Option<String>,
    pub default_flow: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl CsmlBot {
    pub fn new(
        id: &str,
        name: &str,
        fn_endpoint: Option<String>,
        flows: Vec<CsmlFlow>,
        native_components: Option<serde_json::Map<String, serde_json::Value>>,
        custom_components: Option<serde_json::Value>,
        default_flow: &str,
        bot_ast: Option<String>,
    ) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            fn_endpoint,
            flows,
            native_components,
            custom_components,
            default_flow: default_flow.to_owned(),
            bot_ast,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl CsmlBot {
    pub fn get_flow(&self, name: &str) -> Result<String, Vec<ErrorInfo>> {
        for flow in self.flows.iter() {
            if flow.name == name {
                return Ok(flow.content.to_owned());
            }
        }

        Err(vec![gen_error_info(
            Position::new(Interval::new_as_u32(0, 0)),
            format!("{} {}", ERROR_INVALID_FLOW, name),
        )])
    }

    pub fn to_serializable_bot(&self) -> SerializeCsmlBot {
        SerializeCsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            flows: self.flows.to_owned(),
            native_components: {
                match self.native_components.to_owned() {
                    Some(value) => Some(serde_json::Value::Object(value).to_string()),
                    None => None,
                }
            },
            custom_components: {
                match self.custom_components.to_owned() {
                    Some(value) => Some(value.to_string()),
                    None => None,
                }
            },
            default_flow: self.default_flow.to_owned(),
        }
    }
}

impl SerializeCsmlBot {
    pub fn to_bot(&self) -> CsmlBot {
        CsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            fn_endpoint: None,
            flows: self.flows.to_owned(),
            native_components: {
                match self.native_components.to_owned() {
                    Some(value) => match serde_json::from_str(&value) {
                        Ok(serde_json::Value::Object(map)) => Some(map),
                        _ => unreachable!(),
                    },
                    None => None,
                }
            },
            custom_components: {
                match self.custom_components.to_owned() {
                    Some(value) => match serde_json::from_str(&value) {
                        Ok(value) => Some(value),
                        Err(_e) => unreachable!(),
                    },
                    None => None,
                }
            },
            default_flow: self.default_flow.to_owned(),
            bot_ast: None,
        }
    }
}

impl DynamoBot {
    pub fn to_bot(&self, flows: Vec<CsmlFlow>) -> CsmlBot {
        CsmlBot {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            fn_endpoint: None,
            flows,
            native_components: None,
            custom_components: {
                match self.custom_components.to_owned() {
                    Some(value) => match serde_json::from_str(&value) {
                        Ok(value) => Some(value),
                        Err(_e) => unreachable!(),
                    },
                    None => None,
                }
            },
            default_flow: self.default_flow.to_owned(),
            bot_ast: None,
        }
    }
}
