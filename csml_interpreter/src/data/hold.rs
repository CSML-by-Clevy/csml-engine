use serde::{Serialize, Deserialize};
use crate::data::{Data, Literal};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub command_index: usize,
    pub loop_index: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Hold {
    pub index: IndexInfo,
    pub step_vars: serde_json::Value,
    pub step_name: String,
    pub flow_name: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Hold {
    pub fn new(index: IndexInfo, step_vars: serde_json::Value, step_name: String, flow_name: String) -> Self {
        Self {
            index,
            step_vars,
            step_name,
            flow_name,
        }
    }

    pub fn default() -> Self {
        Self {
            index: IndexInfo{command_index: 0 , loop_index: vec![]},
            step_vars: serde_json::json!({}),
            step_name: "".to_owned(),
            flow_name: "".to_owned(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////


pub fn hold_index_start_loop<'a>(data: &mut Data, mut _array: &'a [Literal], skip_value: &mut usize) {
    // add the new loop index in stack
    data.loop_indexs.push(0);

    if let Some(hold) = &mut data.context.hold {
        let loop_index = &mut hold.index.loop_index;
        if data.loop_index < loop_index.len() {
            _array = &_array[loop_index[data.loop_index]..];
            *skip_value = loop_index[data.loop_index];
        }
    }
}


// remove the loop index of the stack
pub fn hold_index_end_loop(data: &mut Data) {
    data.loop_indexs.pop();
}

pub fn hold_loop_incrs_index(data: &mut Data, index: usize) {
    data.loop_indexs[data.loop_index] = index;
    data.loop_index = data.loop_index + 1;
}

pub fn hold_loop_decrs_index(data: &mut Data) {
    data.loop_index = data.loop_index - 1;
}
