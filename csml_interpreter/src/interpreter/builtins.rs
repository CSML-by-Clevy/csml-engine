pub mod api;
pub mod format;
pub mod functions;
pub mod http;
pub mod jwt;
pub mod crypto;
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
use http::http;
use jwt::jwt;
use crypto::crypto;

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
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match name {
        HTTP => http(args, interval),
        BASE64 => base64(args, interval),
        HEX => hex(args, interval),
        FN | APP => api(args, interval, data, msg_data, sender),
        ONE_OF => one_of(args, interval),
        SHUFFLE => shuffle(args, interval),
        LENGTH => length(args, interval),
        FIND => find(args, interval),
        RANDOM => random(interval),
        DEBUG => debug(args, interval),
        FLOOR => floor(args, interval),
        UUID => uuid_command(args, interval),
        JWT => jwt(args, interval),
        CRYPTO => crypto(args, interval),

        //old builtin
        _object => object(args, interval),
    }
}
