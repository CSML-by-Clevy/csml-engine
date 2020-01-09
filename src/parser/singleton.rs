use lazy_static::*;
use std::sync::*;
use std::thread::*;
use std::collections::*;

lazy_static! {
    static ref CONTEXT: Mutex<Context> = Mutex::new(Context{
        state: HashMap::new(),
        warning: Vec::new(),
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

#[derive(Debug)]
pub struct Context {
    state: HashMap<ThreadId, Vec<State>>,
    warning: Vec<String>,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl State {
    pub fn set(self) {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        if hashmap.contains_key(&thread_id) == false {
            hashmap.insert(thread_id, Vec::new());
        }

        match self {
            State::Loop => {
                hashmap.get_mut(&thread_id).unwrap().push(self);
            }
            State::Normal => {
                hashmap.get_mut(&thread_id).unwrap().pop();
            }
        }
    }

    pub fn get() -> Self {
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

    pub fn clear() {
        let thread_id = current().id();
        let hashmap = &mut CONTEXT.lock().unwrap().state;

        if hashmap.contains_key(&thread_id) == true {
            hashmap.get_mut(&thread_id).unwrap().clear();
        }
    }
}