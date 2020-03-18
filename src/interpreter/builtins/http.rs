use crate::data::{ast::Interval, Literal};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::json_to_rust::json_to_literal;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
/// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_value<'lifetime, T: 'static>(
    key: &str,
    object: &'lifetime HashMap<String, Literal>,
    interval: Interval,
) -> Result<&'lifetime T, ErrorInfo> {
    match object.get(key) {
        Some(literal) => {
            let url = Literal::get_value::<T>(&literal.primitive)?;

            Ok(url)
        }
        None => Err(ErrorInfo {
            message: format!("csml: error on .get({})", key),
            interval,
        }),
    }
}

fn get_url(object: &HashMap<String, Literal>, interval: Interval) -> Result<String, ErrorInfo> {
    let url = &mut get_value::<String>("url", object, interval)?.to_owned();
    let query = get_value::<HashMap<String, Literal>>("query", object, interval)?;

    if !query.is_empty() {
        let length = query.len();

        url.push_str("?");

        for (index, key) in query.keys().enumerate() {
            let value = get_value::<String>(key, query, interval)?;

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
) -> Result<Literal, ErrorInfo> {
    let url = get_url(object, interval)?;
    let header = get_value::<HashMap<String, Literal>>("header", object, interval)?;
    let body = get_value::<HashMap<String, Literal>>("body", object, interval)?;

    let mut request = function(&url);

    for key in header.keys() {
        let value = get_value::<String>(key, header, interval)?;

        request.set(key, value);
    }

    let response = request.send_json(serialize(body));
    let status = response.status();
    let body = response.into_json();

    match body {
        Ok(value) => json_to_literal(&value, interval),
        Err(_) => Err(ErrorInfo {
            message: format!("error {}: failed to read response as JSON", status),
            interval,
        }),
    }
}
