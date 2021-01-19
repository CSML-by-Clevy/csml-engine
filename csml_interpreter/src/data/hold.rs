////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Hold {
    pub index: usize,
    pub step_vars: serde_json::Value,
    pub step_hash: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Hold {
    pub fn new(index: usize, step_vars: serde_json::Value, step_hash: String) -> Self {
        Self {
            index,
            step_vars,
            step_hash,
        }
    }

    pub fn default() -> Self {
        Self {
            index: 0,
            step_vars: serde_json::json!({}),
            step_hash: "".to_owned(),
        }
    }
}
