use crate::data::context::Context;
use crate::data::Event;
use crate::data::{ast::*, Literal};

use crate::data::context::ContextStepInfo;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviousInfo {
    pub flow: String,
    pub step_at_flow: (ContextStepInfo, String), // step / flow
}

#[derive(Debug)]
pub struct Data<'a> {
    pub flows: &'a HashMap<String, Flow>,
    pub extern_flows: &'a HashMap<String, Flow>,
    pub flow: &'a Flow,
    pub constants: HashMap<String, Literal>,
    pub default_flow: String,
    pub context: &'a mut Context,
    pub event: &'a Event,
    pub env: &'a Literal,

    pub loop_indexes: Vec<usize>,
    pub loop_index: usize,

    pub step_count: &'a mut usize,
    pub step_limit: usize,

    pub step_vars: HashMap<String, Literal>,
    pub previous_info: Option<PreviousInfo>,
    pub custom_component: &'a serde_json::Map<String, serde_json::Value>,
    pub native_component: &'a serde_json::Map<String, serde_json::Value>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PreviousInfo {
    pub fn new(flow: String, step: ContextStepInfo) -> Self {
        Self {
            flow: flow.clone(),
            step_at_flow: (step, flow),
        }
    }

    pub fn goto(&mut self, flow: String, step: ContextStepInfo) {
        if self.step_at_flow.1 != flow {
            self.flow = self.step_at_flow.1.clone();
        }

        self.step_at_flow = (step, flow);
    }
}

impl<'a> Data<'a> {
    pub fn new(
        flows: &'a HashMap<String, Flow>,
        extern_flows: &'a HashMap<String, Flow>,
        flow: &'a Flow,
        default_flow: String,
        context: &'a mut Context,
        event: &'a Event,
        env: &'a Literal,
        loop_indexes: Vec<usize>,
        loop_index: usize,
        step_count: &'a mut usize,
        step_limit: usize,
        step_vars: HashMap<String, Literal>,
        previous_info: Option<PreviousInfo>,
        custom_component: &'a serde_json::Map<String, serde_json::Value>,
        native_component: &'a serde_json::Map<String, serde_json::Value>,
    ) -> Self {
        let constants = flow.constants.clone();

        Self {
            flows,
            extern_flows,
            flow,
            constants,
            default_flow,
            context,
            event,
            env,
            loop_indexes,
            loop_index,
            step_count,
            step_limit,
            step_vars,
            previous_info,
            custom_component,
            native_component,
        }
    }

    pub fn copy_scope(
        &self,
    ) -> (
        String,
        Context,
        Event,
        Literal,
        Vec<usize>,
        usize,
        usize,
        usize,
        HashMap<String, Literal>,
    ) {
        (
            self.default_flow.to_string(),
            init_child_context(&self),
            self.event.clone(),
            self.env.clone(),
            self.loop_indexes.clone(),
            self.loop_index.clone(),
            *self.step_count,
            self.step_limit,
            self.step_vars.clone(),
        )
    }

    // get permanent and temporary memories in a single hashmap
    pub fn get_all_memories(&self) -> HashMap<String, Literal> {
        let remember_memory = self.context.current.clone();
        let step_memory = self.step_vars.clone();

        remember_memory.into_iter().chain(step_memory).collect()
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
        previous_bot: data.context.previous_bot.clone(),
    }
}

pub fn init_child_scope<'a>(
    data: &'a Data,
    context: &'a mut Context,
    step_count: &'a mut usize,
) -> Data<'a> {
    Data::new(
        &data.flows,
        &data.extern_flows,
        &data.flow,
        data.default_flow.clone(),
        context,
        &data.event,
        &data.env,
        data.loop_indexes.clone(),
        data.loop_index,
        step_count,
        data.step_limit,
        HashMap::new(),
        data.previous_info.clone(),
        &data.custom_component,
        &data.native_component,
    )
}
