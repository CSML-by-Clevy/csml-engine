use crate::data::primitive::PrimitiveString;
use crate::data::{ast::Interval, Literal};

pub fn gen_error(error_msg: String, interval: Interval) -> Literal {
    let mut error = PrimitiveString::get_literal(&error_msg, interval);

    error.set_content_type("error");

    error
}
