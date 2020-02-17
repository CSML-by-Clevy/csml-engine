pub mod data;
pub mod error_format;
pub mod interpreter;
pub mod parser;

use crate::data::context::get_hashmap;
use data::{ast::*, ContextJson, Data, Event, MessageData, MSG};
use error_format::ErrorInfo;
use interpreter::interpret_scope;
use parser::csml_rules::check_valid_flow;
use parser::Parser;

use curl::easy::Easy;
use std::{collections::HashMap, sync::mpsc};

pub fn parse_file(file: &str) -> Result<Flow, ErrorInfo> {
    match Parser::parse_flow(file) {
        Ok(flow) => {
            check_valid_flow(&flow)?;
            Ok(flow)
        }
        Err(e) => Err(e),
    }
}

pub fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Expr> {
    flow.flow_instructions
        .get(&InstructionType::NormalStep(name.to_owned()))
}

pub fn execute_step(
    flow: &Flow,
    name: &str,
    mut data: Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match search_for(flow, name) {
        Some(Expr::Scope { scope, .. }) => {
            let mut msg_data = interpret_scope(scope, &mut data, instruction_index, sender)?;
            // tmp
            if !data.step_vars.is_empty() {
                msg_data.step_vars = Some(data.step_vars);
            }
            Ok(msg_data)
        }
        _ => Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: format!("Error: step {} not found", name),
        }),
    }
}

// HashMap<String, Literal>
pub fn interpret(
    flow: &str,
    step_name: &str,
    memory: ContextJson,
    event: &Option<Event>,
    step_vars: Option<serde_json::Value>,
    instruction_index: Option<usize>,
    sender: Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let ast: Flow = match parse_file(flow) {
        Ok(flow) => flow,
        Err(e) => {
            return MessageData::error_to_message(
                Err(ErrorInfo {
                    message: format!("Error in parsing Flow: {:?}", e),
                    interval: Interval { line: 0, column: 0 },
                }),
                &sender,
            );
        }
    };

    let curl = Easy::new();
    let mut memory = memory.to_literal();
    let data = Data {
        ast: &ast,
        memory: &mut memory,
        event,
        curl,
        step_vars: match step_vars {
            Some(vars) => get_hashmap(&vars),
            None => HashMap::new(),
        },
    };

    MessageData::error_to_message(
        execute_step(&ast, &step_name, data, instruction_index, &sender),
        &sender,
    )
}
