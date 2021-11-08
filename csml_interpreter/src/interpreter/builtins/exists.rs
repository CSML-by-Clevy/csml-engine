use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{PrimitiveBoolean, PrimitiveType};
use crate::data::{ast::{Interval, Identifier}, ArgsType, Literal, Data};
use crate::interpreter::variable_handler::memory::search_in_memory_type;
use crate::error_format::*;

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn exists(args: ArgsType, data: &mut Data, interval: Interval) -> Result<Literal, ErrorInfo> {
    match args.get("string", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let value = literal.primitive.to_string();
            let ident = Identifier::new(&value, interval.clone());

            let result = search_in_memory_type(
                &ident,
                data
            );

            match result {
                Ok(_) => Ok(PrimitiveBoolean::get_literal(true, interval)),
                Err(_) => Ok(PrimitiveBoolean::get_literal(false, interval)),
            }
        }
        _ => Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            ERROR_VAR_EXISTS.to_owned(),
        )),
    }
}
