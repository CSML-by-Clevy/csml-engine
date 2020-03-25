use crate::data::literal::ContentType;
use crate::data::primitive::array::PrimitiveArray;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::int::PrimitiveInt;
use crate::data::primitive::null::PrimitiveNull;
use crate::data::primitive::string::PrimitiveString;
use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{ast::Interval, message::Message, Literal};
use crate::error_format::ErrorInfo;
use crate::interpreter::builtins::http::http_request;
use lazy_static::*;
use std::cmp::Ordering;
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    static ref FUNCTIONS_HTTP: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("set", (PrimitiveObject::set as PrimitiveMethod, Right::Read));
        map.insert("query", (PrimitiveObject::query as PrimitiveMethod, Right::Read));
        map.insert("get", (PrimitiveObject::get_http as PrimitiveMethod, Right::Read));
        map.insert("post", (PrimitiveObject::post as PrimitiveMethod, Right::Read));
        map.insert("put", (PrimitiveObject::put as PrimitiveMethod, Right::Read));
        map.insert("delete", (PrimitiveObject::delete as PrimitiveMethod, Right::Read));
        map.insert("patch", (PrimitiveObject::patch as PrimitiveMethod, Right::Read));
        map.insert("send", (PrimitiveObject::send as PrimitiveMethod, Right::Read));

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_EVENT: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("get_type", (PrimitiveObject::get_type as PrimitiveMethod, Right::Read));
        map.insert("get_metadata", (PrimitiveObject::get_metadata as PrimitiveMethod, Right::Read));
        map.insert("is_number", (PrimitiveObject::is_number_event as PrimitiveMethod, Right::Read));

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_READ: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("type_of", (PrimitiveObject::type_of as PrimitiveMethod, Right::Read));
        map.insert("to_string", (PrimitiveObject::to_string as PrimitiveMethod, Right::Read));
        map.insert("is_number", (PrimitiveObject::is_number_generics as PrimitiveMethod, Right::Read));
        
        map.insert("contains", (PrimitiveObject::contains as PrimitiveMethod, Right::Read));
        map.insert("is_empty", (PrimitiveObject::is_empty as PrimitiveMethod, Right::Read));
        map.insert("length", (PrimitiveObject::length as PrimitiveMethod, Right::Read));
        map.insert("keys", (PrimitiveObject::keys as PrimitiveMethod, Right::Read));
        map.insert("values", (PrimitiveObject::values as PrimitiveMethod, Right::Read));
        map.insert("get", (PrimitiveObject::get_generics as PrimitiveMethod, Right::Read));

        map
    };
}

lazy_static! {
    static ref FUNCTIONS_WRITE: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("clear_values", (PrimitiveObject::clear_values as PrimitiveMethod, Right::Write));
        map.insert("insert", (PrimitiveObject::insert as PrimitiveMethod, Right::Write));
        map.insert("remove", (PrimitiveObject::remove as PrimitiveMethod, Right::Write));

        map
    };
}

type PrimitiveMethod = fn(
    object: &mut PrimitiveObject,
    args: &[Literal],
    interval: Interval,
    content_type: &str,
) -> Result<Literal, ErrorInfo>;

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveObject {
    pub value: HashMap<String, Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveObject {
    fn set(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "set(header: object) => http object";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let literal = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        let mut object = object.to_owned();

        match Literal::get_value::<HashMap<String, Literal>>(&literal.primitive) {
            Ok(header) => {
                insert_to_object(header, &mut object, "header", literal);

                let mut result = PrimitiveObject::get_literal(&object.value, interval);

                result.set_content_type("http");

                Ok(result)
            }
            Err(_) => Err(ErrorInfo::new(
                "error: lhs must be of type 'object'".to_owned(),
                interval,
            )),
        }
    }

    fn query(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "query(parameters: object) => http object";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let literal = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        let mut object = object.to_owned();

        match Literal::get_value::<HashMap<String, Literal>>(&literal.primitive) {
            Ok(header) => {
                insert_to_object(header, &mut object, "query", literal);

                let mut result = PrimitiveObject::get_literal(&object.value, interval);

                result.set_content_type("http");

                Ok(result)
            }
            Err(_) => Err(ErrorInfo::new(
                "error: lhs must be of type 'object'".to_owned(),
                interval,
            )),
        }
    }

    fn get_http(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get() => http object";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("get", interval),
        );

        let mut result = PrimitiveObject::get_literal(&object.value, interval);

        result.set_content_type("http");

        Ok(result)
    }

    fn post(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "post(body: object) => http object";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let literal = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("post", interval),
        );

        match Literal::get_value::<HashMap<String, Literal>>(&literal.primitive) {
            Ok(header) => {
                insert_to_object(header, &mut object, "body", literal);

                let mut result = PrimitiveObject::get_literal(&object.value, interval);

                result.set_content_type("http");

                Ok(result)
            }
            Err(_) => Err(ErrorInfo::new(
                "error: lhs must be of type 'object'".to_owned(),
                interval,
            )),
        }
    }

    fn put(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "put(body: object) => http object";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let literal = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("put", interval),
        );

        match Literal::get_value::<HashMap<String, Literal>>(&literal.primitive) {
            Ok(header) => {
                insert_to_object(header, &mut object, "body", literal);

                let mut result = PrimitiveObject::get_literal(&object.value, interval);

                result.set_content_type("http");

                Ok(result)
            }
            Err(_) => Err(ErrorInfo::new(
                "error: lhs must be of type 'object'".to_owned(),
                interval,
            )),
        }
    }

    fn delete(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "delete() => http object";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

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
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "patch(body: object) => http object";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let literal = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        let mut object = object.to_owned();

        object.value.insert(
            "method".to_owned(),
            PrimitiveString::get_literal("patch", interval),
        );

        match Literal::get_value::<HashMap<String, Literal>>(&literal.primitive) {
            Ok(header) => {
                insert_to_object(header, &mut object, "body", literal);

                let mut result = PrimitiveObject::get_literal(&object.value, interval);

                result.set_content_type("http");

                Ok(result)
            }
            Err(_) => Err(ErrorInfo::new(
                "error: lhs must be of type 'object'".to_owned(),
                interval,
            )),
        }
    }

    fn send(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "send() => http object";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        if let Some(literal) = object.value.get("method") {
            let method = Literal::get_value::<String>(&literal.primitive)?;

            let function = match method.as_ref() {
                "delete" => ureq::delete,
                "put" => ureq::put,
                "patch" => ureq::patch,
                "post" => ureq::post,
                "get" => ureq::get,
                _ => {
                    return Err(ErrorInfo::new(
                        format!("error: unknown http request {}", method),
                        interval,
                    ));
                }
            };

            return http_request(&object.value, function, interval);
        }

        Err(ErrorInfo::new(
            "error: lhs must be of type 'object' and contains the key 'method'".to_owned(),
            interval,
        ))
    }
}

