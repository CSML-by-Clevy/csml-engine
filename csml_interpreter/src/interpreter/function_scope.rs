use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::{ast::*, primitive::PrimitiveNull, Data, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::{for_loop, match_actions, solve_if_statement},
    variable_handler::{expr_to_literal, interval::interval_from_expr},
};
use crate::parser::ExitCondition;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn interpret_function_scope(
    actions: &Block,
    data: &mut Data,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let mut message_data = MessageData::default();

    for (action, instruction_info) in actions.commands.iter() {
        match action {
            Expr::ObjectExpr(ObjectType::Return(var)) => {
                let lit = expr_to_literal(var, false, None, data, &mut message_data, sender)?;

                // return Ok(lit);
                message_data.exit_condition = Some(ExitCondition::Return(lit));
                return Ok(message_data);
            }
            Expr::ObjectExpr(fun) => {
                message_data = match_actions(fun, message_data, data, None, sender)?
            }
            Expr::IfExpr(ref if_statement) => {
                message_data = solve_if_statement(
                    if_statement,
                    message_data,
                    data,
                    &None,
                    instruction_info,
                    sender,
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
                    &None,
                    sender,
                )?
            }
            e => {
                return Err(gen_error_info(
                    Position::new(interval_from_expr(e)),
                    ERROR_START_INSTRUCTIONS.to_owned(),
                ));
            }
        };

        // dbg!(&message_data);
        if let Some(ExitCondition::Return(_)) = &message_data.exit_condition {
            return Ok(message_data);
        }
    }

    // Ok(PrimitiveNull::get_literal(interval))
    Ok(message_data)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn exec_fn_in_new_scope(
    expr: Expr,
    new_scope_data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::Scope {
            block_type: BlockType::Function,
            scope,
            range: RangeInterval { start, .. },
        } => {
            let fn_msg_data = interpret_function_scope(&scope, new_scope_data, sender)?;

            let mut return_value = PrimitiveNull::get_literal(start);
            if let Some(ExitCondition::Return(lit)) = fn_msg_data.exit_condition {
                return_value = lit;
            }

            msg_data.messages = [&msg_data.messages[..], &fn_msg_data.messages[..]].concat();

            Ok(return_value)
        }
        _ => panic!("error in parsing need to be expr scope"),
    }
}
