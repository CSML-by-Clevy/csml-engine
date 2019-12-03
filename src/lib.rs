pub mod error_format;
pub mod interpreter;
pub mod parser;

use error_format::data::ErrorInfo;
use interpreter::{
    ast_interpreter::{interpret_scope, interpret_scope_at_index},
    csml_rules::check_valid_flow,
    data::Data,
    json_to_rust::{Context, Event},
    message::MessageData,
};
use parser::{ast::*, literal::*, Parser};
use std::collections::HashMap;

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
    index: Option<i64>,
) -> Result<MessageData, ErrorInfo> {
    match (search_for(flow, name), index) {
        (Some(Expr::Scope { scope, .. }), Some(index)) => {
            let mut msg_data = interpret_scope_at_index(scope, &mut data, index)?;
            // tmp
            if !data.step_vars.is_empty() {
                msg_data.step_vars = Some(data.step_vars);
            }
            Ok(msg_data)
        }
        (
            Some(Expr::Scope {
                scope, block_type, ..
            }),
            None,
        ) => {
            let mut msg_data = interpret_scope(block_type, scope, &mut data)?;
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
    index: Option<i64>,
) -> MessageData {
    let data = Data {
        ast,
        memory,
        event,
        step_vars: match step_vars {
            Some(vars) => vars,
            None => HashMap::new(),
        },
    };

    MessageData::error_to_message(execute_step(ast, step_name, data, index))
}
