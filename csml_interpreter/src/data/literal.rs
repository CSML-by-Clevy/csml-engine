use crate::data::position::Position;
use crate::data::primitive::Primitive;
use crate::data::Interval;
use crate::error_format::*;

use std::cmp::Ordering;
use std::ops::Add;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literal {
    pub content_type: String,
    pub primitive: Box<dyn Primitive>,
    pub interval: Interval,
}

#[derive(Debug)]
pub enum ContentType {
    Event(String),
    Http,
    Primitive,
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Literal {
    pub fn get_value<'lifetime, T: 'static>(
        primitive: &'lifetime Box<dyn Primitive>,
        interval: Interval,
        error_message: String,
    ) -> Result<&'lifetime T, ErrorInfo> {
        match primitive.get_value().downcast_ref::<T>() {
            Some(sep) => Ok(sep),
            None => Err(gen_error_info(Position::new(interval), error_message)),
        }
    }

    pub fn get_mut_value<'lifetime, T: 'static>(
        primitive: &'lifetime mut Box<dyn Primitive>,
        interval: Interval,
        error_message: String,
    ) -> Result<&'lifetime mut T, ErrorInfo> {
        match primitive.get_mut_value().downcast_mut::<T>() {
            Some(sep) => Ok(sep),
            None => Err(gen_error_info(Position::new(interval), error_message)),
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
