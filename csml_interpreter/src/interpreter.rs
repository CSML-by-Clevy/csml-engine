pub mod ast_interpreter;
pub mod builtins;
pub mod components;
pub mod function_scope;
pub mod json_to_rust;
pub mod variable_handler;

pub use json_to_rust::{json_to_literal, memory_to_literal};

use crate::{clean_step::clean_step_intervals};
use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::{ast::*, Data, Hold, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::{for_loop, match_actions, solve_if_statement},
    variable_handler::{expr_to_literal, interval::interval_from_expr},
};
use crate::parser::ExitCondition;

use md5::{Digest, Md5};
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
    mut instruction_index: Option<(usize, Vec<usize>)>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let mut message_data = MessageData::default();

    // let commands = match &mut instruction_index {
    //     Some((index, _loop_index)) if !index.is_empty() => {
    //         let i = index.drain(1..).as_slice()[0];
    //         &actions.commands[i..]
    //     },
    //     _ => &actions.commands,
    // };

    for (action, instruction_info) in actions.commands.iter() {
        let instruction_total = instruction_info.index + instruction_info.total;

        if let Some((instruction_index, _)) = instruction_index {
            if instruction_index >= instruction_total {
                continue;
            }
        }

        if message_data.exit_condition.is_some() {
            return Ok(message_data);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Return(var)) => {
                let lit = expr_to_literal(var, false, None, data, &mut message_data, &None)?;
                message_data.exit_condition = Some(ExitCondition::Return(lit));

                return Ok(message_data);
            }
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                message_data.exit_condition = Some(ExitCondition::Break);

                return Ok(message_data);
            }
            Expr::ObjectExpr(ObjectType::Continue(..)) => {
                message_data.exit_condition = Some(ExitCondition::Continue);

                return Ok(message_data);
            }
            Expr::ObjectExpr(ObjectType::Hold(..)) => {
                let mut step_hash = Md5::new();

                // need to set all the intervals to 0 in order to avoid new lines conflicts
                let clean_step = clean_step_intervals(actions.clone());
                step_hash.update(bincode::serialize(&clean_step).unwrap());

                message_data.exit_condition = Some(ExitCondition::Hold);

                let step_hash = format!("{:x}", step_hash.finalize());
                let index = instruction_info.index;
                let map = data.step_vars.to_owned();

                let hold = Hold::new(index, vec!(0), step_vars_to_json(map), step_hash);

                message_data.hold = Some(hold.to_owned());

                MSG::send(&sender, MSG::Hold(hold));

                return Ok(message_data);
            }
            Expr::ObjectExpr(fun) => {
                message_data = match_actions(fun, message_data, data, instruction_index.clone(), &sender)?
            }
            Expr::IfExpr(ref if_statement) => {
                message_data = solve_if_statement(
                    if_statement,
                    message_data,
                    data,
                    instruction_index.clone(),
                    instruction_info,
                    &sender,
                )?;
            }
            Expr::ForEachExpr(ident, index, expr, block, range) => {
                message_data = for_loop(
                    ident,
                    index,
                    expr,
                    block,
                    range,
                    message_data,
                    data,
                    instruction_index.clone(),
                    &sender,
                )?
            }
            e => {
                return Err(gen_error_info(
                    Position::new(interval_from_expr(e)),
                    ERROR_START_INSTRUCTIONS.to_owned(),
                ));
            }
        };
    }

    Ok(message_data)
}
