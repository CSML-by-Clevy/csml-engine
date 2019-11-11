pub mod expr_to_literal;
pub mod gen_literal;
pub mod interval;
pub mod match_literals;
pub mod memory;
pub mod object;
pub mod operations;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::{if_statment::evaluate_condition, match_functions},
    data::Data,
    variable_handler::{
        expr_to_literal::expr_to_literal,
        gen_literal::{gen_literal_form_builder, gen_literal_form_event},
        interval::interval_from_expr,
        memory::search_var_memory,
    },
};
use crate::parser::{
    ast::{Expr, Identifier, Interval},
    literal::Literal,
    tokens::{EVENT, RETRIES},
};

//TODO: return Warning or Error Component
pub fn get_literal(
    literal: &Literal,
    opt: &Option<Box<Expr>>,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match (literal, opt) {
        (
            Literal::ArrayLiteral {
                ref items,
                interval,
            },
            Some(expr),
        ) => {
            let index = expr_to_literal(expr, data)?;

            if let Literal::IntLiteral { value, .. } = index {
                match items.get(value as usize) {
                    Some(lit) => Ok(lit.to_owned()),
                    None => Err(ErrorInfo {
                        message: format!("Error Array don't have {} index", value),
                        interval: interval.to_owned(),
                    }),
                }
            } else {
                Err(ErrorInfo {
                    message: "Error index must resolve to int type".to_string(),
                    interval: index.get_interval(),
                })
            }
        }
        (_, Some(_)) => Err(ErrorInfo {
            message: "Error value is not of type Array".to_owned(),
            interval: literal.get_interval(),
        }),
        (literal, None) => Ok(literal.to_owned()),
    }
}

fn get_var_from_stepvar(name: &str, data: &mut Data) -> Option<Literal> {
    match data.step_vars.get(name) {
        Some(var) => Some(var.to_owned()),
        None => None,
    }
}

pub fn get_var(name: Identifier, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match &name.ident {
        var if var == EVENT => gen_literal_form_event(data.event, name.interval),
        var if var == RETRIES => Ok(Literal::int(data.memory.retries, name.interval.to_owned())), // tmp
        _ => {
            let var = get_var_from_stepvar(&name.ident, data);
            match var {
                Some(val) => get_literal(&val, &name.index, data),
                None => search_var_memory(data.memory, name, data),
            }
        }
    }
}

pub fn get_string_from_complexstring(exprs: &[Expr], data: &mut Data) -> Literal {
    let mut new_string = String::new();
    let mut interval: Option<Interval> = None;

    //TODO: log error with span
    for elem in exprs.iter() {
        match match_functions(elem, data) {
            Ok(var) => {
                if interval.is_none() {
                    interval = Some(var.get_interval())
                }
                new_string.push_str(&var.to_string())
            }
            Err(err) => {
                if interval.is_none() {
                    interval = Some(err.interval)
                }
                new_string.push_str(&Literal::null(interval.clone().unwrap()).to_string())
            }
        }
    }
    //TODO: check for error empty list
    Literal::string(new_string, interval.unwrap())
}

pub fn get_var_from_ident(expr: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::LitExpr(literal) => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        Expr::BuilderExpr(..) | Expr::ComplexLiteral(..) => gen_literal_form_builder(expr, data),
        Expr::InfixExpr(infix, exp_1, exp_2) => evaluate_condition(infix, exp_1, exp_2, data),
        e => Err(ErrorInfo {
            message: "unknown variable in Ident err get_var_from_ident".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}
