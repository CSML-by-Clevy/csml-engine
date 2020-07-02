pub mod api;
pub mod buttons;
pub mod format;
pub mod functions;
pub mod http;
pub mod media;
pub mod tools;

pub mod components;

use crate::data::{ast::*, tokens::*, Data, Literal};
use crate::interpreter::variable_handler::gen_generic_component::gen_generic_component;
use crate::error_format::ErrorInfo;
use std::collections::HashMap;

use buttons::*;
use media::*;

use api::api;
use format::*;
use functions::*;

pub fn match_builtin(
    name: &str,
    args: HashMap<String, Literal>,
    args_v2: Literal,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {

    match name {
        // Native
        name if data.native_component.contains_key(name) => {
            if let Some(component) = data.native_component.get(name) {
                gen_generic_component(name, &interval, &args_v2, component)
            } else {
                panic!("error in native_component")
            }
        },
        
        // TYPING => typing(args, interval),
        // WAIT => wait(args, interval),
        // URL => url(args, interval),
        // IMAGE => img(args, interval),
        // AUDIO => audio(args, interval),
        // VIDEO => video(args, interval),
        // FILE => file(args, interval),

        // BUTTON => button(args, interval),
        // QUESTION => question(args, interval),

        // CARD => card(args, interval),
        // CAROUSEL => carousel(args, interval),


        // DEFAULT
        HTTP => http(args, interval),
        FN => api(args, interval, data),
        ONE_OF => one_of(args, interval),
        SHUFFLE => shuffle(args, interval),
        LENGTH => length(args, interval),
        FIND => find(args, interval),
        RANDOM => random(interval),
        FLOOR => floor(args, interval),

        //old builtin
        OBJECT => object(args, interval),

        _ => text(args, interval),
    }
}
