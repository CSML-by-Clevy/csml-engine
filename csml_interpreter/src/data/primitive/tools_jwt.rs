use crate::data::{ast::Interval, position::Position, primitive::PrimitiveString, Literal};
use crate::error_format::*;
use crate::interpreter::json_to_literal;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

use super::PrimitiveObject;

fn jwt_algorithm_to_str(algo: &jsonwebtoken::Algorithm) -> String {
    match algo {
        jsonwebtoken::Algorithm::HS256 => "HS256".to_owned(),
        jsonwebtoken::Algorithm::HS384 => "HS384".to_owned(),
        jsonwebtoken::Algorithm::HS512 => "HS512".to_owned(),

        jsonwebtoken::Algorithm::ES256 => "ES256".to_owned(),
        jsonwebtoken::Algorithm::ES384 => "ES384".to_owned(),
        jsonwebtoken::Algorithm::RS256 => "RS256".to_owned(),
        jsonwebtoken::Algorithm::RS384 => "RS384".to_owned(),
        jsonwebtoken::Algorithm::PS256 => "PS256".to_owned(),
        jsonwebtoken::Algorithm::PS384 => "PS384".to_owned(),
        jsonwebtoken::Algorithm::PS512 => "PS512".to_owned(),
        jsonwebtoken::Algorithm::RS512 => "RS512".to_owned(),
    }
}

fn header_to_literal(
    header: &jsonwebtoken::Header,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut map = HashMap::new();

    if let Some(typ) = &header.typ {
        map.insert(
            "typ".to_owned(),
            PrimitiveString::get_literal(typ, interval.to_owned()),
        );
    }
    map.insert(
        "alg".to_owned(),
        PrimitiveString::get_literal(&jwt_algorithm_to_str(&header.alg), interval.to_owned()),
    );
    if let Some(cty) = &header.cty {
        map.insert(
            "cty".to_owned(),
            PrimitiveString::get_literal(cty, interval.to_owned()),
        );
    }
    if let Some(jku) = &header.jku {
        map.insert(
            "jku".to_owned(),
            PrimitiveString::get_literal(jku, interval.to_owned()),
        );
    }
    if let Some(kid) = &header.kid {
        map.insert(
            "kid".to_owned(),
            PrimitiveString::get_literal(kid, interval.to_owned()),
        );
    }
    if let Some(x5u) = &header.x5u {
        map.insert(
            "x5u".to_owned(),
            PrimitiveString::get_literal(x5u, interval.to_owned()),
        );
    }
    if let Some(x5t) = &header.x5t {
        map.insert(
            "x5t".to_owned(),
            PrimitiveString::get_literal(x5t, interval.to_owned()),
        );
    }

    Ok(PrimitiveObject::get_literal(&map, interval.to_owned()))
}

pub fn token_data_to_literal(
    data: jsonwebtoken::TokenData<serde_json::Value>,
    flow_name: &str,
    interval: Interval,
) -> Result<Literal, ErrorInfo> {
    let mut map = HashMap::new();

    let headers = header_to_literal(&data.header, interval)?;
    map.insert("header".to_owned(), headers);

    let claims = json_to_literal(&data.claims, interval.to_owned(), flow_name)?;
    map.insert("payload".to_owned(), claims);

    Ok(PrimitiveObject::get_literal(&map, interval.to_owned()))
}

pub fn get_algorithm(
    lit: &Literal,
    flow_name: &str,
    interval: Interval,
) -> Result<jsonwebtoken::Algorithm, ErrorInfo> {
    let algo = Literal::get_value::<String>(
        &lit.primitive,
        flow_name,
        interval,
        ERROR_JWT_ALGO.to_owned(),
    )?;

    match jsonwebtoken::Algorithm::from_str(algo) {
        Ok(algorithm)
            if algorithm == jsonwebtoken::Algorithm::HS256
                || algorithm == jsonwebtoken::Algorithm::HS384
                || algorithm == jsonwebtoken::Algorithm::HS512 =>
        {
            Ok(algorithm)
        }
        _ => {
            return Err(gen_error_info(
                Position::new(interval, flow_name),
                ERROR_JWT_ALGO.to_string(),
            ))
        }
    }
}

