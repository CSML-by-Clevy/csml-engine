use crate::data::{
    ast::Interval,
    error_info::ErrorInfo,
    position::Position,
    primitive::PrimitiveType,
    primitive::{Data, PrimitiveInt, PrimitiveObject},
    Literal,
};
use crate::error_format::*;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_date_string(
    args: &HashMap<String, Literal>,
    index: usize,
    data: &mut Data,
    interval: Interval,
    error: &str,
) -> Result<String, ErrorInfo> {
    match args.get(&format!("arg{}", index)) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            let value = Literal::get_value::<String>(
                &literal.primitive,
                &data.context.flow,
                literal.interval,
                format!("{}", error),
            )?;

            Ok(value.to_owned())
        }
        _ => {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("{}", error),
            ))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_date(args: &HashMap<String, Literal>) -> [i64; 7] {
    let mut date: [i64; 7] = [0; 7];

    // set default month, day, and hour to 1 year does not need to have a default
    // value because set_date_at expect at least the year value as parameter
    date[1] = 1; // month
    date[2] = 1; // day
    date[3] = 1; // hour

    let len = args.len();

    for index in 0..len {
        match args.get(&format!("arg{}", index)) {
            Some(lit) if lit.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                let value = serde_json::from_str(&lit.primitive.to_string()).unwrap();

                date[index] = value;
            }
            _ => {}
        }
    }

    date
}

pub fn parse_rfc3339(
    args: &HashMap<String, Literal>,
    data: &mut Data,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let usage = "invalid value, 'parse(String)' expect a valid RFC 3339 and ISO 8601 date and time string such as '1996-12-19T00:00:00Z'";

    let date_str = get_date_string(args, 0, data, interval, usage)?;

    // autocomplete format with default values
    let date_str = match date_str.len() {
        4 => format!(
            "{}-{a1}-{a1}T{a2}:{a2}:{a2}Z",
            date_str,
            a1 = "01",
            a2 = "00"
        ),
        7 => format!("{}-{a1}T{a2}:{a2}:{a2}Z", date_str, a1 = "01", a2 = "00"),
        10 => format!("{}T{a2}:{a2}:{a2}Z", date_str, a2 = "00"),
        13 => format!("{}:{a2}:{a2}Z", date_str, a2 = "00"),
        16 => format!("{}:{a2}Z", date_str, a2 = "00"),
        19 => format!("{}Z", date_str),
        _ => date_str.to_owned(),
    };

    let date = match DateTime::parse_from_rfc3339(&date_str) {
        Ok(date) => date,
        Err(_) => {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("{}", usage),
            ))
        }
    };

    let mut object = HashMap::new();

    object.insert(
        "milliseconds".to_owned(),
        PrimitiveInt::get_literal(date.timestamp_millis(), interval),
    );
    let mut lit = PrimitiveObject::get_literal(&object, interval);
    lit.set_content_type("time");

    Ok(lit)
}

pub fn pasre_from_str(
    args: &HashMap<String, Literal>,
    data: &mut Data,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let usage = "invalid value,
    expect date and a specified format to parse the date example:
    parse(\"1983 08 13 12:09:14.274\", \"%Y %m %d %H:%M:%S%.3f\")";

    let date_str = get_date_string(args, 0, data, interval, usage)?;

    let format_str = get_date_string(args, 1, data, interval, usage)?;

    let date_millis = if let Ok(date) = DateTime::parse_from_str(&date_str, &format_str) {
        date.timestamp_millis()
    } else if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(&date_str, &format_str) {
        let date = DateTime::<Utc>::from_utc(naive_datetime, Utc);
        date.timestamp_millis()
    } else if let Ok(naive_date) = NaiveDate::parse_from_str(&date_str, &format_str) {
        let naive_datetime: NaiveDateTime = naive_date.and_hms(0, 0, 0);
        let date = DateTime::<Utc>::from_utc(naive_datetime, Utc);
        date.timestamp_millis()
    } else {
        return Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            format!("{}", usage),
        ));
    };

    let mut object = HashMap::new();
    object.insert(
        "milliseconds".to_owned(),
        PrimitiveInt::get_literal(date_millis, interval),
    );
    let mut lit = PrimitiveObject::get_literal(&object, interval);
    lit.set_content_type("time");

    Ok(lit)
}
