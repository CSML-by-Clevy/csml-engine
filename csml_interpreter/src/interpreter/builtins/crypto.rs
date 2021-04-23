use crate::data::position::Position;
use crate::data::primitive::PrimitiveObject;
use std::collections::HashMap;

use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;

pub fn crypto(args: ArgsType, flow_name: &str,interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut map: HashMap<String, Literal> = HashMap::new();

    match args.get("value", 0) {
        Some(value) => {
            map.insert("value".to_owned(), value.to_owned());
            let mut result = PrimitiveObject::get_literal(&map, interval);

            result.set_content_type("crypto");
            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_HTTP.to_owned(),
        )),
    }
}
