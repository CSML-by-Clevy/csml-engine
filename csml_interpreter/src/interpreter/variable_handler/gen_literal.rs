use crate::data::literal::ContentType;
use crate::data::position::Position;
use crate::data::primitive::string::PrimitiveString;
use crate::data::{ast::PathLiteral, primitive::PrimitiveNull};
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
    condition: bool,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match path {
        Some(path) => {
            let path = resolve_path(path, condition, data, msg_data, sender)?;
            let mut lit = json_to_literal(&data.event.content, interval.to_owned())?;

            lit.set_content_type("event");

            let content_type = match ContentType::get(&lit) {
                ContentType::Event(_) => ContentType::Event(data.event.content_type.to_owned()),
                _ => {
                    return Err(gen_error_info(
                        Position::new(interval),
                        ERROR_EVENT_CONTENT_TYPE.to_owned(),
                    ))
                }
            };

            let (lit, _tmp_mem_update) = exec_path_actions(
                &mut lit,
                condition,
                None,
                &Some(path),
                &content_type,
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

// pub fn gen_literal_from_env(
//     interval: Interval,
//     condition: bool,
//     path: Option<&[(Interval, PathState)]>,
//     data: &mut Data,
//     msg_data: &mut MessageData,
//     sender: &Option<mpsc::Sender<MSG>>,
// ) -> Result<Literal, ErrorInfo> {
//     match path {
//         Some(path) => {
//             let path = resolve_path(path, condition, data, msg_data, sender)?;
//             let mut lit = json_to_literal(&data.event.content, interval.to_owned())?;

//             lit.set_content_type("event");

//             let content_type = match ContentType::get(&lit) {
//                 ContentType::Event(_) => ContentType::Event(data.event.content_type.to_owned()),
//                 _ => {
//                     return Err(gen_error_info(
//                         Position::new(interval),
//                         ERROR_EVENT_CONTENT_TYPE.to_owned(),
//                     ))
//                 }
//             };

//             let (lit, _tmp_mem_update) = exec_path_actions(
//                 &mut lit,
//                 condition,
//                 None,
//                 &Some(path),
//                 &content_type,
//                 msg_data,
//                 sender,
//             )?;

//             Ok(lit)
//         }
//         None => Ok(PrimitiveNull::get_literal(
//             interval.to_owned(),
//         )),
//     }
// }

pub fn gen_literal_from_component(
    interval: Interval,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match path {
        Some(path) => {
            let mut path = resolve_path(path, false, data, msg_data, sender)?;

            if let Some((_interval, function_name)) = path.first() {
                if let PathLiteral::Func {
                    name,
                    interval,
                    args,
                } = function_name
                {
                    if let Some(component) = data.custom_component.get(name) {
                        let mut lit = gen_generic_component(name, true, interval, args, component)?;

                        path.drain(..1);

                        let (lit, _tmp_mem_update) = exec_path_actions(
                            &mut lit,
                            false,
                            None,
                            &Some(path),
                            &ContentType::Primitive,
                            msg_data,
                            sender,
                        )?;

                        return Ok(lit);
                    }
                }
            }

            Err(gen_error_info(
                Position::new(interval),
                ERROR_COMPONENT_UNKNOWN.to_owned(),
            ))
        }
        None => Err(gen_error_info(
            Position::new(interval),
            ERROR_COMPONENT_NAMESPACE.to_owned(),
        )),
    }
}

pub fn get_literal_form_metadata(
    path: &[(Interval, PathLiteral)],
    condition: bool,
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
                Position::new(interval),
                ERROR_FIND_BY_INDEX.to_owned(),
            ));
        }
    };

    let content_type = ContentType::get(&lit);
    let (lit, _tmp_mem_update) = exec_path_actions(
        &mut lit,
        condition,
        None,
        &Some(path[1..].to_owned()),
        &content_type,
        msg_data,
        sender,
    )?;
    Ok(lit)
}
