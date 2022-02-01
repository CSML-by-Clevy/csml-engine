use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{
    PrimitiveObject, PrimitiveString,
    PrimitiveInt, PrimitiveType
};
use crate::data::{ast::Interval, ArgsType, Literal};
use crate::error_format::*;
use std::collections::HashMap;
use std::env;

use std::sync::Arc;
use ureq::{Request, Response};
use log::{debug, error, info,};

use rustls::{
    Certificate,
    client::{ServerCertVerified, ServerName, ServerCertVerifier}
};

////////////////////////////////////////////////////////////////////////////////
/// DATA TYPES
////////////////////////////////////////////////////////////////////////////////

pub(crate) struct NoVerifier;

impl ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}


////////////////////////////////////////////////////////////////////////////////
/// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_value<'lifetime, T: 'static>(
    key: &str,
    object: &'lifetime HashMap<String, Literal>,
    flow_name: &str,
    interval: Interval,
    error: &'static str,
) -> Result<&'lifetime T, ErrorInfo> {
    if let Some(literal) = object.get(key) {
        Literal::get_value::<T>(&literal.primitive, flow_name, interval, format!("'{}' {}", key, error))
    } else {
        Err(gen_error_info(
            Position::new(interval, flow_name),
            format!("'{}' {}", key, error),
        ))
    }
}

fn set_http_error_info(
    response: &Response,
    error_message: String,
    flow_name: &str,
    interval: Interval,
) ->  ErrorInfo {
    let mut error = gen_error_info(
        Position::new(interval, flow_name),
        error_message,
    );

    let status = PrimitiveInt::get_literal(response.status() as i64, interval);
    error.add_info("status", status);

    let headers = response.headers_names()
    .iter()
        .fold(HashMap::new(), |mut acc, name| {
        if let Some(header) = response.header(name) {
            let value = PrimitiveString::get_literal(header, interval);
            acc.insert(name.to_owned(), value);
        }
        acc
    });

    error.add_info("headers", PrimitiveObject::get_literal(&headers , interval));

    error
}

pub fn get_ssl_state(object: &HashMap<String, Literal>) -> bool {
    match object.get("disable_ssl_verify") {
        Some(val) 
        if val.primitive.get_type() == PrimitiveType::PrimitiveBoolean => val.primitive.as_bool(),
        _ => false
    }
}

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_url(object: &HashMap<String, Literal>, flow_name: &str, interval: Interval) -> Result<String, ErrorInfo> {
    let url = &mut get_value::<String>("url", object, flow_name,interval, ERROR_HTTP_GET_VALUE)?.to_owned();

    if object.get("query").is_some() {
        let query =
            get_value::<HashMap<String, Literal>>("query", object, flow_name,interval, ERROR_HTTP_GET_VALUE)?;

        let length = query.len();
        if length > 0 {
            url.push_str("?");

            for (index, key) in query.keys().enumerate() {
                let value = match query.get(key) {
                    Some(val) => {val.primitive.to_string()},
                    None => {
                        return Err(gen_error_info(
                            Position::new(interval, flow_name),
                            format!("'{}' {}", key, ERROR_HTTP_GET_VALUE),
                        ))
                    }
                };

                url.push_str(key);
                url.push_str("=");
                url.push_str(&value);

                if index + 1 < length {
                    url.push_str("&");
                }
            }
        }
    }

    Ok(url.to_owned())
}

fn get_no_certificate_verifier_agent() -> ureq::Agent {
    let root_store = rustls::RootCertStore::empty();

    let mut tls_config = rustls::ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth();

    tls_config.dangerous().set_certificate_verifier(Arc::new(NoVerifier));

    ureq::AgentBuilder::new()
    .tls_config(Arc::new(tls_config))
    .build()
}

