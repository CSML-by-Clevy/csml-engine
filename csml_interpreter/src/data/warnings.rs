use crate::data::ast::Interval;
use crate::data::position::Position;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

pub const WARNING_FN: &str =
    "'Fn()' will soon be deprecated. Please use the 'App()' keyword instead";
pub const WARNING_OBJECT: & str = "'Object(key = value)' will be soon a deprecated Macro please use '{key: value}' instead; https://docs.csml.dev/automatic-type-inference/literals-objects-arrays";
pub const WARNING_USE: & str = "use will be soon a deprecated keyword please use 'do' instead. https://docs.csml.dev/memory/temporary-and-long-term-variables";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warnings {
    pub message: String,
    pub position: Position,
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

impl Warnings {
    pub fn new(flow_name: &str, interval: Interval, message: &'static str) -> Self {
        Self {
            message: message.to_owned(),
            position: Position::new(interval, flow_name),
        }
    }
}
