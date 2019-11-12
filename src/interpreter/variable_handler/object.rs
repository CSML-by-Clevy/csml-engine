use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    variable_handler::{get_literal, interval::interval_from_expr},
};
use crate::parser::{
    ast::{Expr, Identifier, Interval},
    literal::Literal,
};
use std::collections::HashMap;

fn get_properties_form_object<'a>(
    literal: &'a Literal,
    interval: &Interval,
) -> Result<&'a HashMap<String, Literal>, Literal> {
    match literal {
        Literal::ObjectLiteral { properties, .. } => Ok(properties),
        Literal::FunctionLiteral { value, .. } => {
            let lit: &Literal = value;
            match lit {
                Literal::ObjectLiteral { properties, .. } => Ok(properties),
                _ => Err(Literal::null(interval.to_owned())),
            }
        }
        _ => Err(Literal::null(interval.to_owned())),
    }
}

fn search_ident_in_obj(
    map: &HashMap<String, Literal>,
    ident: &Identifier,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match (map.get(&ident.ident), &ident.index) {
        (Some(val), opt) => get_literal(val, opt, data),
        (None, _) => {
            //TODO: replace with Error FunctionLiteral
            Ok(Literal::null(ident.interval.clone()))
        }
    }
}

pub fn decompose_object(
    literal: &Literal,
    expr: &Expr,
    interval: &Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {

    let map = match get_properties_form_object(literal, interval) {
        Ok(val) => val,
        //TODO: add Warning or change Err to ErrorInfo
        // literal is not a object
        Err(err) => return Ok(err)
    };

    match expr {
        Expr::BuilderExpr(elem, expr) => {
            let elem: &Expr = elem;
            if let Expr::IdentExpr(ident, ..) = elem {
                let literal = search_ident_in_obj(map, ident, data)?;
                decompose_object(&literal, expr, interval, data)
            } else {
                Err(ErrorInfo {
                    message: format!("Bad Expression Type: 'in expression.value' expression need to be of type identifier"),
                    interval: interval.to_owned(),
                })
            }
        }
        Expr::IdentExpr(ident) => search_ident_in_obj(map, &ident, data),
        e => Err(ErrorInfo{
            message: format!("Bad Expression Type: 'in value.expression' expression need to be of type identifier"),
            interval: interval_from_expr(e),
        }),
    }
}
