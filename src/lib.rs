pub mod data;
pub mod error_format;
pub mod interpreter;
pub mod linter;
pub mod parser;

use interpreter::interpret_scope;
use parser::parse_flow;

use crate::data::error_info::ErrorInfo;
use crate::data::context::get_hashmap;
use crate::data::Data;
use crate::data::ast::Flow;
use crate::data::ast::InstructionType;
use crate::data::ast::Expr;
use crate::data::csml_bot::CsmlBot;
use crate::data::ContextJson;
use crate::data::event::Event;
use crate::data::msg::MSG;
use crate::data::execution_context::ExecutionContext;
use crate::data::message_data::MessageData;

use curl::easy::Easy;
use std::collections::HashMap;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_file(flow: &str) -> Result<Flow, ErrorInfo> {
    match parse_flow(flow) {
        Ok(flow) => Ok(flow),
        Err(e) => Err(e),
    }
}

// TODO: Optimisation of ast
/*
    - Instead of parsing the entire flow for just execution one step either:
        - parse only step by step
        - parse all flows at once 
*/

pub fn interpret(bot: CsmlBot, context: ContextJson, event: Event, sender: Option<mpsc::Sender<MSG>>) -> MessageData {
    ExecutionContext::set_flow(&bot.default_flow);
    ExecutionContext::set_step("start");

    loop {
        let flow = ExecutionContext::get_flow();
        let step = ExecutionContext::get_step();

        println!("[+] current flow to be executed: {}", flow);
        println!("[+] current step to be executed: {}", step);

        let content = match bot.get_flow(&flow) {
            Ok(result) => result,
            Err(_) => {
                unimplemented!();
            }
        };

        let flow: Flow = match parse_file(&content) {
            Ok(result) => result,
            Err(_) => {
                unimplemented!();
            }
        };

        let step_vars = match &context.hold {
            Some(hold) => get_hashmap(&hold.step_vars),
            None => HashMap::new(),
        };

        let mut data = Data {
            flow: &flow,
            context: &mut context.to_literal(),
            event: &event,
            curl: Easy::new(),
            step_vars,
        };

        let rip = match &context.hold {
            Some(result) => Some(result.index),
            None => None,
        };

        match flow.flow_instructions.get(&InstructionType::NormalStep(step)) {
            Some(Expr::Scope { scope, .. }) => {
                println!("{:#?}", interpret_scope(scope, &mut data, rip, &sender));
            }
            _ => {
                unimplemented!();
            }
        }
    }
}
