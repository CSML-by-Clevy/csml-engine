pub mod api;
pub mod format;
pub mod functions;
pub mod http;
pub mod tools;

use crate::data::{
    ast::*, position::Position, tokens::*, ArgsType, Data, Literal, MessageData, MSG,
};
use crate::error_format::{gen_error_info, ErrorInfo, ERROR_NATIVE_COMPONENT};
use crate::interpreter::variable_handler::gen_generic_component::gen_generic_component;
use std::sync::mpsc;

use api::api;
use format::*;
use functions::*;

pub fn match_native_builtin(
    name: &str,
    args: ArgsType,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    if let Some(component) = data.native_component.get(name) {
        gen_generic_component(name, false, &interval, &args, component)
    } else {
        Err(gen_error_info(
            Position::new(interval),
            format!("{} [{}]", ERROR_NATIVE_COMPONENT, name),
        ))
    }
}

pub fn match_builtin(
    name: &str,
    args: ArgsType,
    interval: Interval,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match name {
        HTTP => http(args, interval),
        FN => api(args, interval, data, root, sender),
        ONE_OF => one_of(args, interval),
        SHUFFLE => shuffle(args, interval),
        LENGTH => length(args, interval),
        FIND => find(args, interval),
        RANDOM => random(interval),
        FLOOR => floor(args, interval),

        //old builtin
        _object => object(args, interval),
    }
}
