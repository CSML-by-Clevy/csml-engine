use crate::data::{ast::Interval, Literal};
use crate::error_format::data::ErrorInfo;
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

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn http_get(
    object: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let url = get_value::<String>("url", object, interval)?;
    let header = get_value::<HashMap<String, Literal>>("header", object, interval)?;

    let mut request = ureq::get(url);

    for key in header.keys() {
        let value = get_value::<String>(key, header, interval)?;

        request.set(key, value);
    }

    let response = request.call();

    println!("status: {:#?}", response.status());
    println!("body: {:#?}", response.into_json().unwrap());

    unimplemented!();
}

pub fn http_delete(
    object: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    unimplemented!();
}

pub fn http_put(
    object: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    unimplemented!();
}

pub fn http_patch(
    object: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    unimplemented!();
}

pub fn http_post(
    object: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    unimplemented!();
}
