use std::collections::HashMap;
use crate::parser::{ast::*};
use crate::interpreter:: {
    json_to_rust::*
};

pub struct Data<'a> {
    pub ast: &'a Flow,
    pub memory: &'a Memory,
    pub event: &'a Option<Event>,
    pub step_vars: HashMap<String, Literal>,
}