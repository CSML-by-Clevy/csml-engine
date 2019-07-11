use crate::interpreter::{ast_interpreter::evaluate_condition, data::Data, json_to_rust::*};
use crate::parser::{ast::*, tokens::*};
use std::str::FromStr;

pub fn gen_literal_form_event(event: &Option<Event>, interval: Interval) -> Result<SmartLiteral, String> {
    match event {
        Some(event) => {
            match event.payload {
                PayLoad {
                    content_type: ref t,
                    content: ref c,
                } if t == "text" => Ok(
                    SmartLiteral{
                        literal: Literal::StringLiteral(c.text.to_string()),
                        interval
                    }
                ),
                PayLoad {
                    content_type: ref t,
                    content: ref c,
                } if t == "float" => match c.text.to_string().parse::<f64>() {
                    Ok(float) => Ok(
                        SmartLiteral{
                            literal: Literal::FloatLiteral(float),
                            interval
                        }
                    ),
                    Err(..) => Err(format!("event value {} is not a float", c.text)),
                },
                PayLoad {
                    content_type: ref t,
                    content: ref c,
                } if t == "int" => match c.text.to_string().parse::<i64>() {
                    Ok(int) => Ok(
                        SmartLiteral{
                            literal: Literal::IntLiteral(int),
                            interval
                        }
                    ),
                    Err(..) => Err(format!("event value {} is not a int", c.text)),
                },
                _ => Err("event type is unown".to_owned()),
            }
        }
        None => Err("no event is received in gen_literal_form_event".to_owned()),
    }
}

pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(SmartIdent{ident, ..}, ..) if ident == name => true,
        _ => false,
    }
}

pub fn get_var(name: SmartIdent, data: &mut Data) -> Result<SmartLiteral, String> {
    match name.ident {
        var if var == EVENT => gen_literal_form_event(data.event, name.interval),
        _                   => match data.step_vars.get(&name) {
            Some(val) => Ok(val.to_owned()),
            None      => search_var_memory(data.memory, name),
        },
    }
}

pub fn get_string_from_complexstring(exprs: &[Expr], data: &mut Data) -> SmartLiteral {
    let mut new_string = String::new();
    let mut interval: Option<Interval> = None;

    //TODO: log error and cactch inrterval
    for elem in exprs.iter() {
        match get_var_from_ident(elem, data) {
            Ok(var) => {
                if interval.is_none() {
                    interval = Some(var.interval)
                }
                new_string.push_str(&var.literal.to_string())
            },
            Err(_) => new_string.push_str(" NULL "), 
        }
    }
    //TODO: check for error empty list
    SmartLiteral{
        literal: Literal::StringLiteral(new_string),
        interval: interval.unwrap()
    }
    
}

pub fn get_var_from_ident(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, String> {
    match expr {
        Expr::LitExpr(literal)              => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..)          => get_var(ident.clone(), data),
        Expr::BuilderExpr(..)               => gen_literal_form_builder(expr, data),
        Expr::ComplexLiteral(..)            => gen_literal_form_builder(expr, data),
        Expr::InfixExpr(infix, exp1, exp2)  => evaluate_condition(infix, exp1, exp2, data),
        _                                   => Err("unown variable in Ident err n#1".to_owned()),
    }
}

pub fn gen_literal_form_exp(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, String> {
    match expr {
        Expr::LitExpr(literal)      => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..)  => get_var(ident.clone(), data),
        _                           => Err("Expression must be a literal or an identifier".to_owned()),
    }
}

pub fn gen_literal_form_builder(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, String> {
    match expr {
        Expr::BuilderExpr(elem, exp) if search_str(PAST, elem) => {
            get_memory_action(data.memory, elem, exp)
        }
        Expr::BuilderExpr(elem, exp) if search_str(MEMORY, elem) => {
            get_memory_action(data.memory, elem, exp)
        }
        Expr::BuilderExpr(elem, exp) if search_str(METADATA, elem) => {
            get_memory_action(data.memory, elem, exp)
        }
        Expr::ComplexLiteral(vec) => Ok(get_string_from_complexstring(vec, data)),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data),
        _ => Err("Error in Exprecion builder".to_owned()),
    }
}

