use crate::data::error_info::ErrorInfo;
use crate::data::primitive::{PrimitiveInt, PrimitiveObject};
use crate::data::{ast::Interval, ArgsType, Literal};
use chrono::Utc;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn time(_args: ArgsType, _flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut time: HashMap<String, Literal> = HashMap::new();
    let date = Utc::now();

    time.insert(
        "milliseconds".to_owned(),
        PrimitiveInt::get_literal(date.timestamp_millis(), interval),
    );

    let mut result = PrimitiveObject::get_literal(&time, interval);

    result.set_content_type("time");

    Ok(result)
}
