use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::{ast::Identifier, Data, Literal, Memory, MemoryType, MessageData, MSG};
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
            Position::new(name.interval),
            format!("< {} > {}", name.ident, ERROR_FIND_MEMORY),
        )),
    }
}

pub fn search_var_memory<'a>(
    name: Identifier,
    data: &'a mut Data,
) -> Result<&'a mut Literal, ErrorInfo> {
    match data.context.current.get_mut(&name.ident) {
        Some(lit) => {
            lit.interval = name.interval;
            Ok(lit)
        }
        None => Err(gen_error_info(
            Position::new(name.interval),
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
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) {
    match mem_type {
        MemoryType::Remember if update => {
            // save new value in current memory
            msg_data.add_to_memory(&name, lit.clone());
            // send new value to manager in order to be save in db
            MSG::send(sender, MSG::Memory(Memory::new(name.clone(), lit.clone())));
            data.context.current.insert(name, lit);
        }
        MemoryType::Use if update => {
            data.step_vars.insert(name, lit);
        }
        _ => {
            // TODO: Warning msg element is immutable ?
        }
    }
}
