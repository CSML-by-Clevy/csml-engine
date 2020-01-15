use lazy_static::*;
use std::sync::*;
use std::thread::*;
use std::collections::*;

lazy_static! {
    static ref CONTEXT: Mutex<Context> = Mutex::new(Context{
        state: HashMap::new(),
        warning: Vec::new(),
        index: HashMap::new(),
    });
}

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES 
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub enum State {
    Normal,
    Loop,
}

/// TODO: Check for usize ofverflow vuln !
#[derive(Debug)]
pub struct Context {
    state: HashMap<ThreadId, Vec<State>>,
    warning: Vec<String>,
    index: HashMap<ThreadId, usize>,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Context {
    pub fn clear_index() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().index;

        if hashmap.contains_key(&thread_id) == true {
            hashmap.remove(&thread_id);
        }
    }

    pub fn inc_index() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().index;

        if hashmap.contains_key(&thread_id) == false {
            hashmap.insert(thread_id, 0);
        }

        let result: usize = *(hashmap.get(&thread_id).unwrap());

        hashmap.insert(thread_id, result + 1);
    }

    pub fn get_index() -> usize {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().index;

        if hashmap.contains_key(&thread_id) == false {
            hashmap.insert(thread_id, 0);
        }

        let result: usize = *(hashmap.get(&thread_id).unwrap());

        return result;
    }

    pub fn clear_state() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        if hashmap.contains_key(&thread_id) == true {
            hashmap.remove(&thread_id);
        }
    }

    pub fn set_state(state: State) {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        if hashmap.contains_key(&thread_id) == false {
            hashmap.insert(thread_id, Vec::new());
        }

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

        if hashmap.contains_key(&thread_id) == false {
            return State::Normal;
        }

        if hashmap.get(&thread_id).unwrap().is_empty() {
            return State::Normal;
        }
       
        return State::Loop;
    }
}