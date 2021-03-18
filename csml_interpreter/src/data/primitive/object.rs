use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::{
    ast::Interval,
    literal::ContentType,
    message::Message,
    primitive::{
        tools_jwt, Primitive, PrimitiveArray, PrimitiveBoolean, PrimitiveInt, PrimitiveNull,
        PrimitiveString, PrimitiveType, Right,
    },
    tokens::TYPES,
    Literal,
};
use crate::error_format::*;
use crate::interpreter::{
    builtins::http::http_request, json_to_rust::json_to_literal,
    variable_handler::match_literals::match_obj,
};
use lazy_static::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref FUNCTIONS_HTTP: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "set",
            (PrimitiveObject::set as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "query",
            (PrimitiveObject::query as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "get",
            (PrimitiveObject::get_http as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "post",
            (PrimitiveObject::post as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "put",
            (PrimitiveObject::put as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "delete",
            (PrimitiveObject::delete as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "patch",
            (PrimitiveObject::patch as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "send",
            (PrimitiveObject::send as PrimitiveMethod, Right::Read),
        );

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_JWT: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "sign",
            (PrimitiveObject::jwt_sign as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "decode",
            (PrimitiveObject::jwt_decode as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "verify",
            (PrimitiveObject::jwt_verity as PrimitiveMethod, Right::Read),
        );

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_BASE64: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "encode",
            (
                PrimitiveObject::base64_encode as PrimitiveMethod,
                Right::Read,
            ),
        );
        map.insert(
            "decode",
            (
                PrimitiveObject::base64_decode as PrimitiveMethod,
                Right::Read,
            ),
        );
        map
    };
}

lazy_static! {
    static ref FUNCTIONS_HEX: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "encode",
            (PrimitiveObject::hex_encode as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "decode",
            (PrimitiveObject::hex_decode as PrimitiveMethod, Right::Read),
        );
        map
    };
}

lazy_static! {
    static ref FUNCTIONS_EVENT: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "get_type",
            (PrimitiveObject::get_type as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "get_content",
            (PrimitiveObject::get_content as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_email",
            (PrimitiveObject::is_email as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "match",
            (PrimitiveObject::match_args as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "match_array",
            (PrimitiveObject::match_array as PrimitiveMethod, Right::Read),
        );

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_READ: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "type_of",
            (PrimitiveObject::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveObject::to_string as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_number",
            (PrimitiveObject::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_int",
            (PrimitiveObject::is_int as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_float",
            (PrimitiveObject::is_float as PrimitiveMethod, Right::Read),
        );

        map.insert(
            "contains",
            (PrimitiveObject::contains as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_empty",
            (PrimitiveObject::is_empty as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "length",
            (PrimitiveObject::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "keys",
            (PrimitiveObject::keys as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "values",
            (PrimitiveObject::values as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "get",
            (
                PrimitiveObject::get_generics as PrimitiveMethod,
                Right::Read,
            ),
        );

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_WRITE: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "clear_values",
            (
                PrimitiveObject::clear_values as PrimitiveMethod,
                Right::Write,
            ),
        );
        map.insert(
            "insert",
            (PrimitiveObject::insert as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "remove",
            (PrimitiveObject::remove as PrimitiveMethod, Right::Write),
        );

        map
    };
}

type PrimitiveMethod = fn(
    object: &mut PrimitiveObject,
    args: &HashMap<String, Literal>,
    interval: Interval,
    content_type: &str,
) -> Result<Literal, ErrorInfo>;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveObject {
    pub value: HashMap<String, Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveObject {
    fn set(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "set(header: object) => http object";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let literal = match args.get("arg0") {
            Some(res) => res,
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ));
            }
        };

        let mut object = object.to_owned();

        let header = Literal::get_value::<HashMap<String, Literal>>(
            &literal.primitive,
            interval,
            ERROR_HTTP_SET.to_owned(),
        )?;

        insert_to_object(header, &mut object, "header", literal);

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn query(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "query(parameters: object) => http object";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let literal = match args.get("arg0") {
            Some(res) => res,
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ));
            }
        };

        let mut object = object.to_owned();

        let header = Literal::get_value::<HashMap<String, Literal>>(
            &literal.primitive,
            interval,
            ERROR_HTTP_QUERY.to_owned(),
        )?;
        insert_to_object(header, &mut object, "query", literal);

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn get_http(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get() => http object";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("get", interval),
        );

        object.value.remove("body");

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn post(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        match args.get("arg0") {
            Some(body) => object.value.insert("body".to_owned(), body.to_owned()),
            _ => object.value.remove("body"),
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("post", interval),
        );

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn put(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        match args.get("arg0") {
            Some(body) => object.value.insert("body".to_owned(), body.to_owned()),
            _ => object.value.remove("body"),
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("put", interval),
        );

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn delete(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        match args.get("arg0") {
            Some(body) => object.value.insert("body".to_owned(), body.to_owned()),
            _ => object.value.remove("body"),
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("delete", interval),
        );

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn patch(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let body = match args.get("arg0") {
            Some(res) => res.to_owned(),
            _ => PrimitiveNull::get_literal(Interval::default()),
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("patch", interval),
        );

        object.value.insert("body".to_owned(), body);

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn send(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "send() => http object";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        if let Some(literal) = object.value.get("method") {
            let function = match Literal::get_value::<String>(
                &literal.primitive,
                interval,
                ERROR_HTTP_UNKNOWN_METHOD.to_string(),
            ) {
                Ok(delete) if delete == "delete" => ureq::delete,
                Ok(put) if put == "put" => ureq::put,
                Ok(patch) if patch == "patch" => ureq::patch,
                Ok(post) if post == "post" => ureq::post,
                Ok(get) if get == "get" => ureq::get,
                _ => {
                    return Err(gen_error_info(
                        Position::new(interval),
                        ERROR_HTTP_UNKNOWN_METHOD.to_string(),
                    ))
                }
            };

            let value = http_request(&object.value, function, interval)?;
            return json_to_literal(&value, interval);
        }

        Err(gen_error_info(
            Position::new(interval),
            ERROR_HTTP_SEND.to_owned(),
        ))
    }
}

impl PrimitiveObject {
    fn jwt_sign(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let mut headers = jsonwebtoken::Header::default();

        match args.get("arg0") {
            Some(algo) if algo.primitive.get_type() == PrimitiveType::PrimitiveString => {
                headers.alg = tools_jwt::get_algorithm(algo, interval)?;
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_SIGN_ALGO.to_string(),
                ))
            }
        }

        let claims = match object.value.get("jwt") {
            Some(literal) => literal.primitive.to_json(),
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_SIGN_CLAIMS.to_string(),
                ))
            }
        };

        let key = match args.get("arg1") {
            Some(key) if key.primitive.get_type() == PrimitiveType::PrimitiveString => {
                let key = Literal::get_value::<String>(
                    &key.primitive,
                    interval,
                    ERROR_JWT_SIGN_SECRET.to_string(),
                )?;

                jsonwebtoken::EncodingKey::from_secret(key.as_ref())
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_ALGO.to_string(),
                ))
            }
        };

        if let Some(lit) = args.get("arg2") {
            tools_jwt::get_headers(lit, interval, &mut headers)?;
        }

        match jsonwebtoken::encode(&headers, &claims, &key) {
            Ok(value) => Ok(PrimitiveString::get_literal(&value, interval)),
            Err(e) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("Invalid JWT encode {:?}", e.kind()),
                ))
            }
        }
    }

    fn jwt_decode(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let token = match object.value.get("jwt") {
            Some(literal) => Literal::get_value::<String>(
                &literal.primitive,
                interval,
                ERROR_JWT_TOKEN.to_owned(),
            )?,
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_TOKEN.to_string(),
                ))
            }
        };

        let algo = match args.get("arg0") {
            Some(algo) if algo.primitive.get_type() == PrimitiveType::PrimitiveString => {
                tools_jwt::get_algorithm(algo, interval)?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_DECODE_ALGO.to_string(),
                ))
            }
        };

        let key = match args.get("arg1") {
            Some(key) if key.primitive.get_type() == PrimitiveType::PrimitiveString => {
                let key = Literal::get_value::<String>(
                    &key.primitive,
                    interval,
                    ERROR_JWT_DECODE_SECRET.to_owned(),
                )?;

                jsonwebtoken::DecodingKey::from_secret(key.as_ref())
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_DECODE_SECRET.to_string(),
                ))
            }
        };

        match jsonwebtoken::decode::<serde_json::Value>(
            token,
            &key,
            &jsonwebtoken::Validation::new(algo),
        ) {
            Ok(token_message) => tools_jwt::token_data_to_literal(token_message, interval),
            Err(e) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("Invalid JWT decode {:?}", e.kind()),
                ))
            }
        }
    }

    fn jwt_verity(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let mut validation = jsonwebtoken::Validation::default();

        let token = match object.value.get("jwt") {
            Some(literal) => Literal::get_value::<String>(
                &literal.primitive,
                interval,
                ERROR_JWT_TOKEN.to_owned(),
            )?,
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_TOKEN.to_string(),
                ))
            }
        };

        match args.get("arg0") {
            Some(lit) => tools_jwt::get_validation(lit, interval, &mut validation)?,
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_VALIDATION_CLAIMS.to_string(),
                ))
            }
        }

        match args.get("arg1") {
            Some(algo) if algo.primitive.get_type() == PrimitiveType::PrimitiveString => {
                validation.algorithms = vec![tools_jwt::get_algorithm(algo, interval)?];
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_VALIDATION_ALGO.to_string(),
                ))
            }
        };

        let key = match args.get("arg2") {
            Some(key) if key.primitive.get_type() == PrimitiveType::PrimitiveString => {
                let key = Literal::get_value::<String>(
                    &key.primitive,
                    interval,
                    ERROR_JWT_SECRET.to_owned(),
                )?;

                jsonwebtoken::DecodingKey::from_secret(key.as_ref())
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_JWT_VALIDATION_SECRETE.to_string(),
                ))
            }
        };

        match jsonwebtoken::decode::<serde_json::Value>(token, &key, &validation) {
            Ok(token_message) => tools_jwt::token_data_to_literal(token_message, interval),
            Err(e) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("Invalid JWT verify {:?}", e.kind()),
                ))
            }
        }
    }
}

