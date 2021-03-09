use crate::data::{position::Position, primitive::Primitive};
use crate::data::primitive::{
    PrimitiveType, PrimitiveObject, PrimitiveBoolean, PrimitiveFloat, PrimitiveInt, PrimitiveString,
};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;

// use uuid::{Uuid, v1::{Context, Timestamp}};
// use rand::seq::SliceRandom;
// use rand::Rng;

fn update_content_type(literal: &mut Literal) -> Result<(), ErrorInfo> {
    literal.content_type = "jwt".to_owned();

    match literal.primitive.get_type() {
        PrimitiveType::PrimitiveObject => {
            let map= Literal::get_mut_value::<HashMap<String, Literal>>(
                &mut literal.primitive,
                literal.interval.to_owned(),
                "error_message".to_owned(),
            )?;

            for (_, value) in map.iter_mut() {
                update_content_type(value)?;
            }
        },
        PrimitiveType::PrimitiveArray => {
            let vec= Literal::get_mut_value::<Vec<Literal>>(
                &mut literal.primitive,
                literal.interval.to_owned(),
                "error_message".to_owned(),
            )?;

            for value in vec.iter_mut() {
                update_content_type(value)?;
            }
        },
        _ => {},
    }

    Ok(())
}

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