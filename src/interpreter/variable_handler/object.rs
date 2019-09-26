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

fn get_values<'a>(
    literal: &'a Literal,
    expr: &Expr,
    interval: &Interval,
) -> Result<&'a HashMap<String, Literal>, ErrorInfo> {
    match literal {
        Literal::ObjectLiteral { properties, .. } => Ok(properties),
        Literal::FunctionLiteral { value, .. } => {
            let lit: &Literal = value;
            match lit {
                Literal::ObjectLiteral { properties, .. } => Ok(properties),
                _ => Err(ErrorInfo {
                    message: "Error ... bad type".to_owned(),
                    interval: interval_from_expr(expr),
                }),
            }
        }
        _ => Err(ErrorInfo {
            message: "Error: Bad Expression in object builder ".to_owned(),
            interval: interval.to_owned(),
        }),
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
            //TODO: replace with Error component
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
    let map = get_values(literal, expr, interval)?;

    match expr {
        Expr::BuilderExpr(elem, expr) => {
            let elem: &Expr = elem;
            if let Expr::IdentExpr(ident, ..) = elem {
                let literal = search_ident_in_obj(map, ident, data)?;
                decompose_object(&literal, expr, interval, data)
            } else {
                Err(ErrorInfo {
                    message: "Error in Object decomposer".to_owned(),
                    interval: interval.to_owned(),
                })
            }
        }
        Expr::IdentExpr(ident) => search_ident_in_obj(map, &ident, data),
        e => Err(ErrorInfo {
            message: "Error: Bad Expression in object decomposer".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}