impl PrimitiveObject {
    fn base64_encode(
        object: &mut PrimitiveObject,
        _args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "Base64(\"...\").encode() => String";

        let string = match object.value.get("string") {
            Some(lit) => lit.primitive.to_string(),
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ))
            }
        };

        let result = base64::encode(string.as_bytes());

        Ok(PrimitiveString::get_literal(&result, interval))
    }

    fn base64_decode(
        object: &mut PrimitiveObject,
        _args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "Base64(\"...\").decode() => String";

        let string = match object.value.get("string") {
            Some(lit) => lit.primitive.to_string(),
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ))
            }
        };

        let result = match base64::decode(string.as_bytes()) {
            Ok(buf) => format!("{}", String::from_utf8_lossy(&buf)),
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("Base64 invalid value: {}, can't be decode", string),
                ))
            }
        };

        Ok(PrimitiveString::get_literal(&result, interval))
    }
}

impl PrimitiveObject {
    fn hex_encode(
        object: &mut PrimitiveObject,
        _args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "Hex(\"...\").encode() => String";

        let string = match object.value.get("string") {
            Some(lit) => lit.primitive.to_string(),
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ))
            }
        };

        let result = hex::encode(string.as_bytes());

        Ok(PrimitiveString::get_literal(&result, interval))
    }

    fn hex_decode(
        object: &mut PrimitiveObject,
        _args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "Hex(\"...\").decode() => String";

        let string = match object.value.get("string") {
            Some(lit) => lit.primitive.to_string(),
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ))
            }
        };

        let result = match hex::decode(string.as_bytes()) {
            Ok(buf) => format!("{}", String::from_utf8_lossy(&buf)),
            Err(_) => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("Hex invalid value: {}, can't be decode", string),
                ))
            }
        };

        Ok(PrimitiveString::get_literal(&result, interval))
    }
}

