use crate::data::error_info::ErrorInfo;
use crate::data::{ast::Identifier, Data, Literal, Memories, MemoryType, MessageData, MSG};
use crate::error_format::*;
use std::sync::mpsc;

pub fn search_in_memory_type(name: &Identifier, data: &Data) -> Result<String, ErrorInfo> {
    match (
        data.context.current.get(&name.ident),
        data.step_vars.get(&name.ident),
    ) {
        (_, Some(_)) => Ok("use".to_owned()),
        (Some(_), _) => Ok("remember".to_owned()),
        (None, None) => Err(gen_error_info(
            name.interval.to_owned(),
            format!("< {} > {}", name.ident, ERROR_FIND_MEMORY),
        )),
    }
}

pub fn search_var_memory(name: Identifier, data: &mut Data) -> Result<&mut Literal, ErrorInfo> {
    match data.context.current.get_mut(&name.ident) {
        Some(lit) => {
            lit.interval = name.interval;
            Ok(lit)
        }
        None => Err(gen_error_info(
            name.interval.to_owned(),
            format!("< {} > {}", name.ident, ERROR_FIND_MEMORY),
        )),
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
            MSG::send(
                sender,
                MSG::Memory(Memories::new(name.clone(), lit.clone())),
            );
            data.context.current.insert(name, lit);
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

// pub fn memory_get<'a>(memory: &'a Context, name: &Expr, expr: &Expr) -> Option<&'a Literal> {
//     match (name, expr) {
//         (Expr::IdentExpr(Identifier { ident, .. }), Expr::LitExpr(literal))
//             if ident == MEMORY
//                 && literal.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             let value = Literal::get_value::<String>(&literal.primitive)?;
//             memory.current.get(value)
//         }
//         (_, Expr::LitExpr(literal))
//             if literal.primitive.get_type() == PrimitiveType::PrimitiveString =>
//         {
//             let value = Literal::get_value::<String>(&literal.primitive)?;
//             memory.metadata.get(value)
//         }
//         _ => None,
//     }
// }
