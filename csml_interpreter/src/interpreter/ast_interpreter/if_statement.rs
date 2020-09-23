use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::{
    ast::{Block, Expr, IfStatement, Infix, InstructionInfo},
    Data, Literal, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::{
    interpret_scope,
    variable_handler::{
        expr_to_literal, get_var, interval::interval_from_expr, operations::evaluate,
    },
};
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn valid_literal(result: Result<Literal, ErrorInfo>) -> bool {
    match result {
        Ok(literal) => literal.primitive.as_bool(),
        Err(_) => false,
    }
}

//TODO: add warning when comparing some objects
fn valid_condition(
    expr: &Expr,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> bool {
    match expr {
        Expr::LitExpr(literal) => valid_literal(Ok(literal.to_owned())),
        Expr::IdentExpr(ident) => valid_literal(get_var(
            ident.to_owned(),
            true,
            None,
            data,
            msg_data,
            sender,
        )),
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
    instruction_index: &Option<usize>,
    instruction_info: &InstructionInfo,
    sender: &Option<mpsc::Sender<MSG>>,
    then_branch: &Option<(Box<IfStatement>, InstructionInfo)>,
) -> Result<MessageData, ErrorInfo> {
    if valid_condition(cond, data, &mut msg_data, sender) {
        msg_data = msg_data + interpret_scope(consequence, data, instruction_index, sender)?;
        return Ok(msg_data);
    }
    if let Some((then, _)) = then_branch {
        solve_if_statement(
            then,
            msg_data,
            data,
            instruction_index,
            instruction_info,
            sender,
        )
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
    match (expr1, expr2) {
        (exp_1, ..) if Infix::Not == *infix => {
            let value = !valid_literal(expr_to_literal(exp_1, true, None, data, msg_data, sender));
            let interval = interval_from_expr(exp_1);
            Ok(PrimitiveBoolean::get_literal(value, interval))
        }
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp_1, exp_2)) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data, msg_data, sender),
            evaluate_condition(i2, exp_1, exp_2, data, msg_data, sender),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data, msg_data, sender),
            expr_to_literal(exp, true, None, data, msg_data, sender),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => evaluate(
            infix,
            expr_to_literal(exp, true, None, data, msg_data, sender),
            evaluate_condition(i1, ex1, ex2, data, msg_data, sender),
        ),
        (exp_1, exp_2) => evaluate(
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
    instruction_index: &Option<usize>,
    instruction_info: &InstructionInfo,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match statement {
        IfStatement::IfStmt {
            cond,
            consequence,
            then_branch,
        } => {
            match instruction_index {
                Some(index) if *index <= instruction_info.index => {
                    return evaluate_if_condition(
                        cond,
                        msg_data,
                        data,
                        consequence,
                        instruction_index,
                        instruction_info,
                        sender,
                        then_branch,
                    );
                }
                Some(index) => {
                    if let Some((then_branch, then_index)) = then_branch {
                        if *index < then_index.index {
                            msg_data = msg_data
                                + interpret_scope(consequence, data, instruction_index, sender)?;
                            return Ok(msg_data);
                        } else {
                            return solve_if_statement(
                                &then_branch,
                                msg_data,
                                data,
                                instruction_index,
                                instruction_info,
                                sender,
                            );
                        }
                    }

                    if *index != instruction_info.index {
                        msg_data = msg_data
                            + interpret_scope(consequence, data, instruction_index, sender)?;
                    }
                }
                None => {
                    return evaluate_if_condition(
                        cond,
                        msg_data,
                        data,
                        consequence,
                        instruction_index,
                        instruction_info,
                        sender,
                        then_branch,
                    );
                }
            }
            Ok(msg_data)
        }
        IfStatement::ElseStmt(consequence, ..) => {
            msg_data = msg_data + interpret_scope(consequence, data, instruction_index, sender)?;
            Ok(msg_data)
        }
    }
}
