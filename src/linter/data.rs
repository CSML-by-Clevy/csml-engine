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
pub struct Position {
    flow: String,
    step: String,
    // add line, column ???
}

#[derive(Debug, Clone)]
pub struct Goto {
    pub flow: String,
    pub step: String,
    pub interval: Interval,
}

#[derive(Debug, Clone)]
pub struct Warning {
    pub message: &'static str,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct Linter {
    pub flow: HashMap<String, HashMap<String, Vec<Interval>>>,
    pub goto: Vec<Goto>,
    pub position: Position,
    pub warnings: Vec<Warning>,
    // Todo: add function, var, unreachable_code inside linter
    // pub var: HashMap<String, Vec<Variable>>,
    // pub function: HashMap<String, Vec<Function>>,
    // pub unreachable_code: Vec<Code>
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Default for Position {
    fn default() -> Self {
        Self {
            flow: String::default(),
            step: String::default(),
        }
    }
}

impl Default for Linter {
    fn default() -> Self {
        Self {
            position: Position::default(),
            flow: HashMap::default(),
            goto: Vec::default(),
            warnings: Vec::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Goto {
    fn new(flow: &str, step: &str, interval: Interval) -> Self {
        Self {
            flow: flow.to_owned(),
            step: step.to_owned(),
            interval,
        }
    }
}

impl Position {
    fn clear(&mut self) {
        self.flow.clear();
        self.step.clear();
    }
}

impl Linter {
    fn add_flow(&mut self, flow: &str) {
        self.flow
            .entry(flow.to_owned())
            .or_insert_with(|| HashMap::default());
    }

    fn add_step(&mut self, flow: &str, step: &str, interval: Interval) {
        self.flow
            .entry(flow.to_owned())
            .or_insert_with(|| HashMap::default());

        if let Some(hashmap_step) = self.flow.get_mut(flow) {
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

    fn add_goto(&mut self, flow: &str, step: &str, interval: Interval) {
        self.goto.push(Goto::new(flow, step, interval));
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Linter {
    pub fn set_flow(flow: &str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter.position.flow.clear();
            linter.position.flow.push_str(flow);
            linter.add_flow(flow);
        }
    }

    pub fn set_step(flow: &str, step: &str, interval: Interval) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter.position.step.clear();
            linter.position.step.push_str(step);
            linter.add_step(flow, step, interval);
        }
    }

    pub fn set_goto(flow: &str, step: &str, interval: Interval) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            linter.add_goto(flow, step, interval);
        }
    }

    pub fn add_warning(message: &'static str) {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get_mut(&thread_id) {
            let warning = Warning {
                position: linter.position.clone(),
                message,
            };
            linter.warnings.push(warning);
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
            linter.position.clear();
        }
    }
}

impl Linter {
    pub fn get_flow() -> String {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get(&thread_id) {
            linter.position.flow.to_owned()
        } else {
            unreachable!();
        }
    }

    pub fn get_step() -> String {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get(&thread_id) {
            linter.position.step.to_owned()
        } else {
            unreachable!();
        }
    }

    pub fn get() -> Linter {
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

    // tmp
    pub fn print_warnings() {
        let thread_id = current().id();
        let mut hashmap = HASHMAP.lock().unwrap();

        hashmap
            .entry(thread_id)
            .or_insert_with(|| Linter::default());

        if let Some(linter) = hashmap.get(&thread_id) {
            println!("warnings: {:?}", linter.warnings);
        } else {
            unreachable!();
        }
    }
}
