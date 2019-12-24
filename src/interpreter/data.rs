use crate::interpreter::json_to_rust::*;
use crate::parser::{ast::*, literal::Literal};
use curl::easy::Easy;
use std::collections::HashMap;

pub struct Data<'a> {
    pub ast: &'a Flow,
    pub memory: &'a mut Context,
    pub event: &'a Option<Event>,
    pub curl: Easy,
    pub step_vars: HashMap<String, Literal>,
}
