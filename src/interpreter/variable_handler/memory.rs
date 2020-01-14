use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::get_path,
    data::Data,
    json_to_rust::Context,
    variable_handler::{interval::interval_from_expr, object::get_value_in_object}, //get_literal,
};
use crate::parser::{
    ast::{Expr, Identifier},
    literal::Literal,
    tokens::{MEMORY, PAST},
};

fn extract_indent(expr: &Expr) -> Result<Identifier, ErrorInfo> {
    match expr {
        Expr::IdentExpr(ident) => Ok(ident.to_owned()),
        _ => Err(ErrorInfo {
            message: "_metadata expect identifier | ex: _metadata.firstname".to_owned(),
            interval: interval_from_expr(expr),
        }),
    }
}

pub fn search_in_metadata(path: &[Expr], data: &mut Data) -> Result<Literal, ErrorInfo> {
    let name = extract_indent(&path[0])?;
    let lit = match data.memory.metadata.get(&name.ident) {
        Some(lit) => lit.to_owned(),
        None => {
            return Err(ErrorInfo {
                message: format!("no variable named < {} > in metadata", name.ident),
                interval: name.interval.to_owned(),
            })
        }
    };

    if path.len() >= 2 {
        let path = get_path(&path[1..], data)?;
        Ok(get_value_in_object(&lit, &path, &name.interval)?)
    } else {
        Ok(lit.to_owned())
    }
}

pub fn search_in_memory_type<'a>(name: &Identifier, data: &Data) -> Result<String, ErrorInfo> {
    match (
        data.memory.current.get(&name.ident),
        data.memory.past.get(&name.ident),
        data.step_vars.get(&name.ident),
    ) {
        (_, _, Some(_)) => Ok("use".to_owned()),
        (_, Some(_), _) | (Some(_), _, _) => Ok("remember".to_owned()),
        (None, None, None) => Err(ErrorInfo {
            message: format!("no variable named < {} > in memory", name.ident),
            interval: name.interval.to_owned(),
        }),
    }
}

pub fn search_var_memory<'a>(
    name: Identifier,
    data: &'a mut Data,
) -> Result<&'a mut Literal, ErrorInfo> {
    match (
        data.memory.current.get_mut(&name.ident),
        data.memory.past.get_mut(&name.ident),
    ) {
        (Some(lit), _) => {
            lit.set_interval(name.interval);
            Ok(lit)
        }
        (_, Some(lit)) => {
            lit.set_interval(name.interval);
            Ok(lit)
        }
        (None, None) => Err(ErrorInfo {
            message: format!("no variable named < {} > in memory", name.ident),
            interval: name.interval.to_owned(),
        }),
    }
}

pub fn memory_get<'a>(memory: &'a Context, name: &Expr, expr: &Expr) -> Option<&'a Literal> {
    match (name, expr) {
        (
            Expr::IdentExpr(Identifier { ident, .. }),
            Expr::LitExpr(Literal::StringLiteral { value, .. }),
        ) if ident == PAST => memory.past.get(value),
        (
            Expr::IdentExpr(Identifier { ident, .. }),
            Expr::LitExpr(Literal::StringLiteral { value, .. }),
        ) if ident == MEMORY => memory.current.get(value),
        (_, Expr::LitExpr(Literal::StringLiteral { value, .. })) => memory.metadata.get(value),
        _ => None,
    }
}
