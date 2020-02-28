use crate::data::primitive::string::PrimitiveString;
use crate::data::{
    ast::{Expr, Identifier, Interval, PathExpr},
    Data, Literal, MemoryType, MessageData, MSG,
};
use crate::error_format::ErrorInfo;
use crate::interpreter::{
    json_to_rust::json_to_literal,
    variable_handler::{exec_path_actions, get_var, interval::interval_from_expr, resolve_path},
};
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
            let mut lit = json_to_literal(&data.event.metadata, interval.to_owned())?;
            let (lit, _tmp_mem_update) = exec_path_actions(
                &mut lit,
                None,
                &Some(path),
                &MemoryType::Event(data.event.content_type.to_owned()),
            )?;
            Ok(lit)
        }
        None => Ok(PrimitiveString::get_literal(
            &data.event.content,
            interval.to_owned(),
        )),
    }
}
