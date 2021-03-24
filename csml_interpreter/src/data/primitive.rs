pub mod array;
pub mod boolean;
pub mod float;
pub mod int;
pub mod null;
pub mod object;
pub mod string;
pub mod closure;

pub mod tools;
pub mod tools_jwt;
pub mod tools_crypto;

use crate::data::literal::ContentType;
pub use array::PrimitiveArray;
pub use boolean::PrimitiveBoolean;
pub use float::PrimitiveFloat;
pub use int::PrimitiveInt;
pub use null::PrimitiveNull;
pub use object::PrimitiveObject;
pub use string::PrimitiveString;
pub use closure::PrimitiveClosure;

use crate::data::primitive::tools::*;
use crate::data::{Interval, Literal, Message, MessageData, MSG, Data};
use crate::error_format::*;

use std::cmp::Ordering;
use std::{collections::HashMap, sync::mpsc};
use std::ops::{Add, Div, Mul, Rem, Sub};


////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Copy, Clone)]
pub enum Right {
    Read,
    Write,
}

#[derive(PartialEq, Debug)]
pub enum PrimitiveType {
    PrimitiveArray,
    PrimitiveBoolean,
    PrimitiveFloat,
    PrimitiveInt,
    PrimitiveNull,
    PrimitiveObject,
    PrimitiveString,
    PrimitiveClosure,
}

#[typetag::serde(tag = "primitive")]
pub trait Primitive: Send {
    fn is_eq(&self, other: &dyn Primitive) -> bool;
    fn is_cmp(&self, other: &dyn Primitive) -> Option<Ordering>;
    fn do_add(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String>;
    fn do_sub(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String>;
    fn do_div(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String>;
    fn do_mul(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String>;
    fn do_rem(&self, other: &dyn Primitive) -> Result<Box<dyn Primitive>, String>;

    fn as_debug(&self) -> &dyn std::fmt::Debug;
    fn as_any(&self) -> &dyn std::any::Any;
    fn get_type(&self) -> PrimitiveType;
    fn as_box_clone(&self) -> Box<dyn Primitive>;
    fn to_json(&self) -> serde_json::Value;
    fn format_mem(&self, content_type: &str, first: bool) -> serde_json::Value;
    fn to_string(&self) -> String;
    fn as_bool(&self) -> bool;
    fn get_value(&self) -> &dyn std::any::Any;
    fn get_mut_value(&mut self) -> &mut dyn std::any::Any;
    fn to_msg(&self, content_type: String) -> Message;
    fn do_exec(
        &mut self,
        name: &str,
        args: &HashMap<String, Literal>,
        interval: Interval,
        content_type: &ContentType,
        data: &mut Data,
        msg_data: &mut MessageData,
        sender: &Option<mpsc::Sender<MSG>>,
    ) -> Result<(Literal, Right), ErrorInfo>;
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveType {
    pub fn to_string(&self) -> String {
        match self {
            PrimitiveType::PrimitiveArray => "array".to_owned(),
            PrimitiveType::PrimitiveBoolean => "boolean".to_owned(),
            PrimitiveType::PrimitiveFloat => "float".to_owned(),
            PrimitiveType::PrimitiveInt => "int".to_owned(),
            PrimitiveType::PrimitiveNull => "null".to_owned(),
            PrimitiveType::PrimitiveObject => "object".to_owned(),
            PrimitiveType::PrimitiveString => "string".to_owned(),
            PrimitiveType::PrimitiveClosure => "closure".to_owned(),
        }
    }
}

impl dyn Primitive {
    pub fn exec(
        &mut self,
        name: &str,
        args: &HashMap<String, Literal>,
        interval: Interval,
        content_type: &ContentType,
        mem_update: &mut bool,
        data: &mut Data,
        msg_data: &mut MessageData,
        sender: &Option<mpsc::Sender<MSG>>,
    ) -> Result<Literal, ErrorInfo> {
        *mem_update = false;

        let (res, right) = self.do_exec(name, args, interval, content_type, data, msg_data, sender)?;
        if right == Right::Write {
            *mem_update = true;
        }

        Ok(res)
    }
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for dyn Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{\n\t{:?}\n}}", self.as_debug())
    }
}

impl Clone for Box<dyn Primitive> {
    fn clone(&self) -> Box<dyn Primitive> {
        self.as_box_clone()
    }
}

impl PartialEq for dyn Primitive {
    fn eq(&self, other: &Self) -> bool {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.is_eq(other),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                lhs.value as f64 == rhs.value
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                lhs.value == rhs.value as f64
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => rhs.value == int,
                    Ok(Integer::Float(float)) => (rhs.value as f64) == float,
                    Err(_) => false,
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => rhs.value == (int as f64),
                    Ok(Integer::Float(float)) => rhs.value == float,
                    Err(_) => false,
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.value == int,
                    Ok(Integer::Float(float)) => (lhs.value as f64) == float,
                    Err(_) => false,
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.value == (int as f64),
                    Ok(Integer::Float(float)) => lhs.value == float,
                    Err(_) => false,
                }
            }
            _ => false,
        }
    }
}

