pub mod message;
pub mod csml_rules;
pub mod ast_interpreter;
pub mod builtins;
pub mod json_to_rust;

use crate::parser::ast::*;
use csml_rules::*;
use ast_interpreter::AstInterpreter;
use json_to_rust::*;
use multimap::MultiMap;

// use std::io::*;
// use std::io::{Error, ErrorKind};

pub struct Interpreter {
}

impl Interpreter {
    pub fn check_valid_flow(flow: &Flow) -> bool {
        let mut accept_flow = false;
        let mut start = false;

        if !flow.accept.is_empty() { accept_flow = true; }
        for step in flow.steps.iter() {
            match step {
                Step::Block { label, .. }       =>
                    match label {
                        Ident(t) if t == "start"    => start = true,
                        _                           => {}
                    },
                _                               => continue
            }
        }
        start && accept_flow && double_label(&flow.steps)
    }

    pub fn is_trigger(flow: &Flow, string: &str) -> bool {
        for elem in flow.accept.iter() {
            match elem {
                Expr::LitExpr(Literal::StringLiteral(tag)) if tag == string => return true,
                _                                                           => continue,
            }
        }
        false
    }

    // return Result<struct, error>
    pub fn search_for(flow: &Flow, name: &str, preter: AstInterpreter) -> Option<String> {
        for step in flow.steps.iter() {
            match step {
                Step::Block { label, actions } if check_ident(label, name) => {
                    let result = preter.match_block(actions).unwrap();
                    let ser = serde_json::to_string(&result).unwrap();
                    // println!("--|--|----> {}", ser);
                    return Some(ser);
                }
                _ => continue,
            }
        }
        None
    }

    pub fn add_to_memory(memory: &mut MultiMap<String, MemoryType>, vec: &Vec<serde_json::Value>) {

        for value in vec.iter() {
            let memory_value: Result<MemoryType, _> = serde_json::from_value(value.clone()); 
            match memory_value {
                Ok(memory_value)              => memory.insert(memory_value.key.clone(), memory_value),
                Err(e)                        => println!("value is not of fomrat MemoryType {:?} error -> {:?}", value, e),
            }
        }
    }

    pub fn contex_to_memory(context: &JsContext) -> Memory {
        let mut memory = Memory {past: MultiMap::new(), current: MultiMap::new(), metadata: MultiMap::new()};

        if let Some(ref vec) = context.past {
            Interpreter::add_to_memory(&mut memory.past, vec);
        }
        if let Some(ref vec) = context.current {
            Interpreter::add_to_memory(&mut memory.current, vec);
        }
        if let Some(ref vec) = context.metadata {
            Interpreter::add_to_memory(&mut memory.metadata, vec);
        }
        memory
    }

    pub fn interpret(ast: &Flow, step_name: &str, context: &JsContext, event: &Option<Event>) -> String {
        if !Interpreter::check_valid_flow(ast) {
            return "The Flow is not valid it need a start/end Labels, a Accept Flow, and each label must be unique".to_owned();
        }

        let memory = Interpreter::contex_to_memory(context);
        let intpreter = AstInterpreter{ memory: &memory, event};

        match Interpreter::search_for(ast, step_name, intpreter) {
            Some(json) => json,
            None       => "error in step".to_owned()
        }
    }
}

// loop {
// match (read, &json) {
//     (Ok(..), Some(string)) => {
//         let deserialized: RootInterface = serde_json::from_str(&string).unwrap();
//         self.search_for(&deserialized.next_step.unwrap());
//     },
//     (Ok(ref string), None) if string.trim() == "flow"   => {
//         // check if flow can start
//         self.search_for("flow");
//         json = self.search_for("start");
//     },
//     (Ok(ref string), None) if string.trim() == "exit"   => break,
//     (Ok(_string), None)                                 => continue,
//     (Err(e), _)                                         => {
//         println!("Error => {:?}", e);
//         break;
//     }
// }
// }

// fn read_standar_in() -> Result<String> {
//     let mut buffer = String::new();
//     let stdin = stdin();
//     let mut handle = stdin.lock();

//     handle.read_to_string(&mut buffer)?;
//     Ok(buffer)
// }