pub fn memorytype_to_literal(memtype: Option<&MemoryType>, interval: Interval) -> Result<SmartLiteral, String> {
    if let Some(elem) = memtype {
        match elem.r#type {
            Some(t) if t == "text" => Ok(
                SmartLiteral{
                    literal: Literal::StringLiteral(elem.value.to_string()),
                    interval
                }
            ),
            Some(t) if t == "bool" => Ok(
                SmartLiteral{
                    literal: Literal::BoolLiteral(FromStr::from_str(&elem.value).unwrap()),
                    interval
                }
            ),
            Some(t) if t == "int" => Ok(
                SmartLiteral{
                    literal: Literal::IntLiteral(FromStr::from_str(&elem.value).unwrap()),
                    interval
                }
            ),
            Some(t) if t == "float" => Ok(
                SmartLiteral{
                    literal: Literal::FloatLiteral(FromStr::from_str(&elem.value).unwrap()),
                    interval
                }
            ),
            Some(t) => Ok(
                SmartLiteral{
                    literal: Literal::StringLiteral(elem.value.to_string()),
                    interval
                }
            ),
            None => Ok(
                SmartLiteral{
                    literal: Literal::StringLiteral(elem.value.to_string()),
                    interval
                }
            )
        }
    } else {
        Err("Error in memory action".to_owned())
    }
}

// MEMORY ------------------------------------------------------------------

pub fn search_var_memory(memory: &Memory, name: SmartIdent) -> Result<SmartLiteral, String> {
    match name.ident {
        var if memory.metadata.contains_key(&var) => memorytype_to_literal(memory.metadata.get(&var), name.interval.clone()),
        var if memory.current.contains_key(&var) => memorytype_to_literal(memory.current.get(&var), name.interval.clone()),
        var if memory.past.contains_key(&var) => memorytype_to_literal(memory.past.get(&var), name.interval.clone()),
        _ => Err("unown variable in search_var_memory".to_owned()),
    }
}

pub fn memory_get<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (Expr::IdentExpr(SmartIdent{ident, ..}), Expr::LitExpr(SmartLiteral{literal: Literal::StringLiteral(lit), ..})) if ident == PAST => {
            memory.past.get(lit)
        }
        (Expr::IdentExpr(SmartIdent{ident, ..}), Expr::LitExpr(SmartLiteral{literal: Literal::StringLiteral(lit), ..})) if ident == MEMORY => {
            memory.current.get(lit)
        }
        (_, Expr::LitExpr(SmartLiteral{literal: Literal::StringLiteral(lit), ..})) => memory.metadata.get(lit),
        _ => None,
    }
}

pub fn memory_first<'a>(memory: &'a Memory, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (Expr::IdentExpr(SmartIdent{ident, ..}), Expr::LitExpr(SmartLiteral{literal: Literal::StringLiteral(lit), ..})) if ident == PAST => {
            memory.past.get_vec(lit).unwrap().last()
        }
        (Expr::IdentExpr(SmartIdent{ident, ..}), Expr::LitExpr(SmartLiteral{literal: Literal::StringLiteral(lit), ..})) if ident == MEMORY => {
            memory.current.get_vec(lit).unwrap().last()
        }
        (_, Expr::LitExpr(SmartLiteral{literal: Literal::StringLiteral(lit), ..})) => {
            memory.metadata.get_vec(lit).unwrap().last()
        }
        _ => None,
    }
}

pub fn get_memory_action(memory: &Memory, name: &Expr, expr: &Expr) -> Result<SmartLiteral, String> {
    match expr {
        Expr::FunctionExpr(ReservedFunction::Normal(SmartIdent{ident, interval}, exp)) if ident == GET_VALUE => {
            memorytype_to_literal(memory_get(memory, name, exp), interval.clone())
        }
        Expr::FunctionExpr(ReservedFunction::Normal(SmartIdent{ident, interval}, exp)) if ident == FIRST => {
            memorytype_to_literal(memory_first(memory, name, exp), interval.clone())
        }
        _ => Err("Error in memory action".to_owned()),
    }
}
