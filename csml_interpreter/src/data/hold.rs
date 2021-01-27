////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct IndexInfo<'a> {
    pub current_command_index: Vec<usize>,
    pub current_loop_index: Vec<usize>,

    pub command_index: &'a [usize],
    pub loop_index: &'a [usize]
}

#[derive(Debug, Clone)]
pub struct Hold {
    pub command_index: usize,
    pub loop_index: Vec<usize>,
    pub step_vars: serde_json::Value,
    pub step_hash: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Hold {
    pub fn new(command_index: usize, loop_index: Vec<usize>, step_vars: serde_json::Value, step_hash: String) -> Self {
        Self {
            command_index,
            loop_index,
            step_vars,
            step_hash,
        }
    }

    pub fn default() -> Self {
        Self {
            command_index: 0,
            loop_index: vec![],
            step_vars: serde_json::json!({}),
            step_hash: "".to_owned(),
        }
    }
}
