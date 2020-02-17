use lazy_static::*;
use std::collections::*;
use std::sync::*;
use std::thread::*;

lazy_static! {
    static ref CONTEXT: Mutex<StateContext> = Mutex::new(StateContext {
        state: HashMap::new(),
        warning: Vec::new(),
        index: HashMap::new(),
    });
}

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub enum ExitCondition {
    Goto,
    Error,
    Break,
    Hold,
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Normal,
    Loop,
}

// TODO: Check for usize overflow vuln !
#[derive(Debug)]
pub struct StateContext {
    state: HashMap<ThreadId, Vec<State>>,
    warning: Vec<String>,
    index: HashMap<ThreadId, usize>,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl StateContext {
    pub fn clear_index() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().index;

        if hashmap.contains_key(&thread_id) {
            hashmap.remove(&thread_id);
        }
    }

    pub fn inc_index() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().index;

        hashmap.entry(thread_id).or_insert(0);

        let result: usize = *(hashmap.get(&thread_id).unwrap());

        hashmap.insert(thread_id, result + 1);
    }

    pub fn get_index() -> usize {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().index;

        hashmap.entry(thread_id).or_insert(0);

        let result: usize = *(hashmap.get(&thread_id).unwrap());

        result
    }

    pub fn clear_state() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        if hashmap.contains_key(&thread_id) {
            hashmap.remove(&thread_id);
        }
    }

    pub fn set_state(state: State) {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        hashmap.entry(thread_id).or_insert_with(Vec::new);

        match state {
            State::Loop => {
                hashmap.get_mut(&thread_id).unwrap().push(state);
            }
            State::Normal => {
                hashmap.get_mut(&thread_id).unwrap().pop();
            }
        }
    }

    pub fn get_state() -> State {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        if !hashmap.contains_key(&thread_id) {
            return State::Normal;
        }

        if hashmap.get(&thread_id).unwrap().is_empty() {
            return State::Normal;
        }

        State::Loop
    }
}
