pub mod ast_interpreter;
pub mod builtins;
pub mod json_to_rust;
pub mod variable_handler;

pub use json_to_rust::{json_to_literal, memory_to_literal};

use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::{ast::*, Data, Hold, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::{for_loop, match_actions, solve_if_statement},
    variable_handler::interval::interval_from_expr,
};
use crate::parser::ExitCondition;

use nom::lib::std::collections::HashMap;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn step_vars_to_json(map: HashMap<String, Literal>) -> serde_json::Value {
    let mut json_map = serde_json::Map::new();

    for (key, val) in map.iter() {
        let content_type = &val.content_type;
        json_map.insert(key.to_owned(), val.primitive.format_mem(content_type, true));
    }

    serde_json::json!(json_map)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn interpret_scope(
    actions: &Block,
    data: &mut Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let mut message_data = MessageData::default();

    for (action, instruction_info) in actions.commands.iter() {
        let instruction_total = instruction_info.index + instruction_info.total;

        if let Some(instruction_index) = instruction_index {
            if instruction_index >= instruction_total {
                continue;
            }
        }

        if message_data.exit_condition.is_some() {
            return Ok(message_data);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                message_data.exit_condition = Some(ExitCondition::Break);

                return Ok(message_data);
            }
            Expr::ObjectExpr(ObjectType::Hold(..)) => {
                message_data.exit_condition = Some(ExitCondition::Hold);

                let index = instruction_info.index;
                let map = data.step_vars.to_owned();
                let hold = Hold::new(index, step_vars_to_json(map));

                message_data.hold = Some(hold.to_owned());

                MSG::send(&sender, MSG::Hold(hold));

                return Ok(message_data);
            }
            Expr::ObjectExpr(fun) => {
                message_data = match_actions(fun, message_data, data, instruction_index, &sender)?
            }
            Expr::IfExpr(ref if_statement) => {
                message_data = solve_if_statement(
                    if_statement,
                    message_data,
                    data,
                    instruction_index,
                    instruction_info,
                    &sender,
                )?;
            }
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                message_data = for_loop(
                    ident,
                    i,
                    expr,
                    block,
                    range,
                    message_data,
                    data,
                    instruction_index,
                    &sender,
                )?
            }
            e => {
                // TODO: make Expr printable in order to be included in the error message
                return Err(gen_error_info(
                    Position::new(interval_from_expr(e)),
                    ERROR_START_INSTRUCTIONS.to_owned(),
                ));
            }
        };
    }

    if message_data.exit_condition.is_none() {
        message_data.exit_condition = Some(ExitCondition::End);
        data.context.step = "end".to_string();
    }

    Ok(message_data)
}
