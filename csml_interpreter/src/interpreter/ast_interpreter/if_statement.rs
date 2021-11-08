use crate::data::{
    ast::{Block, Expr, IfStatement, Infix, InstructionInfo},
    Data, Literal, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::{
    interpret_scope,
    variable_handler::{
        expr_to_literal, get_var,
        operations::{evaluate_infix, evaluate_postfix, valid_literal},
    },
};
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

//TODO: add warning when comparing some objects
pub fn valid_condition(
    expr: &Expr,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> bool {
    match expr {
        Expr::LitExpr { literal, .. } => valid_literal(Ok(literal.to_owned())),
        Expr::IdentExpr(ident) => valid_literal(get_var(
            ident.to_owned(),
            true,
            None,
            data,
            msg_data,
            sender,
        )),
        Expr::PostfixExpr(post, exp) => valid_literal(evaluate_postfix(
            post, exp, data, msg_data, sender,
            ))
        ,
        Expr::InfixExpr(inf, exp_1, exp_2) => valid_literal(evaluate_condition(
            inf, exp_1, exp_2, data, msg_data, sender,
        )),
        value => valid_literal(expr_to_literal(value, true, None, data, msg_data, sender)),
    }
}

fn evaluate_if_condition(
    cond: &Expr,
    mut msg_data: MessageData,
    data: &mut Data,
    consequence: &Block,
    instruction_info: &InstructionInfo,
    sender: &Option<mpsc::Sender<MSG>>,
    then_branch: &Option<Box<IfStatement>>,
) -> Result<MessageData, ErrorInfo> {
    if valid_condition(cond, data, &mut msg_data, sender) {
        msg_data = msg_data + interpret_scope(consequence, data, sender)?;
        return Ok(msg_data);
    }
    if let Some(then) = then_branch {
        solve_if_statement(then, msg_data, data, instruction_info, sender)
    } else {
        Ok(msg_data)
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let flow_name = &data.context.flow.clone();

    match (expr1, expr2) {
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp_1, exp_2)) => evaluate_infix(
            &flow_name,
            infix,
            evaluate_condition(i1, ex1, ex2, data, msg_data, sender),
            evaluate_condition(i2, exp_1, exp_2, data, msg_data, sender),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => evaluate_infix(
            &flow_name,
            infix,
            evaluate_condition(i1, ex1, ex2, data, msg_data, sender),
            expr_to_literal(exp, true, None, data, msg_data, sender),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => evaluate_infix(
            &flow_name,
            infix,
            expr_to_literal(exp, true, None, data, msg_data, sender),
            evaluate_condition(i1, ex1, ex2, data, msg_data, sender),
        ),
        (exp_1, exp_2) => evaluate_infix(
            &flow_name,
            infix,
            expr_to_literal(exp_1, true, None, data, msg_data, sender),
            expr_to_literal(exp_2, true, None, data, msg_data, sender),
        ),
    }
}

pub fn solve_if_statement(
    statement: &IfStatement,
    mut msg_data: MessageData,
    data: &mut Data,
    instruction_info: &InstructionInfo,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match statement {
        IfStatement::IfStmt {
            cond,
            consequence: scope,
            then_branch,
            last_action_index,
        } => {
            match &data.context.hold {
                Some(hold) => {
                    if hold.index.command_index <= *last_action_index {
                        msg_data = msg_data + interpret_scope(scope, data, sender)?;
                    } else if let Some(then_branch) = then_branch {
                        return solve_if_statement(
                            &then_branch,
                            msg_data,
                            data,
                            instruction_info,
                            sender,
                        );
                    }
                }
                None => {
                    return evaluate_if_condition(
                        cond,
                        msg_data,
                        data,
                        scope,
                        instruction_info,
                        sender,
                        then_branch,
                    );
                }
            }
            Ok(msg_data)
        }
        IfStatement::ElseStmt(consequence, ..) => {
            msg_data = msg_data + interpret_scope(consequence, data, sender)?;
            Ok(msg_data)
        }
    }
}
