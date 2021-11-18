pub mod data;
pub mod linter;

use crate::data::ast::Flow;
pub use data::{
    FunctionCallInfo, FunctionInfo, ImportInfo, LinterInfo, ScopeType, State, StepBreakers,
    StepInfo,
};
use std::collections::HashMap;

pub struct FlowToValidate<'a> {
    pub flow_name: String,
    pub ast: Flow,
    pub raw_flow: &'a str,
}

impl<'a> FlowToValidate<'a> {
    pub fn get_bot(flows: Vec<Self>) -> HashMap<String, Flow> {
        flows
            .into_iter()
            .map(|flow| (flow.flow_name, flow.ast))
            .collect::<HashMap<String, Flow>>()
    }
}