impl PrimitiveObject {
    fn get_type(
        _object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get_type() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(content_type, interval))
    }

    fn get_content(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get_content() => object";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(Literal {
            content_type: content_type.to_owned(),
            primitive: Box::new(object.clone()),
            interval,
        })
    }

    fn is_email(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_email() => boolean";

        let text = match object.value.get("text") {
            Some(lit) if lit.content_type == "string" => lit.primitive.to_string(),
            _ => return Ok(PrimitiveBoolean::get_literal(false, interval)),
        };

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let email_regex = Regex::new(
            r"^([a-zA-Z0-9_+]([a-zA-Z0-9_+.]*[a-zA-Z0-9_+])?)@([a-zA-Z0-9]+([\-\.]{1}[a-zA-Z0-9]+)*\.[a-zA-Z]{2,6})",
        )
        .unwrap();

        let lit = PrimitiveBoolean::get_literal(email_regex.is_match(&text), interval);

        Ok(lit)
    }

    fn match_args(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "match(a) => a";

        let lit = match (object.value.get("text"), object.value.get("payload")) {
            (Some(lit), _) | (_, Some(lit)) if lit.content_type == "string" => lit,
            _ => return Ok(PrimitiveNull::get_literal(interval)),
        };

        if args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let is_match = args.iter().find(|(_name, arg)| match_obj(lit, arg));

        match is_match {
            Some((_, lit)) => Ok(lit.to_owned()),
            None => Ok(PrimitiveNull::get_literal(interval)),
        }
    }

    fn match_array(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "match_array([a,b,c]) => a";

        let lit = match (object.value.get("text"), object.value.get("payload")) {
            (Some(lit), _) | (_, Some(lit)) if lit.content_type == "string" => lit,
            _ => return Ok(PrimitiveNull::get_literal(interval)),
        };

        let array = match args.get("arg0") {
            Some(lit) => Literal::get_value::<Vec<Literal>>(
                &lit.primitive,
                interval,
                format!("expect Array value as argument usage: {}", usage),
            )?,
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("expect Array value as argument usage: {}", usage),
                ))
            }
        };

        let is_match = array.iter().find(|&arg| match_obj(lit, arg));

        match is_match {
            Some(lit) => Ok(lit.to_owned()),
            None => Ok(PrimitiveNull::get_literal(interval)),
        }
    }
}

