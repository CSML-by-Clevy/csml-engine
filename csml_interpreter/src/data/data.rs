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

    pub fn copy_scope(
        &self,
    ) -> (
        HashMap<String, Flow>,
        Flow,
        Context,
        Event,
        Literal,
        Vec<usize>,
        usize,
        HashMap<String, Literal>,
        serde_json::Map<String, serde_json::Value>,
        serde_json::Map<String, serde_json::Value>,
    ) {
        (
            self.flows.clone(),
            self.flow.clone(),
            init_child_context(&self),
            self.event.clone(),
            self.env.clone(),
            self.loop_indexs.clone(),
            self.loop_index.clone(),
            self.step_vars.clone(),
            self.custom_component.clone(),
            self.native_component.clone(),
        )
    }
}

pub fn init_child_context(data: &Data) -> Context {
    Context {
        current: HashMap::new(),
        metadata: data.context.metadata.clone(),
        api_info: data.context.api_info.clone(),
        hold: None,
        step: data.context.step.clone(),
        flow: data.context.flow.clone(),
    }
}

pub fn init_child_scope<'a>(data: &'a Data, context: &'a mut Context) -> Data<'a> {
    Data::new(
        &data.flows,
        &data.flow,
        context,
        &data.event,
        &data.env,
        data.loop_indexs.clone(),
        data.loop_index,
        HashMap::new(),
        &data.custom_component,
        &data.native_component,
    )
}
