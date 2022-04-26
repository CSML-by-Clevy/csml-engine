use crate::data::position::Position;
use crate::data::primitive::{Primitive, PrimitiveObject, PrimitiveString};
use crate::data::{Data, Interval};
use crate::error_format::*;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Add;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literal {
    pub content_type: String,
    pub primitive: Box<dyn Primitive>,
    // this adds complementary information about the origin of the variable
    pub additional_info: Option<HashMap<String, Literal>>,
    pub secure_variable: bool,
    pub interval: Interval,
}

#[derive(Debug)]
pub enum ContentType {
    Event(String),
    Http,
    Smtp,
    Base64,
    Hex,
    Jwt,
    Crypto,
    Time,
    Primitive,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn get_info(
    args: &HashMap<String, Literal>,
    additional_info: &Option<HashMap<String, Literal>>,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    let usage = "get_info(Optional<String: search_key>) => Literal";

    match (additional_info, args.get("arg0")) {
        (Some(map), None) => Ok(PrimitiveObject::get_literal(map, interval)),

        (Some(map), Some(key)) => {
            let key = Literal::get_value::<String>(
                &key.primitive,
                &data.context.flow,
                interval,
                usage.to_owned(),
            )?;

            match map.get(key) {
                Some(value) => Ok(value.to_owned()),
                None => {
                    let mut lit = PrimitiveObject::get_literal(map, interval);
                    let error_msg = format!("get_info() failed, key '{}' not found", key);

                    // add error message in additional info
                    lit.add_error_to_info(&error_msg);

                    Ok(lit)
                }
            }
        }

        _ => Ok(PrimitiveString::get_literal("Null", interval)),
    }
}

pub fn create_error_info(error_msg: &str, interval: Interval) -> HashMap<String, Literal> {
    let mut map = HashMap::new();

    map.insert(
        "error".to_owned(),
        PrimitiveString::get_literal(error_msg, interval),
    );

    map
}

////////////////////////////////////////////////////////////////////////////////
// Implementations
////////////////////////////////////////////////////////////////////////////////

impl Literal {
    pub fn get_value<'lifetime, 'a, T: 'static>(
        primitive: &'lifetime Box<dyn Primitive>,
        flow_name: &'a str,
        interval: Interval,
        error_message: String,
    ) -> Result<&'lifetime T, ErrorInfo> {
        match primitive.get_value().downcast_ref::<T>() {
            Some(sep) => Ok(sep),
            None => Err(gen_error_info(
                Position::new(interval, flow_name),
                error_message,
            )),
        }
    }

    pub fn get_mut_value<'lifetime, 'a, T: 'static>(
        primitive: &'lifetime mut Box<dyn Primitive>,
        flow_name: &'a str,
        interval: Interval,
        error_message: String,
    ) -> Result<&'lifetime mut T, ErrorInfo> {
        match primitive.get_mut_value().downcast_mut::<T>() {
            Some(sep) => Ok(sep),
            None => Err(gen_error_info(
                Position::new(interval, flow_name),
                error_message,
            )),
        }
    }

    pub fn set_content_type(&mut self, content_type: &str) {
        self.content_type = content_type.to_owned();
    }

    pub fn add_info(&mut self, key: &str, value: Literal) {
        match self.additional_info {
            Some(ref mut map) => {
                map.insert(key.to_owned(), value);
            }
            None => {
                let mut info = HashMap::new();
                info.insert(key.to_owned(), value);

                self.additional_info = Some(info);
            }
        }
    }

    pub fn add_info_block(&mut self, info: HashMap<String, Literal>) {
        match self.additional_info {
            Some(ref mut map) => {
                for (key, value) in info {
                    map.insert(key, value);
                }
            }
            None => {
                self.additional_info = Some(info);
            }
        }
    }

    pub fn add_error_to_info(&mut self, error_msg: &str) {
        match self.additional_info {
            Some(ref mut map) => {
                map.insert(
                    "error".to_owned(),
                    PrimitiveString::get_literal(error_msg, self.interval),
                );
            }
            None => {
                let error_info = create_error_info(error_msg, self.interval);
                self.additional_info = Some(error_info);
            }
        }
    }

    pub fn add_literal_to_info(&mut self, key: String, lit: Literal) {
        match self.additional_info {
            Some(ref mut map) => {
                map.insert(key, lit);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(key, lit);

                self.additional_info = Some(map);
            }
        }
    }
}

impl ContentType {
    pub fn get(literal: &Literal) -> ContentType {
        match literal.content_type.as_ref() {
            "http" => ContentType::Http,
            "smtp" => ContentType::Smtp,
            "base64" => ContentType::Base64,
            "hex" => ContentType::Hex,
            "jwt" => ContentType::Jwt,
            "crypto" => ContentType::Crypto,
            "time" => ContentType::Time,
            "event" => ContentType::Event(String::from("")),
            _ => ContentType::Primitive,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.primitive.partial_cmp(&other.primitive)
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        (*self).primitive.is_eq(&(*other.primitive))
    }
}

impl Add for Literal {
    type Output = Result<std::boxed::Box<(dyn Primitive + 'static)>, String>;

    fn add(self, rhs: Literal) -> Result<std::boxed::Box<(dyn Primitive + 'static)>, String> {
        self.primitive + rhs.primitive
    }
}