impl PrimitiveObject {
    fn is_number(
        _object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn is_int(
        _object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_int() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn is_float(
        _object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_float() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn type_of(
        _object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("object", interval))
    }

    fn to_string(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&object.to_string(), interval))
    }

    fn contains(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "contains(key: string) => boolean";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let key = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_OBJECT_CONTAINS.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_OBJECT_CONTAINS.to_owned(),
                ));
            }
        };

        let result = object.value.contains_key(key);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn is_empty(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_empty() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = object.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn length(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "length() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = object.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn keys(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "keys() => array";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut result = Vec::new();

        for key in object.value.keys() {
            result.push(PrimitiveString::get_literal(key, interval));
        }

        Ok(PrimitiveArray::get_literal(&result, interval))
    }

    fn values(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "values() => array";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut result = Vec::new();

        for value in object.value.values() {
            result.push(value.to_owned());
        }

        Ok(PrimitiveArray::get_literal(&result, interval))
    }

    fn get_generics(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get(key: string) => primitive";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let key = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_OBJECT_GET_GENERICS.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_OBJECT_GET_GENERICS.to_owned(),
                ));
            }
        };

        match object.value.get(key) {
            Some(res) => Ok(res.to_owned()),
            None => Ok(PrimitiveNull::get_literal(interval)),
        }
    }
}

