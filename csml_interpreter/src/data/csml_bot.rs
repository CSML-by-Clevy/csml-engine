use crate::data::{CsmlFlow, Position};
use crate::error_format::*;
use crate::Interval;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

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
    pub no_interruption_delay: Option<i32>,
    pub env: Option<serde_json::Value>,
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
        no_interruption_delay: Option<i32>,
        env: Option<serde_json::Value>,
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
            no_interruption_delay,
            env,
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
            Position::new(Interval::new_as_u32(0, 0, 0, None, None), name),
            format!("{} {}", ERROR_INVALID_FLOW, name),
        )])
    }
}
