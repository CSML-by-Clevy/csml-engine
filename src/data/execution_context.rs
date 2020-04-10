use lazy_static::*;
use std::collections::*;
use std::sync::*;
use std::thread::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ExecutionContext {
    flow: String,
    step: String,
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ThreadId, ExecutionContext>> = Mutex::new(HashMap::default());
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            flow: String::default(),
            step: String::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl ExecutionContext {
    pub fn new(flow: String, step: String) -> Self {
        Self { flow, step }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl ExecutionContext {
    pub fn set_flow(flow: &str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| ExecutionContext::default());

        if let Some(execution_context) = hashmap.get_mut(&thread_id) {
            execution_context.flow = flow.to_owned();
        }
    }

    pub fn set_step(step: &str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| ExecutionContext::default());

        if let Some(execution_context) = hashmap.get_mut(&thread_id) {
            execution_context.step = step.to_owned();
        }
    }

    pub fn get_flow() -> String {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| ExecutionContext::default());

        if let Some(execution_context) = hashmap.get_mut(&thread_id) {
            execution_context.flow.to_owned()
        } else {
            unreachable!();
        }
    }

    pub fn get_step() -> String {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| ExecutionContext::default());

        if let Some(execution_context) = hashmap.get_mut(&thread_id) {
            execution_context.step.to_owned()
        } else {
            unreachable!();
        }
    }
}