impl PrimitiveObject {
    fn clear_values(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "clear_values() => null";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut vector: Vec<String> = Vec::new();

        for key in object.value.keys() {
            vector.push(key.to_owned());
        }

        for key in vector.iter() {
            object
                .value
                .insert(key.to_owned(), PrimitiveNull::get_literal(interval));
        }

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn insert(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "insert(key: string, value: primitive) => null";

        if args.len() != 2 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let key = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_OBJECT_INSERT.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_OBJECT_INSERT.to_owned(),
                ));
            }
        };

        let value = match args.get("arg1") {
            Some(res) => res,
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ));
            }
        };

        object.value.insert(key.to_owned(), value.to_owned());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn remove(
        object: &mut PrimitiveObject,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "remove(key: string) => primitive";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let key = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(
                    &res.primitive,
                    interval,
                    ERROR_OBJECT_REMOVE.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_OBJECT_REMOVE.to_owned(),
                ));
            }
        };

        match object.value.remove(key) {
            Some(value) => Ok(value),
            None => Ok(PrimitiveNull::get_literal(interval)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn insert_to_object(
    src: &HashMap<String, Literal>,
    dst: &mut PrimitiveObject,
    key_name: &str,
    literal: &Literal,
) {
    dst.value
        .entry(key_name.to_owned())
        .and_modify(|tmp: &mut Literal| {
            if let Ok(tmp) = Literal::get_mut_value::<HashMap<String, Literal>>(
                &mut tmp.primitive,
                literal.interval,
                ERROR_UNREACHABLE.to_owned(),
            ) {
                for (key, value) in src.iter() {
                    tmp.insert(key.to_owned(), value.to_owned());
                }
            } else {
                unreachable!();
            }
        })
        .or_insert_with(|| literal.to_owned());
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveObject {
    pub fn new(value: &HashMap<String, Literal>) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn get_literal(object: &HashMap<String, Literal>, interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveObject::new(object));

        Literal {
            content_type: "object".to_owned(),
            primitive,
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[typetag::serde]
impl Primitive for PrimitiveObject {
    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value == other.value;
        }

        false
    }

    fn is_cmp(&self, _other: &dyn Primitive) -> Option<Ordering> {
        None
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} + {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} - {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} / {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} * {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String> {
        Err(format!(
            "{} {:?} % {:?}",
            ERROR_ILLEGAL_OPERATION,
            self.get_type(),
            other.get_type()
        ))
    }

    fn as_debug(&self) -> &dyn std::fmt::Debug {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_type(&self) -> PrimitiveType {
        PrimitiveType::PrimitiveObject
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        let mut object: serde_json::map::Map<String, serde_json::Value> =
            serde_json::map::Map::new();

        for (key, literal) in self.value.iter() {
            if !TYPES.contains(&&(*literal.content_type)) {
                let mut map = serde_json::Map::new();
                map.insert(
                    "content_type".to_owned(),
                    serde_json::json!(literal.content_type),
                );
                map.insert("content".to_owned(), literal.primitive.to_json());

                object.insert(key.to_owned(), serde_json::json!(map));
            } else {
                object.insert(key.to_owned(), literal.primitive.to_json());
            }
        }

        serde_json::Value::Object(object)
    }

    fn format_mem(&self, content_type: &str, first: bool) -> serde_json::Value {
        let mut object: serde_json::map::Map<String, serde_json::Value> =
            serde_json::map::Map::new();

        match (content_type, first) {
            (content_type, false) if content_type == "object" => {
                for (key, literal) in self.value.iter() {
                    let content_type = &literal.content_type;
                    object.insert(
                        key.to_owned(),
                        literal.primitive.format_mem(content_type, false),
                    );
                }

                serde_json::Value::Object(object)
            }
            (content_type, _) => {
                let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                map.insert("_content_type".to_owned(), serde_json::json!(content_type));

                for (key, literal) in self.value.iter() {
                    let content_type = &literal.content_type;
                    object.insert(
                        key.to_owned(),
                        literal.primitive.format_mem(content_type, false),
                    );
                }
                map.insert("_content".to_owned(), serde_json::Value::Object(object));

                serde_json::Value::Object(map)
            }
        }
    }

    fn to_string(&self) -> String {
        self.to_json().to_string()
    }

    fn as_bool(&self) -> bool {
        true
    }

    fn get_value(&self) -> &dyn std::any::Any {
        &self.value
    }

    fn get_mut_value(&mut self) -> &mut dyn std::any::Any {
        &mut self.value
    }

    fn to_msg(&self, content_type: String) -> Message {
        Message {
            content_type,
            content: self.to_json(),
        }
    }

    fn do_exec(
        &mut self,
        name: &str,
        args: &HashMap<String, Literal>,
        interval: Interval,
        content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        let event = vec![FUNCTIONS_EVENT.clone()];
        let http = vec![
            FUNCTIONS_HTTP.clone(),
            FUNCTIONS_READ.clone(),
            FUNCTIONS_WRITE.clone(),
        ];
        let base64 = vec![FUNCTIONS_BASE64.clone()];
        let hex = vec![FUNCTIONS_HEX.clone()];
        let jwt = vec![FUNCTIONS_JWT.clone()];
        let generics = vec![FUNCTIONS_READ.clone(), FUNCTIONS_WRITE.clone()];

        let mut is_event = false;

        let (content_type, vector) = match content_type {
            ContentType::Event(event_type) => {
                is_event = true;

                (event_type.as_ref(), event)
            }
            ContentType::Http => ("", http),
            ContentType::Base64 => ("", base64),
            ContentType::Hex => ("", hex),
            ContentType::Jwt => ("", jwt),
            ContentType::Primitive => ("", generics),
        };

        for function in vector.iter() {
            if let Some((f, right)) = function.get(name) {
                let result = f(self, args, interval, &content_type)?;

                return Ok((result, *right));
            }
        }

        if is_event {
            let vec = ["text", "payload"];
            for value in vec.iter() {
                if let Some(res) = self.value.get_mut(*value) {
                    return res
                        .primitive
                        .do_exec(name, args, interval, &ContentType::Primitive);
                }
            }
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", name, ERROR_OBJECT_UNKNOWN_METHOD),
        ))
    }
}
