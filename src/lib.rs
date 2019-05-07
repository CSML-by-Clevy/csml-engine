pub mod interpreter;
pub mod lexer;
pub mod parser;

use parser::ast::*;
use interpreter::csml_rules::*;
use interpreter::json_to_rust::*;
use interpreter::ast_interpreter::AstInterpreter;

use std::io::{Error, ErrorKind, Result as IoResult};
use multimap::MultiMap;

pub fn add_to_memory(memory: &mut MultiMap<String, MemoryType>, vec: &[serde_json::Value]) {

    for value in vec.iter() {
        let memory_value: Result<MemoryType, _> = serde_json::from_value(value.clone()); 
        match memory_value {
            Ok(memory_value)              => memory.insert(memory_value.key.clone(), memory_value),
            Err(e)                        => println!("value is not of fomrat MemoryType {:?} error -> {:?}", value, e), // error to the api
        }
    }
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

pub fn context_to_memory(context: &JsContext) -> Memory {
    let mut memory = Memory {past: MultiMap::new(), current: MultiMap::new(), metadata: MultiMap::new()};

    if let Some(ref vec) = context.past {
        add_to_memory(&mut memory.past, vec);
    }
    if let Some(ref vec) = context.metadata {
        add_to_memory(&mut memory.metadata, vec);
    }
    if let Some(ref vec) = context.current {
        add_to_memory(&mut memory.current, vec);
    }
    memory
}

pub fn search_for(flow: &Flow, name: &str, interpreter: AstInterpreter) -> IoResult<String> {
    for step in flow.steps.iter() {
        match step {
            Step{label, actions} if check_ident(label, name) => {
                let result = interpreter.match_block(actions)?;
                let ser = serde_json::to_string(&result)?;

                return Ok(ser);
            }
            _ => continue,
        }
    }

    Err(Error::new(ErrorKind::Other, "Error Empty Flow"))
}

pub fn interpret(ast: &Flow, step_name: &str, context: &JsContext, event: &Option<Event>) -> IoResult<String> {
    if !check_valid_flow(ast) {
        return Err(Error::new(ErrorKind::Other, "The Flow is not valid it need a start/end Labels, a Accept Flow, and each label must be unique"));
    }

    let memory = context_to_memory(context);
    let intpreter = AstInterpreter{ memory: &memory, event};

    Ok(search_for(ast, step_name, intpreter)?)
}
