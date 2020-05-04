pub mod ast_interpreter;
pub mod builtins;
pub mod json_to_rust;
pub mod variable_handler;

pub use json_to_rust::{json_to_literal, memory_to_literal};

use crate::data::{ast::*, send_msg, Data, Hold, Literal, MessageData, MSG};
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
        json_map.insert(key.to_owned(), val.primitive.format_mem(content_type ,true));
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
    let mut root = MessageData::default();

    for (action, instruction_info) in actions.commands.iter() {
        let instruction_total = instruction_info.index + instruction_info.total;
        if let Some(instruction_index) = instruction_index {
            if instruction_index >= instruction_total {
                continue;
            }
        }

        if root.exit_condition.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(..)) => {
                root.exit_condition = Some(ExitCondition::Hold);
                root.hold = Some(Hold {
                    index: instruction_info.index,
                    step_vars: step_vars_to_json(data.step_vars.clone()),
                });
                send_msg(
                    &sender,
                    MSG::Hold {
                        instruction_index: instruction_info.index,
                        step_vars: step_vars_to_json(data.step_vars.clone()),
                    },
                );
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions(fun, root, data, instruction_index, &sender)?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statement(
                    ifstatement,
                    root,
                    data,
                    instruction_index,
                    instruction_info,
                    &sender,
                )?;
            }
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop(
                    ident,
                    i,
                    expr,
                    block,
                    range,
                    root,
                    data,
                    instruction_index,
                    &sender,
                )?
            }
            e => {
                // TODO: make Expr printable in order to be included in the error message
                return Err(gen_error_info(
                    interval_from_expr(e),
                    ERROR_START_INSTRUCTIONS.to_owned(),
                ));
            }
        };
    }

    Ok(root)
}
