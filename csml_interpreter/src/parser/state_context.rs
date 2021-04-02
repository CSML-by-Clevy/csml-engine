use crate::data::Literal;
use lazy_static::*;
use std::collections::*;
use std::sync::*;
use std::thread::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref CONTEXT: Mutex<HashMap<ThreadId, StateContext>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitCondition {
    Goto,
    End,
    Error,
    Break,
    Continue,
    Hold,
    Return(Literal),
}

#[derive(Debug)]
pub struct StateContext {
    rip: usize,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for StateContext {
    fn default() -> Self {
        Self {
            rip: 0,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl StateContext {
    pub fn clear_rip() {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        if let Some(state_context) = hashmap.get_mut(&thread_id) {
            state_context.rip = 0;
        }
    }

    pub fn inc_rip() {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(StateContext::default);

        if let Some(state_context) = hashmap.get_mut(&thread_id) {
            state_context.rip += 1;
        }
    }

    pub fn get_rip() -> usize {
        let thread_id = current().id();
        let mut hashmap = CONTEXT.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(StateContext::default);

        if let Some(state_context) = hashmap.get(&thread_id) {
            state_context.rip
        } else {
            unreachable!();
        }
    }
}
