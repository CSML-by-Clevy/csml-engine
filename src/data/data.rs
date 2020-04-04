use crate::data::context::Context;
use crate::data::Event;
use crate::data::{ast::*, Literal};

use curl::easy;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Data<'a> {
    pub flow: &'a Flow,
    pub context: &'a mut Context,
    pub event: &'a Event,
    pub curl: easy::Easy,
    pub step_vars: HashMap<String, Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

// impl Data {
//     pub fn new(flow: &'static Flow, context: &'static mut Context, event: &'static Event, curl: easy::Easy, step_vars: HashMap<String, Literal>) -> Self {
//         Self {
//             flow,
//             context,
//             event,
//             curl,
//             step_vars
//         }
//     }
// }