fn get_http_request(
    method: &str,
    url: &str,
    flow_name: &str,
    interval: Interval,
    is_ssl_disable: bool
) -> Result<Request, ErrorInfo> {

    if let Ok(disable_ssl_verify) = env::var("DISABLE_SSL_VERIFY") {
        match disable_ssl_verify.parse::<bool>() {
            Ok(low_data) if low_data || is_ssl_disable => {
                let agent = get_no_certificate_verifier_agent();

                let request = match method {
                    delete if delete == "delete" => agent.delete(url),
                    put if put == "put" => agent.put(url),
                    patch if patch == "patch" => agent.request("PATCH",url),
                    post if post == "post" => agent.post(url),
                    get if get == "get" => agent.get(url),
                    _ => {
                        return Err(gen_error_info(
                            Position::new(interval, flow_name),
                            ERROR_HTTP_UNKNOWN_METHOD.to_string(),
                        ))
                    }
                };

                return Ok(request)
            }
            _ => {}
        }
    }

    let request = match method {
        delete if delete == "delete" => ureq::delete(url),
        put if put == "put" => ureq::put(url),
        patch if patch == "patch" => ureq::request("PATCH",url),
        post if post == "post" => ureq::post(url),
        get if get == "get" => ureq::get(url),
        _ => {
            return Err(gen_error_info(
                Position::new(interval, flow_name),
                ERROR_HTTP_UNKNOWN_METHOD.to_string(),
            ))
        }
    };

    Ok(request)
}

pub fn http_request(
    object: &HashMap<String, Literal>,
    method: &str,
    flow_name: &str,
    interval: Interval,
    is_app_call: bool,
) -> Result<serde_json::Value, ErrorInfo> {
    let url = get_url(object, flow_name, interval)?;
    let is_ssl_disable = get_ssl_state(object);

    let header =
        get_value::<HashMap<String, Literal>>("header", object, flow_name, interval, ERROR_HTTP_GET_VALUE)?;

    let mut request = get_http_request(method, &url, flow_name, interval, is_ssl_disable)?;

    for key in header.keys() {
        let value = match header.get(key) {
            Some(val) => {val.primitive.to_string()},
            None => {
                return Err(gen_error_info(
                    Position::new(interval, flow_name),
                    format!("'{}' {}", key, ERROR_HTTP_GET_VALUE),
                ))
            }
        };

        request = request.set(key, &value);
    }

    info!("Make Http call");
    debug!("Make Http call request info: {:?}", request);

    let response = match object.get("body") {
        Some(body) => request.send_json(body.primitive.to_json()),
        None => request.call(),
    };

    match response {
        Ok(response) => {
            let mut error = set_http_error_info(
                &response,
                ERROR_FAIL_RESPONSE_JSON.to_owned(),
                flow_name,
                interval
            );
            let error_body = "Invalid Response format, please send a json or a valid UTF-8 sequence";
            error.add_info("body", PrimitiveString::get_literal(error_body, interval));

            match response.into_string() {
                Ok(value) => {
                    match serde_json::from_str::<serde_json::Value>(&value) {
                        Ok(value) => Ok(value),
                        Err(err) => {
                            error!("Http response Json parsing failed: {:?}", err);
                            Err(error)
                        }
                    }
                },
                Err(err) => {
                    error!("Http response Json parsing failed: {:?}", err);
                    Err(error)
                },
            }
        }
        Err(err) => {
            // if this function is call by the APP system hide the apps_endpoint for de error message
            let error_message = match is_app_call {
                true => {
                    if let ureq::Error::Status(code, _) = err {
                        format!("Apps service: status code {}", code)
                    } else {
                        format!("Apps service: error")
                    }
                },
                false => err.to_string()
            };

            error!("Http call failed: {:?}", error_message);

            if let ureq::Error::Status(_, response) = err {
                let mut error = set_http_error_info(&response, error_message, flow_name, interval);

                let body = match response.into_string() {
                    Ok(body) => body,
                    Err(_) => {
                        "Invalid Response format, please send a json or a valid UTF-8 sequence".to_owned()
                    }
                };

                error.add_info("body", PrimitiveString::get_literal(&body, interval));

                Err(error)
            } else {
                Err(gen_error_info(Position::new(interval, flow_name), error_message))
            }
        }
    }
}

pub fn http(args: ArgsType, flow_name: &str, interval: Interval) -> Result<Literal, ErrorInfo> {
    let mut http: HashMap<String, Literal> = HashMap::new();
    let mut header = HashMap::new();

    match args.get("url", 0) {
        Some(literal) if literal.primitive.get_type() == PrimitiveType::PrimitiveString => {
            header.insert(
                "Content-Type".to_owned(),
                PrimitiveString::get_literal("application/json", interval),
            );
            header.insert(
                "Accept".to_owned(),
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

            args.populate(&mut http, &["url", "header", "query", "body"], flow_name, interval)?;

            let mut result = PrimitiveObject::get_literal(&http, interval);

            result.set_content_type("http");

            Ok(result)
        }
        _ => Err(gen_error_info(
            Position::new(interval, flow_name),
            ERROR_HTTP.to_owned(),
        )),
    }
}
