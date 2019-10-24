use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    json_to_rust::{Context, MemoryType},
    variable_handler::{get_literal, interval::interval_from_expr},
};
use crate::parser::{
    ast::{Expr, Identifier, Interval, ObjectType},
    literal::Literal,
    tokens::{FIRST, GET_VALUE, MEMORY, PAST},
};

pub fn memorytype_to_literal(
    memtype: Option<&MemoryType>,
    interval: Interval,
    index: &Option<Box<Expr>>,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match memtype {
        Some(elem) => get_literal(&elem.value, index, data),
        None => Err(ErrorInfo {
            message: "Error in memorytype_to_literal".to_owned(),
            interval,
        }),
    }
}

pub fn search_var_memory(
    memory: &Context,
    name: Identifier,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match &name.ident {
        var if memory.metadata.contains_key(var) => memorytype_to_literal(
            memory.metadata.get(var),
            name.interval.clone(),
            &name.index,
            data,
        ),
        var if memory.current.contains_key(var) => memorytype_to_literal(
            memory.current.get(var),
            name.interval.clone(),
            &name.index,
            data,
        ),
        var if memory.past.contains_key(var) => memorytype_to_literal(
            memory.past.get(var),
            name.interval.clone(),
            &name.index,
            data,
        ),
        _ => Err(ErrorInfo {
            message: "unown variable in search_var_memory".to_owned(),
            interval: name.interval,
        }),
    }
}

pub fn memory_get<'a>(memory: &'a Context, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
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

pub fn memory_first<'a>(memory: &'a Context, name: &Expr, expr: &Expr) -> Option<&'a MemoryType> {
    match (name, expr) {
        (
            Expr::IdentExpr(Identifier { ident, .. }),
            Expr::LitExpr(Literal::StringLiteral { value, .. }),
        ) if ident == PAST => memory.past.get_vec(value).unwrap().last(),
        (
            Expr::IdentExpr(Identifier { ident, .. }),
            Expr::LitExpr(Literal::StringLiteral { value, .. }),
        ) if ident == MEMORY => memory.current.get_vec(value).unwrap().last(),
        (_, Expr::LitExpr(Literal::StringLiteral { value, .. })) => {
            memory.metadata.get_vec(value).unwrap().last()
        }
        _ => None,
    }
}

pub fn get_memory_action(
    memory: &Context,
    name: &Expr,
    expr: &Expr,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::Normal(
            Identifier {
                ident,
                interval,
                index,
            },
            expr,
        )) if ident == GET_VALUE => memorytype_to_literal(
            memory_get(memory, name, expr),
            interval.clone(),
            index,
            data,
        ),
        Expr::ObjectExpr(ObjectType::Normal(
            Identifier {
                ident,
                interval,
                index,
            },
            expr,
        )) if ident == FIRST => memorytype_to_literal(
            memory_first(memory, name, expr),
            interval.clone(),
            index,
            data,
        ),
        e => Err(ErrorInfo {
            message: "Error in memory action".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}
