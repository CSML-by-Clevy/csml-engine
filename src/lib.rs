pub mod error_format;
pub mod interpreter;
pub mod parser;

use parser::{ast::*, Parser};
use std::collections::HashMap;
use error_format::data::ErrorInfo;
use serde_json::{Value, json, map::Map};
use interpreter::{ast_interpreter::interpret_block, csml_rules::*, data::Data, json_to_rust::*};

pub fn parse_file(file: String) -> Result<Flow, ErrorInfo> {
    match Parser::parse_flow(file.as_bytes()) {
        Ok(flow) => {
            check_valid_flow(&flow)?;
            Ok(flow)
        },
        Err(e) => Err(e),
    }
}

pub fn is_trigger(flow: &Flow, string: &str) -> bool {
    let info = flow.flow_instructions.get(&InstructionType::StartFlow);

    if let Some(Expr::VecExpr(vec, ..)) = info {
        for elem in vec.iter() {
            match elem {
                Expr::LitExpr(SmartLiteral {
                    literal: Literal::StringLiteral{value, ..},
                    ..
                }) if value.to_lowercase() == string.to_lowercase() => return true,
                _ => continue,
            }
        }
    }
    false
}

pub fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Expr> {
    flow.flow_instructions
        .get(&InstructionType::NormalStep(name.to_owned()))
}

pub fn version() -> String {
    "CsmlV2".to_owned()
}

pub fn execute_step(flow: &Flow, name: &str, mut data: Data) -> Result<String, ErrorInfo> {
    match search_for(flow, name) {
        Some(Expr::Block { arg: actions, .. }) => {
            let result = interpret_block(actions, &mut data)?;
            let mut message: Map<String, Value> = Map::new();
            let mut vec = vec![];
            let mut memories = vec![];

            for msg in result.messages.iter() {
                vec.push(msg.to_owned().message_to_json());
            }
            if let Some(mem) = result.memories {
                for elem in mem.iter() {
                    memories.push(elem.to_owned().memorie_to_jsvalue());
                }
            }

            message.insert("memories".to_owned(), Value::Array(memories));
            message.insert("messages".to_owned(), Value::Array(vec));
            message.insert("next_flow".to_owned(), match serde_json::to_value(result.next_flow) { Ok(val) => val, _ => json!(null)});
            message.insert("next_step".to_owned(), match serde_json::to_value(result.next_step) { Ok(val) => val, _ => json!(null)});

            match serde_json::to_string(&message) {
                Ok(msg) => Ok(msg),
                _ => unreachable!()
            } 
        },
        _ => Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: Empty Flow".to_string(),
        }),
    }
}

pub fn interpret(
    ast: &Flow,
    step_name: &str,
    memory: &Context,
    event: &Option<Event>,
) -> Result<String, ErrorInfo> {
    // if !check_valid_flow(ast) {
    //     return Err(ErrorInfo {
    //         interval: Interval { line: 0, column: 0 },
    //         message: "ERROR: invalid Flow".to_string(),
    //     });
    // }

    // dbg!(&ast);
    // let memory = context_to_memory(context);

    let data = Data {
        ast,
        memory,
        event,
        step_vars: HashMap::new(),
    };
    Ok(execute_step(ast, step_name, data)?)
}
