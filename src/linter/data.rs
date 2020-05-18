use crate::data::ast::Interval;

use lazy_static::*;
use std::collections::*;
use std::sync::*;
use std::thread::*;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<ThreadId, Linter>> = Mutex::new(HashMap::default());
}

#[derive(Debug, Clone)]
pub struct Goto {
    pub src_flow: String,
    pub src_step: String,
    pub dst_flow: String,
    pub dst_step: String,
    pub interval: Interval,
}

#[derive(Debug, Clone)]
pub struct Linter {
    pub flow: HashMap<String, HashMap<String, Vec<Interval>>>, // can be change ?
    pub goto: Vec<Goto>,
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for Linter {
    fn default() -> Self {
        Self {
            flow: HashMap::default(),
            goto: Vec::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Goto {
    fn new(
        src_flow: &str,
        src_step: &str,
        dst_flow: &str,
        dst_step: &str,
        interval: Interval,
    ) -> Self {
        Self {
            src_flow: src_flow.to_owned(),
            src_step: src_step.to_owned(),
            dst_flow: dst_flow.to_owned(),
            dst_step: dst_step.to_owned(),
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Linter {
    pub fn add_flow(flow: &str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter
                .flow
                .entry(flow.to_owned())
                .or_insert_with(|| HashMap::default());
        }
    }

    pub fn add_step(flow: &str, step: &str, interval: Interval) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter
                .flow
                .entry(flow.to_owned())
                .or_insert_with(|| HashMap::default());

            if let Some(hashmap_step) = linter.flow.get_mut(flow) {
                match hashmap_step.get_mut(step) {
                    Some(vector_step) => {
                        vector_step.push(interval);
                    }
                    None => {
                        hashmap_step.insert(step.to_owned(), vec![interval]);
                    }
                }
            }
        }
    }

    pub fn add_goto(
        src_flow: &str,
        src_step: &str,
        dst_flow: &str,
        dst_step: &str,
        interval: Interval,
    ) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter
                .goto
                .push(Goto::new(src_flow, src_step, dst_flow, dst_step, interval));
        }
    }

    pub fn clear() {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter.flow.clear();
            linter.goto.clear();
        }
    }
}

impl Linter {
    pub fn get_linter() -> Linter {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get(&thread_id) {
            (*linter).to_owned()
        } else {
            unreachable!();
        }
    }
}
