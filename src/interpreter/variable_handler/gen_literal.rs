use crate::data::{
    ast::{Expr, Identifier, Interval, PathExpr},
    Data, Event, Literal, MemoryType, MessageData, MSG,
};
use crate::error_format::ErrorInfo;
use crate::interpreter::{
    json_to_rust::json_to_literal,
    variable_handler::{exec_path_actions, get_var, interval::interval_from_expr, resolve_path},
};
use crate::data::primitive::{null::PrimitiveNull, string::PrimitiveString};
use std::sync::mpsc;

pub fn search_str(name: &str, expr: &Expr) -> bool {
    match expr {
        Expr::IdentExpr(Identifier { ident, .. }) if ident == name => true,
        _ => false,
    }
}

pub fn gen_literal_form_expr(
    expr: &Expr,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::LitExpr(literal) => Ok(literal.clone()),
        Expr::IdentExpr(ident, ..) => get_var(ident.clone(), data, root, sender),
        e => Err(ErrorInfo {
            message: "Expression must be a literal or an identifier".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn gen_literal_form_event(
    interval: Interval,
    path: Option<Vec<(Interval, PathExpr)>>,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match path {
        Some(path) => {
            let path = resolve_path(path, data, root, sender)?;
            match data.event {
                Some(event) => {
                    let mut lit = json_to_literal(&event.metadata, interval.to_owned())?;
                    let (lit, _tmp_mem_update) = exec_path_actions(
                        &mut lit,
                        None,
                        &Some(path),
                        &MemoryType::Event(event.content_type.to_owned()),
                    )?;
                    Ok(lit)
                }
                //TODO: Add Warning for nonexisting key
                None => Ok(PrimitiveNull::get_literal("null", interval.to_owned())),
            }
        }
        None => match data.event {
            Some(Event { content, .. }) => Ok(PrimitiveString::get_literal(
                "string",
                content,
                interval.to_owned(),
            )),
            None => Ok(PrimitiveNull::get_literal("null", interval.to_owned())),
        },
    }
}
