pub mod interpreter;
pub mod lexer;
pub mod parser;

use parser::ast::*;
use interpreter::csml_rules::*;
use interpreter::json_to_rust::*;
use interpreter::ast_interpreter::AstInterpreter;
use lexer::{Lexer, token::Tokens};
use parser::{Parser};

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

pub fn parse_file(file: String) -> IoResult<Flow> {
    let lex_tokens = Lexer::lex_tokens(file.as_bytes());

    match lex_tokens {
        Ok((_complete, t)) => {
            let tokens = Tokens::new(&t);

            match Parser::parse_tokens(tokens) {
                Ok(flow) => Ok(flow),
                Err(e)   => Err(Error::new(ErrorKind::Other, format!("Error in Paring AST {:?}", e)))
            }
        }
        Err(e) => Err(Error::new(ErrorKind::Other, format!("Problem in Lexing Tokens -> {:?}", e))),
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

pub fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Step> {
    for step in flow.steps.iter() {
        match step {
            Step{ label, ..} if check_ident(label, name) => {
                return Some(step)
            }
            _ => continue,
        }
    }

    None
}

pub fn execute_step(flow: &Flow, name: &str, interpreter: AstInterpreter) -> IoResult<String> {
    match search_for(flow, name) {
        Some(Step{ label: _, actions}) => {
            let result = interpreter.match_block(actions)?;
            let ser = serde_json::to_string(&result)?;

            Ok(ser)
        }
        _ => Err(Error::new(ErrorKind::Other, "Error Empty Flow")),
    }
}

pub fn interpret(ast: &Flow, step_name: &str, context: &JsContext, event: &Option<Event>) -> IoResult<String> {
    if !check_valid_flow(ast) {
        return Err(Error::new(ErrorKind::Other, "The Flow is not valid it need a start/end Labels, a Accept Flow, and each label must be unique"));
    }

    let memory = context_to_memory(context);
    let intpreter = AstInterpreter{ memory: &memory, event};

    Ok(execute_step(ast, step_name, intpreter)?)
}
