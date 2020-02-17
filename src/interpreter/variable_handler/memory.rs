use crate::data::{
    ast::{Expr, Identifier},
    send_msg,
    tokens::{MEMORY, PAST},
    Context, Data, Literal, Memories, MemoryType, MessageData, MSG,
};
use crate::error_format::ErrorInfo;
use crate::data::primitive::PrimitiveType;
use std::sync::mpsc;

pub fn search_in_memory_type(name: &Identifier, data: &Data) -> Result<String, ErrorInfo> {
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
            lit.interval = name.interval;
            Ok(lit)
        }
        (_, Some(lit)) => {
            lit.interval = name.interval;
            Ok(lit)
        }
        (None, None) => Err(ErrorInfo {
            message: format!("no variable named < {} > in memory", name.ident),
            interval: name.interval.to_owned(),
        }),
    }
}

pub fn save_literal_in_mem(
    lit: Literal,
    name: String,
    mem_type: &MemoryType,
    update: bool,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) {
    match mem_type {
        MemoryType::Remember if update => {
            // add mesage to rememeber new value
            root.add_to_memory(&name, lit.clone());
            // add value in current mem
            send_msg(
                sender,
                MSG::Memorie(Memories::new(name.clone(), lit.clone())),
            );
            data.memory.current.insert(name, lit);
        }
        MemoryType::Use if update => {
            data.step_vars.insert(name, lit);
        }
        _ => {
            // TODO: Warning msg element is unmutable ?
            // unimplemented!()
        }
    }
}

pub fn memory_get<'a>(memory: &'a Context, name: &Expr, expr: &Expr) -> Option<&'a Literal> {
    match (name, expr) {
        (Expr::IdentExpr(Identifier { ident, .. }), Expr::LitExpr(literal))
            if ident == PAST && literal.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            let value = Literal::get_value::<String>(&literal.primitive).unwrap();
            memory.past.get(value)
        }
        (Expr::IdentExpr(Identifier { ident, .. }), Expr::LitExpr(literal))
            if ident == MEMORY
                && literal.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            let value = Literal::get_value::<String>(&literal.primitive).unwrap();
            memory.current.get(value)
        }
        (_, Expr::LitExpr(literal))
            if literal.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            let value = Literal::get_value::<String>(&literal.primitive).unwrap();
            memory.metadata.get(value)
        }
        _ => None,
    }
}
