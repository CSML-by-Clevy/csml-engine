pub mod api;
pub mod crypto;
pub mod format;
pub mod functions;
pub mod http;
pub mod smtp;
pub mod jwt;
pub mod time;
pub mod exists;

pub mod tools;

use crate::data::{
    ast::*, position::Position, tokens::*, ArgsType, Data, Literal, MessageData, MSG,
};
use crate::error_format::{gen_error_info, ErrorInfo, ERROR_NATIVE_COMPONENT};
use crate::interpreter::variable_handler::gen_generic_component::gen_generic_component;
use std::sync::mpsc;

use api::api;
use crypto::crypto;
use format::*;
use functions::*;
use http::http;
use smtp::smtp;
use jwt::jwt;
use time::time;
use exists::exists;

pub fn match_native_builtin(
    name: &str,
    args: ArgsType,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    if let Some(component) = data.native_component.get(name) {
        gen_generic_component(name, false, &data.context.flow, &interval, &args, component)
    } else {
        Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            format!("{} [{}]", ERROR_NATIVE_COMPONENT, name),
        ))
    }
}

pub fn match_builtin(
    name: &str,
    args: ArgsType,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match name {
        HTTP => http(args, &data.context.flow, interval),
        SMTP => smtp(args, &data.context.flow, interval),
        BASE64 => base64(args, &data.context.flow, interval),
        HEX => hex(args, &data.context.flow, interval),
        FN | APP => api(args, interval, data, msg_data, sender),
        ONE_OF => one_of(args, &data.context.flow, interval),
        SHUFFLE => shuffle(args, &data.context.flow, interval),
        LENGTH => length(args, &data.context.flow, interval),
        FIND => find(args, &data.context.flow, interval),
        RANDOM => random(interval),
        DEBUG => debug(args, interval),
        FLOOR => floor(args, &data.context.flow, interval),
        UUID => uuid_command(args, &data.context.flow, interval),
        JWT => jwt(args, &data.context.flow, interval),
        CRYPTO => crypto(args, &data.context.flow, interval),
        TIME => time(args, &data.context.flow, interval),
        EXISTS => exists(args, data, interval),

        //old builtin
        _object => object(args, &data.context.flow, interval),
    }
}
