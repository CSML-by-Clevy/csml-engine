use crate::data::context::Context;
use crate::data::Event;
use crate::data::{ast::*, Literal};

use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Data {
    pub flow: Flow,
    pub context: Context,
    pub event: Event,
    pub step_vars: HashMap<String, Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Data {
    pub fn new(
        flow: &Flow,
        context: &mut Context,
        event: &Event,
        step_vars: HashMap<String, Literal>,
    ) -> Self {
        Self {
            flow: flow.to_owned(),
            context: context.to_owned(),
            event: event.to_owned(),
            step_vars,
        }
    }
}
