pub mod interpreter;
pub mod parser;

use interpreter::{ast_interpreter::interpret_block, csml_rules::*, data::Data, json_to_rust::*};
use parser::{ast::*, ErrorInfo, Parser};
use std::collections::HashMap;

use multimap::MultiMap;

pub fn add_to_memory(memory: &mut MultiMap<String, MemoryType>, vec: &[serde_json::Value]) {
    for value in vec.iter() {
        let memory_value: Result<MemoryType, _> = serde_json::from_value(value.clone());
        match memory_value {
            Ok(memory_value) => memory.insert(memory_value.key.clone(), memory_value),
            Err(e) => println!(
                "value is not of fomrat MemoryType {:?} error -> {:?}",
                value, e
            ), // error to the api
        }
    }
}

pub fn parse_file(file: String) -> Result<Flow, ErrorInfo> {
    // add flow validations
    match Parser::parse_flow(file.as_bytes()) {
        Ok(flow) => Ok(flow),
        Err(e) => Err(e),
    }
}

pub fn is_trigger(flow: &Flow, string: &str) -> bool {
    let info = flow
        .flow_instructions
        .get(&InstructionType::StartFlow);

    if let Some(Expr::VecExpr(vec)) = info {
        for elem in vec.iter() {
            match elem {
                Expr::LitExpr(Literal::StringLiteral(tag), ..)
                    if tag.to_lowercase() == string.to_lowercase() => return true,
                _                                                  => continue,
            }
        }
    }
    false
}

pub fn context_to_memory(context: &JsContext) -> Memory {
    let mut memory = Memory {
        past: MultiMap::new(),
        current: MultiMap::new(),
        metadata: MultiMap::new(),
    };

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

pub fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Expr> {
    flow.flow_instructions
        .get(&InstructionType::NormalStep(name.to_string()))
}

pub fn execute_step(flow: &Flow, name: &str, mut data: Data) -> Result<String, ErrorInfo> {
    match search_for(flow, name) {
        Some(Expr::Block { arg: actions, .. }) => {
            // let result = interpreter.interpret_block(actions)?;
            let result = match interpret_block(actions, &mut data) {
                Ok(val) => val,
                Err(e) => {
                    return Err(ErrorInfo {
                        line: 0,
                        column: 0,
                        message: e,
                    })
                }
            };
            dbg!(&result);
            match serde_json::to_string(&result) {
                Ok(ser) => Ok(ser),
                Err(_) => unreachable!(),
            }
        }
        _ => Err(ErrorInfo {
            line: 0,
            column: 0,
            message: "ERROR: Empty Flow".to_string(),
        }),
    }
}

pub fn interpret(ast: &Flow, step_name: &str, context: &JsContext, event: &Option<Event>) -> Result<String, ErrorInfo> {
    if !check_valid_flow(ast) {
        return Err(ErrorInfo {
            line: 0,
            column: 0,
            message: "ERROR: invalid Flow".to_string(),
        });
    }

    dbg!(&ast);

    let memory = context_to_memory(context);
    let data = Data {
        ast,
        memory: &memory,
        event,
        step_vars: HashMap::new(),
    };
    Ok(execute_step(ast, step_name, data)?)
}
