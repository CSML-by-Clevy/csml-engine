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
        memory::{search_in_memory_type, search_var_memory},
    },
};
use crate::parser::{
    ast::{Expr, Identifier, Interval},
    literal::Literal,
    tokens::{EVENT, RETRIES},
};

pub fn get_index(index: Option<Box<Expr>>, data: &mut Data) -> Result<Option<Literal>, ErrorInfo> {
    match index {
        Some(expr) => Ok(Some(expr_to_literal(&expr, data)?)),
        None => Ok(None),
    }
}

//TODO: return Warning or Error Component
pub fn get_literal<'a>(
    literal: &'a mut Literal,
    index: Option<Literal>,
) -> Result<&'a mut Literal, ErrorInfo> {
    let interval = literal.get_interval();

    match (literal, index) {
        (
            Literal::ArrayLiteral {
                ref mut items,
                interval,
            },
            Some(Literal::IntLiteral { value, .. }),
        ) => match items.get_mut(value as usize) {
            Some(lit) => Ok(lit),
            None => Err(ErrorInfo {
                message: format!("Array don't have {} index", value),
                interval: interval.to_owned(),
            }),
        },
        (literal, None) => Ok(literal),
        (_, Some(_)) => Err(ErrorInfo {
            message: "value is not of type Array".to_owned(),
            interval,
        }),
    }
}

fn get_var_from_stepvar<'a>(
    name: &Identifier,
    data: &'a mut Data,
) -> Result<&'a mut Literal, ErrorInfo> {
    match data.step_vars.get_mut(&name.ident) {
        Some(var) => Ok(var),
        None => Err(ErrorInfo {
            message: format!("no variable named < {} > in memory", name.ident),
            interval: name.interval.to_owned(),
        }),
    }
}

pub fn get_var(name: Identifier, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match &name.ident {
        var if var == EVENT => gen_literal_form_event(data.event, name.interval),
        var if var == RETRIES => Ok(Literal::int(data.memory.retries, name.interval.to_owned())), // tmp
        _ => {
            let interval = name.interval.to_owned();
            let index = get_index(name.index.clone(), data)?;
            let lit = match get_var_from_mem(name, data) {
                Ok((lit, ..)) => get_literal(lit, index)?.to_owned(),
                Err(_) => Literal::null(interval),
            };
            Ok(lit)
        }
    }
}

pub fn get_var_from_mem<'a>(
    name: Identifier,
    data: &'a mut Data,
) -> Result<(&'a mut Literal, String, String), ErrorInfo> {
    match search_in_memory_type(&name, data)? {
        var if var == "use" => {
            let lit = get_var_from_stepvar(&name, data)?;
            Ok((lit, name.ident, "use".to_owned()))
        }
        _ => {
            let lit = search_var_memory(name.clone(), data)?;
            Ok((lit, name.ident, "remember".to_owned()))
        }
    }
}

pub fn get_string_from_complexstring(exprs: &[Expr], interval: Interval ,data: &mut Data) -> Literal {
    let mut new_string = String::new();

    //TODO: log error with span
    for elem in exprs.iter() {
        match match_functions(elem, data) {
            Ok(var) => {
                new_string.push_str(&var.to_string())
            },
            Err(_) => {
                new_string.push_str(&Literal::null(interval.clone()).to_string())
            }
        }
    }
    Literal::string(new_string, interval)
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
