use crate::interpreter::json_to_rust::json_to_literal;
use crate::parser::{ast::*, literal::Literal};
use crate::primitive::{object::PrimitiveObject, PrimitiveType};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Data<'a> {
    pub ast: &'a Flow,
    pub memory: &'a mut Context,
    pub event: &'a Option<Event>,
    pub curl: Easy,
    pub step_vars: HashMap<String, Literal>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    pub bot_id: String,
    pub channel_id: String,
    pub user_id: String,
}

impl Client {
    pub fn new(bot_id: String, channel_id: String, user_id: String) -> Self {
        Self {
            bot_id,
            channel_id,
            user_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextJson {
    pub past: serde_json::Value,
    pub current: serde_json::Value,
    pub metadata: serde_json::Value,
    pub retries: i64,
    pub is_initial_step: bool,
    pub client: Client,
    pub fn_endpoint: String,
}

impl ContextJson {
    pub fn to_literal(self) -> Context {
        let past = get_hashmap(&self.past);
        let current = get_hashmap(&self.current);
        let metadata = get_hashmap(&self.metadata);

        Context {
            past,
            current,
            metadata,
            retries: self.retries,
            is_initial_step: self.is_initial_step,
            client: self.client,
            fn_endpoint: self.fn_endpoint,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub past: HashMap<String, Literal>,
    pub current: HashMap<String, Literal>,
    pub metadata: HashMap<String, Literal>,
    pub retries: i64,
    pub is_initial_step: bool,
    pub client: Client,
    pub fn_endpoint: String,
}

#[derive(Debug, Clone)]
pub enum MemoryType {
    Event(String),
    Metadata,
    Use,
    Remember,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub content_type: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

pub fn get_hashmap(lit: &serde_json::Value) -> HashMap<String, Literal> {
    match json_to_literal(lit, Interval { line: 0, column: 0 }) {
        Ok(vars) if vars.primitive.get_type() == PrimitiveType::PrimitiveObject => {
            match vars.primitive.as_any().downcast_ref::<PrimitiveObject>() {
                Some(map) => map.value.clone(),
                None => HashMap::new(),
            }
        }
        _ => HashMap::new(),
    }
}
