use crate::data::context::Context;
use crate::data::Event;
use crate::data::{ast::*, Literal};

use curl::easy;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Data {
    pub flow: Flow,
    pub context: Context,
    pub event: Event,
    pub curl: easy::Easy,
    pub step_vars: HashMap<String, Literal>,
    pub header: serde_json::Value,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Data {
    pub fn new(
        flow: &Flow,
        context: &mut Context,
        event: &Event,
        curl: easy::Easy,
        step_vars: HashMap<String, Literal>,
        header: serde_json::Value,
    ) -> Self {
        Self {
            flow: flow.to_owned(),
            context: context.to_owned(),
            event: event.to_owned(),
            curl,
            step_vars,
            header,
        }
    }
}
