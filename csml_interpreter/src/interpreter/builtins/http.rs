use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::{ast::Interval, Literal};
use crate::error_format::*;
use std::collections::HashMap;
use std::env;

////////////////////////////////////////////////////////////////////////////////
/// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_value<'lifetime, T: 'static>(
    key: &str,
    object: &'lifetime HashMap<String, Literal>,
    interval: Interval,
    error: &'static str,
) -> Result<&'lifetime T, ErrorInfo> {
    if let Some(literal) = object.get(key) {
        Literal::get_value::<T>(&literal.primitive, interval, format!("'{}' {}", key, error))
    } else {
        Err(gen_error_info(
            Position::new(interval),
            format!("'{}' {}", key, error),
        ))
    }
}

fn get_url(object: &HashMap<String, Literal>, interval: Interval) -> Result<String, ErrorInfo> {
    let url = &mut get_value::<String>("url", object, interval, ERROR_HTTP_GET_VALUE)?.to_owned();
    let query =
        get_value::<HashMap<String, Literal>>("query", object, interval, ERROR_HTTP_GET_VALUE)?;

    if !query.is_empty() {
        let length = query.len();

        url.push_str("?");

        for (index, key) in query.keys().enumerate() {
            let value = get_value::<String>(key, query, interval, ERROR_HTTP_QUERY_VALUES)?;

            url.push_str(key);
            url.push_str("=");
            url.push_str(value);

            if index + 1 < length {
                url.push_str("&");
            }
        }
    }

    Ok(url.to_owned())
}

fn serialize(object: &HashMap<String, Literal>) -> serde_json::Value {
    let mut json: serde_json::map::Map<String, serde_json::Value> = serde_json::map::Map::new();

    for (key, literal) in object.iter() {
        json.insert(key.to_owned(), literal.primitive.to_json());
    }

    serde_json::Value::Object(json)
}

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn http_request(
    object: &HashMap<String, Literal>,
    function: fn(&str) -> ureq::Request,
    interval: Interval,
) -> Result<serde_json::Value, ErrorInfo> {
    let url = get_url(object, interval)?;
    let header =
        get_value::<HashMap<String, Literal>>("header", object, interval, ERROR_HTTP_GET_VALUE)?;

    let body =
        get_value::<HashMap<String, Literal>>("body", object, interval, ERROR_HTTP_GET_VALUE)?;

    let mut request = function(&url);

    for key in header.keys() {
        let value = get_value::<String>(key, header, interval, ERROR_HTTP_GET_VALUE)?;

        request.set(key, value);
    }

    let response = request.send_json(serialize(body));
    // let status = response.status();
    let body = response.into_json();

    match body {
        Ok(value) => Ok(value),
        Err(err) => {
            if let Ok(var) = env::var("DEBUG") {
                if var == "true" {
                    println!(
                        "FN request failed: {:?}",
                        err
                    );
                }
            }

            Ok(serde_json::Value::Null)
        },
        // Err(gen_error_info(
        //     interval,
        //     format!("{}: {}", status, ERROR_FAIL_RESPONSE_JSON),
        // ))
    }
}
