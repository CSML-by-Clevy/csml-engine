use crate::data::position::Position;
use crate::data::primitive::{PrimitiveObject, PrimitiveString};
use std::collections::HashMap;

use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;

pub fn jwt(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut jwt_map: HashMap<String, Literal> = HashMap::new();
    let mut header = HashMap::new();

    match args.get("jwt", 0) {
        Some(jwt) => {
            jwt_map.insert("jwt".to_owned(), jwt.to_owned());

            header.insert(
                "typ".to_owned(),
                PrimitiveString::get_literal("JWT", interval),
            );
            header.insert(
                "alg".to_owned(),
                PrimitiveString::get_literal("HS256", interval),
            );
            let lit_header = PrimitiveObject::get_literal(&header, interval);
            jwt_map.insert("header".to_owned(), lit_header);

            let mut result = PrimitiveObject::get_literal(&jwt_map, interval);
            result.set_content_type("jwt");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval),
            ERROR_HTTP.to_owned(),
        )),
    }
}
