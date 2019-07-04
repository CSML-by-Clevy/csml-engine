use crate::interpreter::json_to_rust::*;
use crate::parser::ast::*;
use std::collections::HashMap;

pub struct Data<'a> {
    pub ast: &'a Flow,
    pub memory: &'a Memory,
    pub event: &'a Option<Event>,
    pub step_vars: HashMap<String, Literal>,
}
