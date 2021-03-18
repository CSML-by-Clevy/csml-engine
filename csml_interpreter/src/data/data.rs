use crate::data::context::Context;
use crate::data::Event;
use crate::data::{ast::*, Literal};

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Data<'a> {
    pub flows: &'a HashMap<String, Flow>,
    pub flow: &'a Flow,
    pub context: &'a mut Context,
    pub event: &'a Event,
    pub env: &'a Literal,

    pub loop_indexs: Vec<usize>,
    pub loop_index: usize,

    pub step_vars: HashMap<String, Literal>,
    pub custom_component: &'a serde_json::Map<String, serde_json::Value>,
    pub native_component: &'a serde_json::Map<String, serde_json::Value>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl<'a> Data<'a> {
    pub fn new(
        flows: &'a HashMap<String, Flow>,
        flow: &'a Flow,
        context: &'a mut Context,
        event: &'a Event,
        env: &'a Literal,
        loop_indexs: Vec<usize>,
        loop_index: usize,
        step_vars: HashMap<String, Literal>,
        custom_component: &'a serde_json::Map<String, serde_json::Value>,
        native_component: &'a serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        Self {
            flows,
            flow,
            context,
            event,
            env,
            loop_indexs,
            loop_index,
            step_vars,
            custom_component,
            native_component,
        }
    }
}
