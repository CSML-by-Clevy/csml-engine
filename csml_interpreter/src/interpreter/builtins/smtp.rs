use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::PrimitiveBoolean;
use crate::data::primitive::{object::PrimitiveObject, PrimitiveInt, PrimitiveType};
use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn smtp(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("smtp_server", 0) {
        Some(server) if server.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let mut map: HashMap<String, Literal> = HashMap::new();

            map.insert("smtp_server".to_owned(), server.to_owned());

            // set default port to [465] for TLS connections [RFC8314](https://tools.ietf.org/html/rfc8314)
            map.insert("port".to_owned(), PrimitiveInt::get_literal(465, interval));

            map.insert(
                "tls".to_owned(),
                PrimitiveBoolean::get_literal(true, interval),
            );

            let mut result = PrimitiveObject::get_literal(&map, interval);

            result.set_content_type("smtp");
            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_SMTP.to_owned(),
        )),
    }
}