impl PartialOrd for dyn Primitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.is_cmp(other),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                let lhs = lhs.value as f64;
                let rhs = rhs.value;

                lhs.partial_cmp(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                let lhs = lhs.value;
                let rhs = rhs.value as f64;

                lhs.partial_cmp(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => int.partial_cmp(&rhs.value),
                    Ok(Integer::Float(float)) => float.partial_cmp(&(rhs.value as f64)),
                    Err(_) => None,
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => (int as f64).partial_cmp(&rhs.value),
                    Ok(Integer::Float(float)) => float.partial_cmp(&rhs.value),
                    Err(_) => None,
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.value.partial_cmp(&(int)),
                    Ok(Integer::Float(float)) => (lhs.value as f64).partial_cmp(&(float)),
                    Err(_) => None,
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.value.partial_cmp(&(int as f64)),
                    Ok(Integer::Float(float)) => lhs.value.partial_cmp(&(float)),
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }
}

impl Add for Box<dyn Primitive> {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Result<Self, String> {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.do_add(&(*other)),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let lhs = PrimitiveFloat::new(lhs.value as f64);

                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                lhs.do_add(rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = PrimitiveFloat::new(rhs.value as f64);

                lhs.do_add(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveInt::new(int).do_add(rhs),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(float).do_add(&PrimitiveFloat::new(rhs.value as f64))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveFloat::new(int as f64).do_add(rhs),
                    Ok(Integer::Float(float)) => PrimitiveFloat::new(float).do_add(rhs),
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_add(&PrimitiveInt::new(int)),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(lhs.value as f64).do_add(&PrimitiveFloat::new(float))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_add(&PrimitiveFloat::new(int as f64)),
                    Ok(Integer::Float(float)) => lhs.do_add(&PrimitiveFloat::new(float)),
                    Err(err) => Err(err),
                }
            }

