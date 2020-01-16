use crate::error_format::data::ErrorInfo;
use std::sync::mpsc;
use crate::interpreter::{
    ast_interpreter::{check_if_ident, interpret_scope, match_functions},
    data::Data,
    message::MessageData,
    message::MSG,
    variable_handler::{
        gen_literal::gen_literal_form_expr, get_var, get_var_from_ident,
        interval::interval_from_expr, operations::evaluate,
    },
};
use crate::parser::{
    ast::{
        Expr,
        IfStatement,
        Infix,
        InstructionInfo,
        Block,
    },
    literal::Literal,
};

fn valid_literal(res: Result<Literal, ErrorInfo>) -> bool {
    match res {
        Ok(Literal::BoolLiteral { value, .. }) => value,
        Ok(Literal::IntLiteral { value, .. }) => value.is_positive(),
        Ok(Literal::FloatLiteral { value, .. }) => value.is_normal(),
        Ok(Literal::Null { .. }) => false,
        Ok(_) => true,
        Err(_) => false,
    }
}

//TODO: add warning when comparing some objects
fn valid_condition(expr: &Expr, data: &mut Data) -> bool {
    match expr {
        Expr::LitExpr(Literal::BoolLiteral { value, .. }) => *value,
        Expr::LitExpr(Literal::Null { .. }) => false,
        Expr::IdentExpr(ident) => valid_literal(get_var(ident.to_owned(), data)),
        Expr::InfixExpr(inf, exp_1, exp_2) => {
            valid_literal(evaluate_condition(inf, exp_1, exp_2, data))
        }
        value => valid_literal(match_functions(value, data)),
    }
}

pub fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match (expr1, expr2) {
        (exp_1, ..) if Infix::Not == *infix && check_if_ident(exp_1) => Ok(Literal::BoolLiteral {
            value: !valid_literal(get_var_from_ident(exp_1, data)),
            interval: interval_from_expr(exp_1),
        }),
        (exp_1, exp_2) if check_if_ident(exp_1) && check_if_ident(exp_2) => evaluate(
            infix,
            match_functions(exp_1, data),
            match_functions(exp_2, data),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp_1, exp_2)) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            evaluate_condition(i2, exp_1, exp_2, data),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            gen_literal_form_expr(exp, data),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => evaluate(
            infix,
            gen_literal_form_expr(exp, data),
            evaluate_condition(i1, ex1, ex2, data),
        ),
        (e1, _e2) => Err(ErrorInfo {
            message: "error in evaluate_condition function".to_owned(),
            interval: interval_from_expr(e1),
        }),
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
    if valid_condition(cond, data) {
        root = root + interpret_scope(consequence, data, instruction_index, sender)?;
        return Ok(root);
    }
    if let Some((then, _)) = then_branch {
        return solve_if_statments(then, root, data, instruction_index, instruction_info, sender);
    }
    return Ok(root);
}

pub fn solve_if_statments(
    statment: &IfStatement,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: Option<usize>,
    instruction_info: &InstructionInfo,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match statment {
        IfStatement::IfStmt {
            cond,
            consequence,
            then_branch,
        } => {
            match instruction_index {
                Some(index) if index <= instruction_info.index => {
                    return evaluate_if_condition(cond, root, data, consequence, instruction_index, instruction_info, sender, then_branch);
                }
                Some(index) => {
                    if let Some((then_branch, then_index)) = then_branch {
                        if index < then_index.index {
                            root = root + interpret_scope(consequence, data, instruction_index, sender) ?;
                            return Ok(root);
                        }
                        else {
                            return solve_if_statments(&then_branch, root, data, instruction_index, instruction_info, sender);
                        }
                    }

                    if index != instruction_info.index {
                        root = root + interpret_scope(consequence, data, instruction_index, sender) ?;
                    }
                }
                None => {
                    return evaluate_if_condition(cond, root, data, consequence, instruction_index, instruction_info, sender, then_branch);
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
