use crate::data::ast::Interval;

use lazy_static::*;
use std::collections::*;
use std::sync::*;
use std::thread::*;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ThreadId, Position>> = Mutex::new(HashMap::default());
}

////////////////////////////////////////////////////////////////////////////////
// STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub flow: String,
    pub step: String,
    pub interval: Interval,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTION
////////////////////////////////////////////////////////////////////////////////

impl Default for Position {
    fn default() -> Self {
        Self {
            flow: String::default(),
            step: String::default(),
            interval: Interval::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

impl Position {
    pub fn new(interval: Interval) -> Self {
        Self {
            flow: Position::get_flow(),
            step: Position::get_step(),
            interval,
        }
    }

    pub fn set_flow(flow: &str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Position::default());

        if let Some(position) = hashmap.get_mut(&thread_id) {
            position.flow = flow.to_owned();
        }
    }

    pub fn set_step(step: &str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Position::default());

        if let Some(position) = hashmap.get_mut(&thread_id) {
            position.step = step.to_owned();
        }
    }

    pub fn get_flow() -> String {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Position::default());

        if let Some(position) = hashmap.get(&thread_id) {
            position.flow.to_owned()
        } else {
            unreachable!();
        }
    }

    pub fn get_step() -> String {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Position::default());

        if let Some(position) = hashmap.get(&thread_id) {
            position.step.to_owned()
        } else {
            unreachable!();
        }
    }
}