impl PrimitiveObject {
    fn get_type(
        _object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get_type() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal(content_type, interval))
    }

    fn get_metadata(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get_metadata() => object";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(Literal {
            content_type: content_type.to_owned(),
            primitive: Box::new(object.clone()),
            interval,
        })
    }

    fn is_number_event(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        if let Some(res) = object.value.get("text") {
            let result = res.primitive.to_string();
            let result = result.parse::<f64>().is_ok();

            return Ok(PrimitiveBoolean::get_literal(result, interval));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }
}

impl PrimitiveObject {
    fn is_number_generics(
        _object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }
    fn type_of(
        _object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal("object", interval))
    }

    fn to_string(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal(&object.to_string(), interval))
    }

    fn contains(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "contains(key: string) => boolean";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let key = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: key must be of type 'string'".to_owned(),
                    interval,
                ));
            }
        };

        let result = object.value.contains_key(key);

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn is_empty(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_empty() => boolean";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = object.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn length(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "length() => int";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = object.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn keys(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "keys() => array";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let mut result = Vec::new();

        for key in object.value.keys() {
            result.push(PrimitiveString::get_literal(key, interval));
        }

        Ok(PrimitiveArray::get_literal(&result, interval))
    }

    fn values(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "values() => array";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let mut result = Vec::new();

        for value in object.value.values() {
            result.push(value.to_owned());
        }

        Ok(PrimitiveArray::get_literal(&result, interval))
    }

    fn get_generics(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "get(key: string) => primitive";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let key = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: key must be of type 'string'".to_owned(),
                    interval,
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
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "clear_values() => null";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
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
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "insert(key: string, value: primitive) => null";

        if args.len() != 2 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let key = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: key must be of type 'string'".to_owned(),
                    interval,
                ));
            }
        };

        let value = match args.get(1) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        object.value.insert(key.to_owned(), value.to_owned());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn remove(
        object: &mut PrimitiveObject,
        args: &[Literal],
        interval: Interval,
        _content_type: &str,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "remove(key: string) => primitive";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let key = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: key must be of type 'string'".to_owned(),
                    interval,
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
    key: &str,
    literal: &Literal,
) {
    dst.value
        .entry(key.to_owned())
        .and_modify(|tmp: &mut Literal| {
            if let Ok(tmp) = Literal::get_mut_value::<HashMap<String, Literal>>(&mut tmp.primitive)
            {
                for (key, value) in src.iter() {
                    tmp.insert(key.to_owned(), value.to_owned());
                }
            } else {
                unreachable!();
            }
        })
        .or_insert(literal.to_owned());
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

impl Primitive for PrimitiveObject {
    fn do_exec(
        &mut self,
        name: &str,
        args: &[Literal],
        interval: Interval,
        content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        let event = vec![FUNCTIONS_EVENT.clone(), FUNCTIONS_READ.clone()];
        let http = vec![
            FUNCTIONS_HTTP.clone(),
            FUNCTIONS_READ.clone(),
            FUNCTIONS_WRITE.clone(),
        ];
        let generics = vec![FUNCTIONS_READ.clone(), FUNCTIONS_WRITE.clone()];

        let (content_type, vector) = match content_type {
            ContentType::Event(event_type) => (event_type.as_ref(), event),
            ContentType::Http => ("", http),
            ContentType::Generics => ("", generics),
        };

        for function in vector.iter() {
            if let Some((f, right)) = function.get(name) {
                let result = f(self, args, interval, &content_type)?;

                return Ok((result, *right));
            }
        }

        Err(ErrorInfo {
            message: format!("unknown method '{}' for type Object", name),
            interval,
        })
    }

    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value == other.value;
        }

        false
    }

    fn is_cmp(&self, _other: &dyn Primitive) -> Option<Ordering> {
        None
    }

    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: format!(
                "error: illegal operation: {:?} + {:?}",
                self.get_type(),
                other.get_type()
            ),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: format!(
                "error: illegal operation: {:?} - {:?}",
                self.get_type(),
                other.get_type()
            ),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: format!(
                "error: illegal operation: {:?} / {:?}",
                self.get_type(),
                other.get_type()
            ),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: format!(
                "error: illegal operation: {:?} * {:?}",
                self.get_type(),
                other.get_type()
            ),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: format!(
                "error: illegal operation: {:?} % {:?}",
                self.get_type(),
                other.get_type()
            ),
            interval: Interval { column: 0, line: 0 },
        })
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
            object.insert(key.to_owned(), literal.primitive.to_json());
        }

        serde_json::Value::Object(object)
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
}
