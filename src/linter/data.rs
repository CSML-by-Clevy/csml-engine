// use crate::data::ast::Interval;
// use lazy_static::*;
// use std::collections::*;
// use std::sync::*;
// use std::thread::*;
// use crate::data::position::Position;

// ////////////////////////////////////////////////////////////////////////////////
// // DATA STRUCTURES
// ////////////////////////////////////////////////////////////////////////////////

// lazy_static! {
//     static ref HASHMAP: Mutex<HashMap<ThreadId, Linter>> = Mutex::new(HashMap::default());
// }

// #[derive(Debug, Clone)]
// pub struct Warning {
//     pub message: &'static str,
//     pub position: Position,
// }

// #[derive(Debug, Clone)]
// pub struct Linter {
//     pub flow: HashMap<String, HashMap<String, Vec<Interval>>>,
//     pub warnings: Vec<Warning>,
// }

// ////////////////////////////////////////////////////////////////////////////////
// // TRAIT FUNCTIONS
// ////////////////////////////////////////////////////////////////////////////////

// impl Default for Linter {
//     fn default() -> Self {
//         Self {
//             flow: HashMap::default(),
//             warnings: Vec::default(),
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////
// // PRIVATE FUNCTIONS
// ////////////////////////////////////////////////////////////////////////////////

// impl Warning {
//     fn new(interval: Interval, message: &'static str) -> Self {
//         Self {
//             message,
//             position: Position::new(interval)
//         }
//     }
// }

// ////////////////////////////////////////////////////////////////////////////////
// // PUBLIC FUNCTIONS
// ////////////////////////////////////////////////////////////////////////////////

// impl Linter {
//     pub fn add_warning(message: &'static str, interval: Interval) {
//         let thread_id = current().id();
//         let mut hashmap = HASHMAP.lock().unwrap();

//         hashmap
//             .entry(thread_id)
//             .or_insert_with(|| Linter::default());

//         if let Some(linter) = hashmap.get_mut(&thread_id) {
//             linter.warnings.push(Warning::new(interval, message));
//         }
//     }

//     pub fn clear() {
//         let thread_id = current().id();
//         let mut hashmap = HASHMAP.lock().unwrap();

//         hashmap
//             .entry(thread_id)
//             .or_insert_with(|| Linter::default());

//         if let Some(linter) = hashmap.get_mut(&thread_id) {
//             linter.flow.clear();
//             linter.warnings.clear();
//         }
//     }
// }

// impl Linter {
//     pub fn get_linter() -> Linter {
//         let thread_id = current().id();
//         let mut hashmap = HASHMAP.lock().unwrap();

//         hashmap
//             .entry(thread_id)
//             .or_insert_with(|| Linter::default());

//         if let Some(linter) = hashmap.get(&thread_id) {
//             (*linter).to_owned()
//         } else {
//             unreachable!();
//         }
//     }

//     pub fn get_warnings() -> Vec<Warning> {
//         let thread_id = current().id();
//         let mut hashmap = HASHMAP.lock().unwrap();

//         hashmap
//             .entry(thread_id)
//             .or_insert_with(|| Linter::default());

//         if let Some(linter) = hashmap.get(&thread_id) {
//             linter.warnings.to_owned()
//         } else {
//             unreachable!();
//         }
//     }
// }
