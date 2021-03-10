use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{object::PrimitiveObject, string::PrimitiveString, PrimitiveType};
use crate::data::{ast::Interval, ArgsType, Literal};
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

    let mut request = function(&url);

    for key in header.keys() {
        let value = get_value::<String>(key, header, interval, ERROR_HTTP_GET_VALUE)?;

        request.set(key, value);
    }

    let response = match object.get("body") {
        Some(body) => request.send_json(body.primitive.to_json()),
        None => request.call(),
    };

    if let Some(err) = response.synthetic_error() {
        if let Ok(var) = env::var("DEBUG") {
            if var == "true" {
                eprintln!("FN request failed: {:?}", err.body_text());
            }
        }
        return Err(gen_error_info(Position::new(interval), err.body_text()));
    }

    match response.into_json() {
        Ok(value) => Ok(value),
        Err(_) => Err(gen_error_info(
            Position::new(interval),
            ERROR_FAIL_RESPONSE_JSON.to_owned(),
        )),
    }
}

pub fn http(args: ArgsType, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut http: HashMap<String, Literal> = HashMap::new();
    let mut header = HashMap::new();

    match args.get("url", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            header.insert(
                "content-type".to_owned(),
                PrimitiveString::get_literal("application/json", interval),
            );
            header.insert(
                "accept".to_owned(),
                PrimitiveString::get_literal("application/json,text/*", interval),
            );
            header.insert(
                "User-Agent".to_owned(),
                PrimitiveString::get_literal("csml/v1", interval),
            );

            http.insert("url".to_owned(), literal.to_owned());
            http.insert(
                "method".to_owned(),
                PrimitiveString::get_literal("get", interval),
            );

            let lit_header = PrimitiveObject::get_literal(&header, interval);
            http.insert("header".to_owned(), lit_header);
            let lit_query = PrimitiveObject::get_literal(&HashMap::default(), interval);
            http.insert("query".to_owned(), lit_query);
            let lit_body = PrimitiveObject::get_literal(&HashMap::default(), interval);
            http.insert("body".to_owned(), lit_body);

            args.populate(&mut http, &["url", "header", "query", "body"], interval)?;

            let mut result = PrimitiveObject::get_literal(&http, interval);

            result.set_content_type("http");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval),
            ERROR_HTTP.to_owned(),
        )),
    }
}
