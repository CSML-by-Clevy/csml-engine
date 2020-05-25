use crate::data::error_info::ErrorInfo;
use crate::data::primitive::PrimitiveObject;
use crate::data::{Interval, Literal};
use nom::lib::std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn gen_generic_component(
    name: &str,
    interval: &Interval,
    args: &Literal,
    header: &serde_json::Value,
) -> Result<Literal, ErrorInfo> {
    let hashmap: HashMap<String, Literal> = HashMap::new();
    let mut literal = PrimitiveObject::get_literal(&hashmap, *interval);

    literal.set_content_type(name);

    Ok(literal)
}
