pub mod data;
pub mod error_format;
pub mod interpreter;
pub mod linter;
pub mod parser;

use interpreter::interpret_scope;
use parser::parse_flow;

use crate::data::ast::Expr;
use crate::data::ast::Flow;
use crate::data::ast::InstructionType;
use crate::data::ast::Interval;
use crate::data::context::get_hashmap;
use crate::data::csml_bot::CsmlBot;
use crate::data::error_info::ErrorInfo;
use crate::data::event::Event;
use crate::data::message_data::MessageData;
use crate::data::msg::MSG;
use crate::data::ContextJson;
use crate::data::Data;
use crate::error_format::*;

use curl::easy::Easy;
use std::collections::HashMap;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
/// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn execute_step(
    step: &str,
    mut data: &mut Data,
    rip: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let flow = data.flow.to_owned();

    let message_data = match flow
        .flow_instructions
        .get(&InstructionType::NormalStep(step.to_owned()))
    {
        Some(Expr::Scope { scope, .. }) => interpret_scope(scope, &mut data, rip, &sender),
        _ => Err(gen_error_info(
            Interval::new_as_u32(0, 0),
            format!("{} {}", step, ERROR_STEP_EXIST),
        )),
    };

    MessageData::error_to_message(message_data, sender)
}

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_file(flow: &str) -> Result<Flow, ErrorInfo> {
    match parse_flow(flow) {
        Ok(flow) => Ok(flow),
        Err(e) => Err(e),
    }
}

pub fn interpret(
    bot: CsmlBot,
    context: ContextJson,
    event: Event,
    sender: Option<mpsc::Sender<MSG>>,
) -> MessageData {
    let mut message_data = MessageData::default();

    let mut flow = context.flow.to_owned();
    let mut step = context.step.to_owned();

    while message_data.exit_condition.is_none() {
        println!("[+] current flow to be executed: {}", flow);
        println!("[+] current step to be executed: {}\n", step);

        let content = match bot.get_flow(&flow) {
            Ok(result) => result,
            Err(_) => {
                unimplemented!();
            }
        };

        let ast = match parse_file(&content) {
            Ok(result) => result,
            Err(_) => {
                unimplemented!();
            }
        };

        let step_vars = match &context.hold {
            Some(hold) => get_hashmap(&hold.step_vars),
            None => HashMap::new(),
        };

        let mut data = Data::new(
            &ast,
            &mut context.to_literal(),
            &event,
            Easy::new(),
            step_vars,
        );

        let rip = match &context.hold {
            Some(result) => Some(result.index),
            None => None,
        };

        message_data = message_data + execute_step(&step, &mut data, rip, &sender);

        flow = data.context.flow;
        step = data.context.step;
    }

    dbg!(&message_data);

    return message_data;
}
