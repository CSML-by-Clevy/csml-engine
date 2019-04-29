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

pub struct Interpreter {
}

impl Interpreter {
    pub fn add_to_memory(memory: &mut MultiMap<String, MemoryType>, vec: &[serde_json::Value]) {

        for value in vec.iter() {
            println!("value before memory insert {:?}", value);
            let memory_value: Result<MemoryType, _> = serde_json::from_value(value.clone()); 
            match memory_value {
                Ok(memory_value)              => memory.insert(memory_value.key.clone(), memory_value),
                Err(e)                        => println!("value is not of fomrat MemoryType {:?} error -> {:?}", value, e),
            }
        }
    }

    pub fn context_to_memory(context: &JsContext) -> Memory {
        let mut memory = Memory {past: MultiMap::new(), current: MultiMap::new(), metadata: MultiMap::new()};

        if let Some(ref vec) = context.past {
            Interpreter::add_to_memory(&mut memory.past, vec);
        }
        if let Some(ref vec) = context.metadata {
            Interpreter::add_to_memory(&mut memory.metadata, vec);
        }
        if let Some(ref vec) = context.current {
            Interpreter::add_to_memory(&mut memory.current, vec);
        }
        memory
    }

    pub fn interpret(ast: &Flow, step_name: &str, context: &JsContext, event: &Option<Event>) -> String {
        if !check_valid_flow(ast) {
            return "The Flow is not valid it need a start/end Labels, a Accept Flow, and each label must be unique".to_owned();
        }

        let memory = Interpreter::context_to_memory(context);
        let intpreter = AstInterpreter{ memory: &memory, event};

        match search_for(ast, step_name, intpreter) {
            Some(json) => json,
            None       => "error in step".to_owned()
        }
    }
}
