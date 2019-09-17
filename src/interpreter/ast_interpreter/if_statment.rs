use crate::parser::ast::{Expr, IfStatement, Infix, Literal};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    message::*,
    variable_handler::{
        get_var,
        operations::evaluate,
        get_var_from_ident,
        interval::interval_from_expr,
        gen_literal::gen_literal_form_expr,
    },
    ast_interpreter::{
        check_if_ident,
        interpret_scope,
    },
};

//TODO: add warning when comparing some objects
fn valid_condition(expr: &Expr, data: &mut Data) -> bool {
    match expr {
        Expr::InfixExpr(inf, exp1, exp2) => match evaluate_condition(inf, exp1, exp2, data) {
            Ok(Literal::BoolLiteral{value: false, ..}) => false,
            Ok(_) => true,
            Err(_e) => false,
        },
        Expr::LitExpr(Literal::BoolLiteral{value, ..}) => *value,
        Expr::LitExpr(Literal::Null{..}) => false,
        Expr::LitExpr(..) => true,
        Expr::BuilderExpr(..) => get_var_from_ident(expr, data).is_ok(), // error
        Expr::IdentExpr(ident, ..) => get_var(ident.to_owned(), data).is_ok(), // error
        _ => false, // return error
    }
}

pub fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match (expr1, expr2) {
        (exp1, ..) if Infix::Not == *infix && check_if_ident(exp1) => {
            match get_var_from_ident(exp1, data) {
                Ok(Literal::BoolLiteral{value: false, interval}) => Ok(Literal::boolean(true, interval)),
                Ok(Literal::IntLiteral{value: 0, interval}) => Ok(Literal::boolean(true, interval)),
                Ok(literal) => Ok(Literal::boolean(false, literal.get_interval())),
                Err(err) => Ok(Literal::boolean(true, err.interval)),
            }
        }
        (exp1, exp2) if check_if_ident(exp1) && check_if_ident(exp2) => {
            evaluate(infix, get_var_from_ident(exp1, data), get_var_from_ident(exp2, data))
        },
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2)) => evaluate(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            evaluate_condition(i2, exp1, exp2, data),
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
        (e1, _e2) => Err(
            ErrorInfo{
                message: "error in evaluate_condition function".to_owned(),
                interval: interval_from_expr(e1)
            }
        )
    }
}

pub fn solve_if_statments(
    statment: &IfStatement,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match statment {
        IfStatement::IfStmt {
            cond,
            consequence,
            then_branch,
        } => {
            if valid_condition(cond, data) {
                root = root + interpret_scope(consequence, data)?;
                return Ok(root);
            }
            if let Some(then) = then_branch {
                return solve_if_statments(then, root, data);
            }
            Ok(root)
        }
        IfStatement::ElseStmt(consequence, ..) => {
            root = root + interpret_scope(consequence, data)?;
            Ok(root)
        }
    }
}