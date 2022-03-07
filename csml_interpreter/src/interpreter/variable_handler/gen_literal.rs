use crate::data::literal::ContentType;
use crate::data::position::Position;
use crate::data::{
    ast::PathLiteral,
    primitive::{PrimitiveNull, PrimitiveString},
    warnings::DisplayWarnings,
};
use crate::data::{
    ast::{Interval, PathState},
    Data, Literal, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::variable_handler::gen_generic_component::gen_generic_component;
use crate::interpreter::{
    json_to_rust::json_to_literal,
    variable_handler::{exec_path_actions, resolve_path},
};
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn gen_literal_from_event(
    interval: Interval,
    dis_warnings: &DisplayWarnings,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match path {
        Some(path) => {
            let path = resolve_path(path, dis_warnings, data, msg_data, sender)?;
            let mut lit = json_to_literal(&data.event.content, interval.to_owned(), &data.context.flow)?;

            println!("===> {:?}", data.event);

            lit.set_content_type("event");

            let content_type = match ContentType::get(&lit) {
                ContentType::Event(_) => ContentType::Event(data.event.content_type.to_owned()),
                _ => {
                    return Err(gen_error_info(
                        Position::new(interval, &data.context.flow),
                        ERROR_EVENT_CONTENT_TYPE.to_owned(),
                    ))
                }
            };

            let (lit, _tmp_mem_update) = exec_path_actions(
                &mut lit,
                dis_warnings,
                None,
                &Some(path),
                &content_type,
                data,
                msg_data,
                sender,
            )?;

            Ok(lit)
        }
        None => Ok(PrimitiveString::get_literal(
            &data.event.content_value,
            interval.to_owned(),
        )),
    }
}

pub fn gen_literal_from_component(
    interval: Interval,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match path {
        Some(path) => {
            let mut path = resolve_path(path, &DisplayWarnings::On, data, msg_data, sender)?;

            if let Some((_interval, function_name)) = path.first() {
                if let PathLiteral::Func {
                    name,
                    interval,
                    args,
                } = function_name
                {
                    if let Some(component) = data.custom_component.get(name) {
                        let mut lit = gen_generic_component(name, true, &data.context.flow, interval, args, component)?;

                        path.drain(..1);

                        let (lit, _tmp_mem_update) = exec_path_actions(
                            &mut lit,
                            &DisplayWarnings::On,
                            None,
                            &Some(path),
                            &ContentType::Primitive,
                            data,
                            msg_data,
                            sender,
                        )?;

                        return Ok(lit);
                    }
                }
            }

            Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                ERROR_COMPONENT_UNKNOWN.to_owned(),
            ))
        }
        None => Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            ERROR_COMPONENT_NAMESPACE.to_owned(),
        )),
    }
}

pub fn get_literal_from_metadata(
    path: &[(Interval, PathLiteral)],
    dis_warnings: &DisplayWarnings,
    data: &mut Data,
    msg_data: &mut MessageData,
    interval: Interval,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let mut lit = match path.get(0) {
        Some((interval, PathLiteral::MapIndex(name))) => match data.context.metadata.get(name) {
            Some(lit) => lit.to_owned(),
            None => PrimitiveNull::get_literal(interval.to_owned()),
        },
        _ => {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                ERROR_FIND_BY_INDEX.to_owned(),
            ));
        }
    };

    let content_type = ContentType::get(&lit);
    let (lit, _tmp_mem_update) = exec_path_actions(
        &mut lit,
        dis_warnings,
        None,
        &Some(path[1..].to_owned()),
        &content_type,
        data,
        msg_data,
        sender,
    )?;
    Ok(lit)
}