pub fn get_headers(
    lit: &Literal,
    flow_name: &str,
    interval: Interval,
    headers: &mut jsonwebtoken::Header,
) -> Result<(), ErrorInfo> {
    let map = Literal::get_value::<HashMap<String, Literal>>(
        &lit.primitive,
        flow_name,
        interval,
        "JWT Headers wrong format".to_owned(),
    )?;
    for (key, value) in map.iter() {
        match key.as_ref() {
            "typ" => {
                headers.typ = Some(
                    Literal::get_value::<String>(
                        &value.primitive,
                        flow_name,
                        interval,
                        "JWT Headers 'typ' must be of type String".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            "cty" => {
                headers.cty = Some(
                    Literal::get_value::<String>(
                        &lit.primitive,
                        flow_name,
                        interval,
                        "JWT Headers 'cty' must be of type String".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            "jku" => {
                headers.jku = Some(
                    Literal::get_value::<String>(
                        &lit.primitive,
                        flow_name,
                        interval,
                        "JWT Headers 'jku' must be of type String".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            "kid" => {
                headers.kid = Some(
                    Literal::get_value::<String>(
                        &lit.primitive,
                        flow_name,
                        interval,
                        "JWT Headers 'kid' must be of type String".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            "x5u" => {
                headers.x5u = Some(
                    Literal::get_value::<String>(
                        &lit.primitive,
                        flow_name,
                        interval,
                        "JWT Headers 'x5u' must be of type String".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            "x5t" => {
                headers.x5t = Some(
                    Literal::get_value::<String>(
                        &lit.primitive,
                        flow_name,
                        interval,
                        "JWT Headers 'x5t' must be of type String".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            _ => {}
        }
    }

    Ok(())
}

pub fn get_validation(
    lit: &Literal,
    flow_name: &str,
    interval: Interval,
    validation: &mut jsonwebtoken::Validation,
) -> Result<(), ErrorInfo> {
    let map = Literal::get_value::<HashMap<String, Literal>>(
        &lit.primitive,
        flow_name,
        interval,
        "JWT Headers wrong format".to_owned(),
    )?;
    for (key, value) in map.iter() {
        match key.as_ref() {
            "leeway" => {
                validation.leeway = Literal::get_value::<u64>(
                    &value.primitive,
                    flow_name,
                    interval,
                    "JWT Validation 'leeway' must be of type Int".to_owned(),
                )?
                .to_owned()
            }
            "validate_exp" => {
                validation.validate_exp = Literal::get_value::<bool>(
                    &value.primitive,
                    flow_name,
                    interval,
                    "JWT Validation 'validate_exp' must be of type Boolean".to_owned(),
                )?
                .to_owned()
            }
            "validate_nbf" => {
                validation.validate_nbf = Literal::get_value::<bool>(
                    &value.primitive,
                    flow_name,
                    interval,
                    "JWT Validation 'validate_nbf' must be of type Boolean".to_owned(),
                )?
                .to_owned()
            }
            "aud" => {
                let vec = Literal::get_value::<Vec<String>>(
                    &value.primitive,
                    flow_name,
                    interval,
                    "JWT Validation 'aud' must be of type Boolean".to_owned(),
                )?;

                validation.aud = Some(HashSet::from_iter(vec.iter().cloned()));
            }
            "iss" => {
                validation.iss = Some(
                    Literal::get_value::<String>(
                        &value.primitive,
                        flow_name,
                        interval,
                        "JWT Validation 'validate_nbf' must be of type Boolean".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            "sub" => {
                validation.sub = Some(
                    Literal::get_value::<String>(
                        &value.primitive,
                        flow_name,
                        interval,
                        "JWT Validation 'validate_nbf' must be of type Boolean".to_owned(),
                    )?
                    .to_owned(),
                )
            }
            _ => {}
        }
    }

    Ok(())
}
