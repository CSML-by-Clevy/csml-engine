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
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> bool {
    match expr {
        Expr::LitExpr(literal) => valid_literal(Ok(literal.to_owned())),
        Expr::IdentExpr(ident) => {
            valid_literal(get_var(ident.to_owned(), None, data, root, sender))
        }
        Expr::InfixExpr(inf, exp_1, exp_2) => {
            valid_literal(evaluate_condition(inf, exp_1, exp_2, data, root, sender))
        }
        value => valid_literal(expr_to_literal(value, None, data, root, sender)),
    }
}

fn evaluate_if_condition(
    cond: &Expr,
    mut root: MessageData,
    data: &mut Data,
    consequence: &Block,
    instruction_index: Option<usize>,
    instruction_info: &InstructionInfo,
    sender: &Option<mpsc::Sender<MSG>>,
    then_branch: &Option<(Box<IfStatement>, InstructionInfo)>,
) -> Result<MessageData, ErrorInfo> {
    if valid_condition(cond, data, &mut root, sender) {
        root = root + interpret_scope(consequence, data, instruction_index, sender)?;
        return Ok(root);
    }
    if let Some((then, _)) = then_branch {
        solve_if_statement(
            then,
            root,
            data,
            instruction_index,
            instruction_info,
            sender,
        )
    } else {
        Ok(root)
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
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match (expr1, expr2) {
        (exp_1, ..) if Infix::Not == *infix => {
            let value = !valid_literal(expr_to_literal(exp_1, None, data, root, sender));
            let interval = interval_from_expr(exp_1);
            Ok(PrimitiveBoolean::get_literal(value, interval))
        }
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp_1, exp_2)) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data, root, sender),
            evaluate_condition(i2, exp_1, exp_2, data, root, sender),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data, root, sender),
            expr_to_literal(exp, None, data, root, sender),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => evaluate(
            infix,
            expr_to_literal(exp, None, data, root, sender),
            evaluate_condition(i1, ex1, ex2, data, root, sender),
        ),
        (exp_1, exp_2) => evaluate(
            infix,
            expr_to_literal(exp_1, None, data, root, sender),
            expr_to_literal(exp_2, None, data, root, sender),
        ),
    }
}

pub fn solve_if_statement(
    statement: &IfStatement,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: Option<usize>,
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
                Some(index) if index <= instruction_info.index => {
                    return evaluate_if_condition(
                        cond,
                        root,
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
                        if index < then_index.index {
                            root = root
                                + interpret_scope(consequence, data, instruction_index, sender)?;
                            return Ok(root);
                        } else {
                            return solve_if_statement(
                                &then_branch,
                                root,
                                data,
                                instruction_index,
                                instruction_info,
                                sender,
                            );
                        }
                    }

                    if index != instruction_info.index {
                        root =
                            root + interpret_scope(consequence, data, instruction_index, sender)?;
                    }
                }
                None => {
                    return evaluate_if_condition(
                        cond,
                        root,
                        data,
                        consequence,
                        instruction_index,
                        instruction_info,
                        sender,
                        then_branch,
                    );
                }
            }
            Ok(root)
        }
        IfStatement::ElseStmt(consequence, ..) => {
            root = root + interpret_scope(consequence, data, instruction_index, sender)?;
            Ok(root)
        }
    }
}
