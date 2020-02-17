pub mod api;
pub mod buttons;
pub mod format;
pub mod functions;
pub mod media;
pub mod tools;

use crate::data::{ast::*, tokens::*, Data, Literal};
use crate::error_format::ErrorInfo;
use std::collections::HashMap;

use api::api;
use buttons::button;
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
        TYPING => typing(args, name.to_owned(), interval),
        WAIT => wait(args, name.to_owned(), interval),
        URL => url(args, name.to_owned(), interval),
        IMAGE => img(args, name.to_owned(), interval),
        QUESTION => question(args, name.to_owned(), interval),
        VIDEO => video(args, name.to_owned(), interval),
        AUDIO => audio(args, name.to_owned(), interval),
        BUTTON => button(args, name.to_owned(), interval),
        OBJECT => object(args, interval),

        // DEFAULT
        FN => api(args, interval, data),
        ONE_OF => one_of(args, interval),
        SHUFFLE => shuffle(args, interval),
        LENGTH => length(args, interval),
        FIND => find(args, interval),
        RANDOM => random(interval),
        FLOOR => floor(args, interval),
        _ => text(args, name.to_owned(), interval),
    }
}
