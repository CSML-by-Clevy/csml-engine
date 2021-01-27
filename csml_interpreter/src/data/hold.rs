////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Hold {
    pub index: usize,
    pub step_vars: serde_json::Value,
    pub step_name: String,
    pub flow_name: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Hold {
    pub fn new(index: usize, step_vars: serde_json::Value, step_name: String, flow_name: String) -> Self {
        Self {
            index,
            step_vars,
            step_name,
            flow_name,
        }
    }

    pub fn default() -> Self {
        Self {
            index: 0,
            step_vars: serde_json::json!({}),
            step_name: "".to_owned(),
            flow_name: "".to_owned(),
        }
    }
}
