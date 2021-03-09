use crate::data::{position::Position,};
use std::collections::{HashMap, HashSet};

use crate::data::{ast::Interval, Literal};
use crate::error_format::*;

use std::iter::FromIterator;
use std::str::FromStr;

pub fn get_algorithm(lit: &Literal, interval: Interval) -> Result<jsonwebtoken::Algorithm, ErrorInfo> {
    let algo =  Literal::get_value::<String>(
        &lit.primitive,
        interval,
        "ERROR_JWT_ALGO".to_owned(),
    )?;

    match jsonwebtoken::Algorithm::from_str(algo) {
        Ok(algo) => Ok(algo),
        _ => return Err(gen_error_info(
            Position::new(interval),
            "ERROR_JWT_ALGO".to_string(),
        ))
    }
}

pub fn get_headers(lit: &Literal, interval: Interval, headers: &mut jsonwebtoken::Header) -> Result<(), ErrorInfo> {
    let map = Literal::get_value::<HashMap<String, Literal>>(
        &lit.primitive,
        interval,
        "JWT Headers wrong format".to_owned(),
    )?;
    for (key,value) in map.iter() {
        match key.as_ref() {
            "typ" => headers.typ = Some(Literal::get_value::<String>(
                &value.primitive,
                interval,
                "JWT Headers 'typ' must be of type String".to_owned(),
            )?.to_owned()),
            "cty" => headers.cty = Some(Literal::get_value::<String>(
                &lit.primitive,
                interval,
                "JWT Headers 'cty' must be of type String".to_owned(),
            )?.to_owned()),
            "jku" => headers.jku = Some(Literal::get_value::<String>(
                &lit.primitive,
                interval,
                "JWT Headers 'jku' must be of type String".to_owned(),
            )?.to_owned()),
            "kid" => headers.kid = Some(Literal::get_value::<String>(
                &lit.primitive,
                interval,
                "JWT Headers 'kid' must be of type String".to_owned(),
            )?.to_owned()),
            "x5u" => headers.x5u = Some(Literal::get_value::<String>(
                &lit.primitive,
                interval,
                "JWT Headers 'x5u' must be of type String".to_owned(),
            )?.to_owned()),
            "x5t" => headers.x5t = Some(Literal::get_value::<String>(
                &lit.primitive,
                interval,
                "JWT Headers 'x5t' must be of type String".to_owned(),
            )?.to_owned()),
            _ => {}
        }
    }

    Ok(())
}

pub fn get_validation(lit: &Literal, interval: Interval, validation: &mut jsonwebtoken::Validation) -> Result<(), ErrorInfo> {
    let map = Literal::get_value::<HashMap<String, Literal>>(
        &lit.primitive,
        interval,
        "JWT Headers wrong format".to_owned(),
    )?;
    for (key,value) in map.iter() {
        match key.as_ref() {
            "leeway" => validation.leeway = Literal::get_value::<u64>(
                &value.primitive,
                interval,
                "JWT Validation 'leeway' must be of type Int".to_owned(),
            )?.to_owned(),
            "validate_exp" => validation.validate_exp = Literal::get_value::<bool>(
                &value.primitive,
                interval,
                "JWT Validation 'validate_exp' must be of type Boolean".to_owned(),
            )?.to_owned(),
            "validate_nbf" => validation.validate_nbf = Literal::get_value::<bool>(
                &value.primitive,
                interval,
                "JWT Validation 'validate_nbf' must be of type Boolean".to_owned(),
            )?.to_owned(),
            "aud" => {
                let vec = Literal::get_value::<Vec<String>>(
                    &value.primitive,
                    interval,
                    "JWT Validation 'aud' must be of type Boolean".to_owned(),
                )?;

                validation.aud = Some(HashSet::from_iter(vec.iter().cloned()));
            },
            "iss" => validation.iss = Some(Literal::get_value::<String>(
                &value.primitive,
                interval,
                "JWT Validation 'validate_nbf' must be of type Boolean".to_owned(),
            )?.to_owned()),
            "sub" => validation.sub = Some(Literal::get_value::<String>(
                &value.primitive,
                interval,
                "JWT Validation 'validate_nbf' must be of type Boolean".to_owned(),
            )?.to_owned()),
            _ => {}
        }
    }

    Ok(())
}
