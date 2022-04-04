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
    #[serde(alias = "fn_endpoint")]
    pub apps_endpoint: Option<String>,
    pub flows: Vec<CsmlFlow>,
    pub modules: Option<Modules>,
    pub native_components: Option<serde_json::Map<String, serde_json::Value>>, //
    pub custom_components: Option<serde_json::Value>,                          // serde_json::Value
    pub default_flow: String,
    pub bot_ast: Option<String>,
    pub no_interruption_delay: Option<i32>,
    pub env: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleData {
    pub name: String,
    pub url: Option<String>,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_version() -> String {
    "latest".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modules {
    pub config: String,
    #[serde(default)]
    pub flows: Vec<CsmlFlow>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl CsmlBot {
    pub fn new(
        id: &str,
        name: &str,
        apps_endpoint: Option<String>,
        flows: Vec<CsmlFlow>,
        native_components: Option<serde_json::Map<String, serde_json::Value>>,
        custom_components: Option<serde_json::Value>,
        default_flow: &str,
        bot_ast: Option<String>,
        no_interruption_delay: Option<i32>,
        env: Option<serde_json::Value>,
        modules: Option<Modules>,
    ) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            apps_endpoint,
            flows,
            modules,
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

    pub fn to_json(&self) -> serde_json::Value {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

        map.insert("id".to_owned(), serde_json::json!(self.id));
        map.insert("name".to_owned(), serde_json::json!(self.name));
        map.insert(
            "apps_endpoint".to_owned(),
            serde_json::json!(self.apps_endpoint),
        );
        map.insert("flows".to_owned(), serde_json::json!(self.flows));
        map.insert(
            "default_flow".to_owned(),
            serde_json::json!(self.default_flow),
        );
        map.insert(
            "no_interruption_delay".to_owned(),
            serde_json::json!(self.no_interruption_delay),
        );
        map.insert("env".to_owned(), serde_json::json!(self.env));

        serde_json::json!(map)
    }
}
