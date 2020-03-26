pub mod data;
pub mod error_format;
pub mod interpreter;
pub mod linter;
pub mod parser;

use crate::data::context::get_hashmap;
use crate::linter::linter::linter;
use crate::linter::Linter;
use data::{ast::*, ContextJson, Data, Event, MessageData, MSG};
use error_format::*;
use interpreter::interpret_scope;
use parser::parse_flow;
use parser::state_context::StateContext;

use curl::easy::Easy;
use std::{collections::HashMap, sync::mpsc};

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
            interpret_scope(scope, &mut data, instruction_index, sender)
        }
        _ => Err(gen_error_info(
            Interval { line: 0, column: 0 },
            format!("{} {}", name, ERROR_STEP_EXIST),
        )),
    }
}

pub fn parse_file(file: &str) -> Result<Flow, ErrorInfo> {
    // TODO: receive more than just the flow to be able to get real flow id
    Linter::clear();
    Linter::set_flow("default_flow");

    match parse_flow(file) {
        Ok(flow) => {
            let mut error = Vec::new();

            linter(&mut error);
            Linter::print_warnings();

            // TODO: tmp check until error handling
            match error.is_empty() {
                true => Ok(flow),
                false => Err(error.first().unwrap().to_owned()),
            }
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
            StateContext::clear_state();
            StateContext::clear_rip();

            return MessageData::error_to_message(
                Err(gen_error_info(
                    Interval { line: 0, column: 0 },
                    format!("{} {}", ERROR_INVALID_FLOW, e.message),
                )),
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
