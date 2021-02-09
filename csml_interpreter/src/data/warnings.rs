use crate::data::ast::Interval;
use crate::data::position::Position;
use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::collections::*;
use std::sync::*;
use std::thread::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

pub const WARNING_USE: & str = "use will be soon a deprecated keyword please use 'do' instead. https://docs.csml.dev/memory/temporary-and-long-term-variables";
pub const WARNING_REMEMBER_AS: & str = "'remember value as key' will be soon a deprecated keyword please use 'remember key = value' instead. https://docs.csml.dev/memory/temporary-and-long-term-variables";
pub const WARNING_OBJECT: & str = "'Object(key = value)' will be soon a deprecated Macro please use '{key: value}' instead; https://docs.csml.dev/automatic-type-inference/literals-objects-arrays";

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ThreadId, Vec<Warnings>>> = Mutex::new(HashMap::default());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warnings {
    pub message: &'static str,
    pub position: Position,
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

impl Warnings {
    fn new(interval: Interval, message: &'static str) -> Self {
        Self {
            message,
            position: Position::new(interval),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Warnings {
    pub fn add(message: &'static str, interval: Interval) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap.entry(thread_id).or_insert_with(Vec::default);

        if let Some(vector) = hashmap.get_mut(&thread_id) {
            vector.push(Warnings::new(interval, message));
        }
    }

    pub fn clear() {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap.entry(thread_id).or_insert_with(Vec::default);

        if let Some(vector) = hashmap.get_mut(&thread_id) {
            vector.clear();
        }
    }

    pub fn get() -> Vec<Warnings> {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap.entry(thread_id).or_insert_with(Vec::default);

        if let Some(vector) = hashmap.get(&thread_id) {
            vector.to_owned()
        } else {
            unreachable!();
        }
    }
}
