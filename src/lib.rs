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

fn search_for<'a>(flow: &'a Flow, name: &str) -> Option<&'a Expr> {
    flow.flow_instructions
        .get(&InstructionType::NormalStep(name.to_owned()))
}

fn execute_step(
    flow: &Flow,
    name: &str,
    mut data: Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match search_for(flow, name) {
        Some(Expr::Scope { scope, .. }) => {
            interpret_scope(scope, &mut data, instruction_index, sender)
        }
        _ => Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: format!("Error: step {} not found", name),
        }),
    }
}

pub fn parse_file(file: &str) -> Result<Flow, ErrorInfo> {
    match Parser::parse_flow(file) {
        Ok(flow) => {
            check_valid_flow(&flow)?;
            Ok(flow)
        }
        Err(e) => Err(e),
    }
}

pub fn interpret(
    flow: &str,
    step_name: &str,
    context: ContextJson,
    event: &Event,
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
    let mut context = context.to_literal();
    let step_vars = match &context.hold {
        Some(hold) => get_hashmap(&hold.step_vars),
        None => HashMap::new(),
    };
    let instruction_index = if let Some(hold) = &context.hold {
        Some(hold.index)
    } else {
        None
    };
    let data = Data {
        ast: &ast,
        context: &mut context,
        event,
        curl,
        step_vars,
    };

    MessageData::error_to_message(
        execute_step(&ast, &step_name, data, instruction_index, &sender),
        &sender,
    )
}
