pub mod error_format;
pub mod interpreter;
pub mod parser;

use curl::easy::Easy;
use error_format::data::ErrorInfo;
use interpreter::{
    ast_interpreter::interpret_scope,
    csml_rules::check_valid_flow,
    data::Data,
    json_to_rust::{
        Context,
        Event
    },
    message::{
        MessageData,
        MSG},
};
use parser::{ast::*, literal::*, Parser};
use std::collections::HashMap;
use std::sync::mpsc;

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
    instruction_index: usize,
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

pub fn interpret(
    ast: &Flow,
    step_name: &str,
    memory: &mut Context,
    event: &Option<Event>,
    step_vars: Option<HashMap<String, Literal>>,
    instruction_index: usize,
    sender: Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let curl = Easy::new();
    let data = Data {
        ast,
        memory,
        event,
        curl,
        step_vars: match step_vars {
            Some(vars) => vars,
            None => HashMap::new(),
        },
    };

    MessageData::error_to_message(execute_step(ast, &step_name, data, instruction_index, &sender), &sender)
}
