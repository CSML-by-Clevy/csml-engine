////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Hold {
    pub index: usize,
    pub step_vars: serde_json::Value,
    pub step_hash: String,
    pub loop_info: Option<Vec<usize>>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Hold {
    pub fn new(index: usize, step_vars: serde_json::Value, step_hash: String, loop_info: Option<Vec<usize>>) -> Self {
        Self {
            index,
            step_vars,
            step_hash,
            loop_info,
        }
    }

    pub fn default() -> Self {
        Self {
            index: 0,
            step_vars: serde_json::json!({}),
            step_hash: "".to_owned(),
            loop_info: None
        }
    }
}
