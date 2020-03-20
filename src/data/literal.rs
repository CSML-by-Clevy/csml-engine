use crate::data::primitive::Primitive;
use crate::data::Interval;
use crate::error_format::ErrorInfo;

use std::cmp::Ordering;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Literal {
    pub content_type: String,
    pub primitive: Box<dyn Primitive>,
    pub interval: Interval,
}

#[derive(Debug)]
pub enum ContentType {
    Event(String),
    Http,
    Generics,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Literal {
    pub fn get_value<'lifetime, T: 'static>(
        primitive: &'lifetime Box<dyn Primitive>,
    ) -> Result<&'lifetime T, ErrorInfo> {
        if let Some(value) = primitive.get_value().downcast_ref::<T>() {
            Ok(value)
        } else {
            Err(ErrorInfo {
                message: "error in  get_value".to_owned(),
                interval: Interval { column: 0, line: 0 },
            })
        }
    }

    pub fn get_mut_value<'lifetime, T: 'static>(
        primitive: &'lifetime mut Box<dyn Primitive>,
    ) -> Result<&'lifetime mut T, ErrorInfo> {
        if let Some(value) = primitive.get_mut_value().downcast_mut::<T>() {
            Ok(value)
        } else {
            Err(ErrorInfo {
                message: "error in get_mut_value".to_owned(),
                interval: Interval { column: 0, line: 0 },
            })
        }
    }

    pub fn set_content_type(&mut self, content_type: &str) {
        self.content_type = content_type.to_owned();
    }
}

impl ContentType {
    pub fn get(literal: &Literal) -> ContentType {
        match literal.content_type.as_ref() {
            "http" => ContentType::Http,
            "event" => ContentType::Event(String::from("")),
            _ => ContentType::Generics,
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
