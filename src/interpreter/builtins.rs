pub mod api;
pub mod buttons;
pub mod format;
pub mod functions;
pub mod http;
pub mod media;
pub mod tools;

use crate::data::{ast::*, tokens::*, Data, Literal};
use crate::error_format::ErrorInfo;
use std::collections::HashMap;

use api::api;
use buttons::*;
use format::*;
use functions::*;
use media::*;

pub fn match_builtin(
    name: &str,
    args: HashMap<String, Literal>,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match name {
        // CUSTOM
        TYPING => typing(args, interval),
        WAIT => wait(args, interval),
        URL => url(args, interval),
        IMAGE => img(args, interval),
        QUESTION => question(args, interval),
        VIDEO => video(args, interval),
        AUDIO => audio(args, interval),
        BUTTON => button(args, interval),
        CAROUSEL => carousel(args, interval),
        CARD => card(args, interval),
        HTTP => http(args, interval),
        FILE => file(args, interval),

        // DEFAULT
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