            _ => Err(format!(
                "{} {:?} + {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }
}

impl Sub for Box<dyn Primitive> {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Result<Self, String> {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.do_sub(&(*other)),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let lhs = PrimitiveFloat::new(lhs.value as f64);

                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                lhs.do_sub(rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = PrimitiveFloat::new(rhs.value as f64);

                lhs.do_sub(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveInt::new(int).do_sub(rhs),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(float).do_sub(&PrimitiveFloat::new(rhs.value as f64))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveFloat::new(int as f64).do_sub(rhs),
                    Ok(Integer::Float(float)) => PrimitiveFloat::new(float).do_sub(rhs),
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_sub(&PrimitiveInt::new(int)),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(lhs.value as f64).do_sub(&PrimitiveFloat::new(float))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_sub(&PrimitiveFloat::new(int as f64)),
                    Ok(Integer::Float(float)) => lhs.do_sub(&PrimitiveFloat::new(float)),
                    Err(err) => Err(err),
                }
            }
            _ => Err(format!(
                "{} {:?} - {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }
}

impl Div for Box<dyn Primitive> {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Result<Self, String> {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.do_div(&(*other)),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let lhs = PrimitiveFloat::new(lhs.value as f64);

                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                lhs.do_div(rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = PrimitiveFloat::new(rhs.value as f64);

                lhs.do_div(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveInt::new(int).do_div(rhs),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(float).do_div(&PrimitiveFloat::new(rhs.value as f64))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveFloat::new(int as f64).do_div(rhs),
                    Ok(Integer::Float(float)) => PrimitiveFloat::new(float).do_div(rhs),
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_div(&PrimitiveInt::new(int)),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(lhs.value as f64).do_div(&PrimitiveFloat::new(float))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_div(&PrimitiveFloat::new(int as f64)),
                    Ok(Integer::Float(float)) => lhs.do_div(&PrimitiveFloat::new(float)),
                    Err(err) => Err(err),
                }
            }
            _ => Err(format!(
                "{} {:?} / {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }
}

impl Mul for Box<dyn Primitive> {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Result<Self, String> {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.do_mul(&(*other)),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let lhs = PrimitiveFloat::new(lhs.value as f64);

                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                lhs.do_mul(rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = PrimitiveFloat::new(rhs.value as f64);

                lhs.do_mul(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveInt::new(int).do_mul(rhs),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(float).do_mul(&PrimitiveFloat::new(rhs.value as f64))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveFloat::new(int as f64).do_mul(rhs),
                    Ok(Integer::Float(float)) => PrimitiveFloat::new(float).do_mul(rhs),
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_mul(&PrimitiveInt::new(int)),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(lhs.value as f64).do_mul(&PrimitiveFloat::new(float))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_mul(&PrimitiveFloat::new(int as f64)),
                    Ok(Integer::Float(float)) => lhs.do_mul(&PrimitiveFloat::new(float)),
                    Err(err) => Err(err),
                }
            }
            _ => Err(format!(
                "{} {:?} * {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }
}

impl Rem for Box<dyn Primitive> {
    type Output = Result<Self, String>;

    fn rem(self, other: Self) -> Result<Self, String> {
        match (self.get_type(), other.get_type()) {
            (lhs, rhs) if lhs == rhs => self.do_rem(&(*other)),
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let lhs = PrimitiveFloat::new(lhs.value as f64);

                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                lhs.do_rem(rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = PrimitiveFloat::new(rhs.value as f64);

                lhs.do_rem(&rhs)
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString && rhs == PrimitiveType::PrimitiveInt =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveInt>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveInt::new(int).do_rem(rhs),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(float).do_rem(&PrimitiveFloat::new(rhs.value as f64))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveString
                    && rhs == PrimitiveType::PrimitiveFloat =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveString>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveFloat>().unwrap();

                match get_integer(&lhs.value) {
                    Ok(Integer::Int(int)) => PrimitiveFloat::new(int as f64).do_rem(rhs),
                    Ok(Integer::Float(float)) => PrimitiveFloat::new(float).do_rem(rhs),
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveInt && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveInt>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_rem(&PrimitiveInt::new(int)),
                    Ok(Integer::Float(float)) => {
                        PrimitiveFloat::new(lhs.value as f64).do_rem(&PrimitiveFloat::new(float))
                    }
                    Err(err) => Err(err),
                }
            }
            (lhs, rhs)
                if lhs == PrimitiveType::PrimitiveFloat
                    && rhs == PrimitiveType::PrimitiveString =>
            {
                let lhs = self.as_any().downcast_ref::<PrimitiveFloat>().unwrap();
                let rhs = other.as_any().downcast_ref::<PrimitiveString>().unwrap();

                match get_integer(&rhs.value) {
                    Ok(Integer::Int(int)) => lhs.do_rem(&PrimitiveFloat::new(int as f64)),
                    Ok(Integer::Float(float)) => lhs.do_rem(&PrimitiveFloat::new(float)),
                    Err(err) => Err(err),
                }
            }
            _ => Err(format!(
                "{} {:?} * {:?}",
                ERROR_ILLEGAL_OPERATION,
                self.get_type(),
                other.get_type()
            )),
        }
    }
}
