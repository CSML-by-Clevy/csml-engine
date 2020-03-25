use crate::data::literal::ContentType;
use crate::data::primitive::{
    Primitive, PrimitiveBoolean, PrimitiveInt, PrimitiveNull, PrimitiveString, PrimitiveType, Right,
};
use crate::data::{Interval, Literal, Message};
use crate::error_format::ErrorInfo;
use lazy_static::*;
use rand::seq::SliceRandom;
use rand::Rng;
use serde_json::json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::usize;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    array: &mut PrimitiveArray,
    args: &[Literal],
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert("is_number", (PrimitiveArray::is_number as PrimitiveMethod, Right::Read));
        map.insert("type_of", (PrimitiveArray::type_of as PrimitiveMethod, Right::Read));
        map.insert("to_string", (PrimitiveArray::to_string as PrimitiveMethod, Right::Read));

        map.insert("find", (PrimitiveArray::find as PrimitiveMethod, Right::Read));
        map.insert("is_empty", (PrimitiveArray::is_empty as PrimitiveMethod, Right::Read));
        map.insert("insert_at", (PrimitiveArray::insert_at as PrimitiveMethod, Right::Write));
        map.insert("index_of", (PrimitiveArray::index_of as PrimitiveMethod, Right::Read));
        map.insert("join", (PrimitiveArray::join as PrimitiveMethod, Right::Read));
        map.insert("length", (PrimitiveArray::length as PrimitiveMethod, Right::Read));
        map.insert("one_of", (PrimitiveArray::one_of as PrimitiveMethod, Right::Read));
        map.insert("push", (PrimitiveArray::push as PrimitiveMethod, Right::Write));
        map.insert("pop", (PrimitiveArray::pop as PrimitiveMethod, Right::Write));
        map.insert("remove_at", (PrimitiveArray::remove_at as PrimitiveMethod, Right::Write));
        map.insert("shuffle", (PrimitiveArray::shuffle as PrimitiveMethod, Right::Write));

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveArray {
    pub value: Vec<Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveArray {
    fn is_number(
        _array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn type_of(
        _array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal("array", interval))
    }

    fn to_string(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        Ok(PrimitiveString::get_literal(&array.to_string(), interval))
    }
}

impl PrimitiveArray {
    fn find(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "find(value: primitive) => array";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let value = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        let mut vector = Vec::new();

        for literal in array.value.iter() {
            if literal == value {
                vector.push(literal.to_owned());
            }
        }

        if !vector.is_empty() {
            return Ok(PrimitiveArray::get_literal(&vector, interval));
        }

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn is_empty(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_empty() => boolean";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = array.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn insert_at(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "insert_at(index: int, value: primitive) => null";

        if args.len() != 2 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let index = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                Literal::get_value::<i64>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: index must be of type 'int'".to_owned(),
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

        check_index(*index, array.value.len() as i64, interval)?;

        array.value.insert(*index as usize, value.clone());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn index_of(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "index_of(value: primitive) => int";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let value = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        for (index, literal) in array.value.iter().enumerate() {
            if literal == value {
                return Ok(PrimitiveInt::get_literal(index as i64, interval));
            }
        }

        Ok(PrimitiveInt::get_literal(-1, interval))
    }

    fn join(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "join(separator: string) => string";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let separator = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: separator must be of type 'string'".to_owned(),
                    interval,
                ));
            }
        };

        let length = array.value.len();
        let mut result = String::new();

        for (index, string) in array.value.iter().enumerate() {
            result.push_str(&string.primitive.to_string());

            if index + 1 != length {
                result.push_str(separator);
            }
        }

        Ok(PrimitiveString::get_literal(&result, interval))
    }

    fn length(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "length() => int";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let result = array.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn one_of(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "one_of() => primitive";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        if let Some(res) = array
            .value
            .get(rand::thread_rng().gen_range(0, array.value.len()))
        {
            return Ok(res.to_owned());
        }

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn push(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "push(value: primitive) => null";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let value = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
            }
        };

        if array.value.len() + args.len() == usize::MAX {
            return Err(ErrorInfo::new(
                format!(
                    "error: can't push inside array since array length is equal to size max: {}",
                    usize::MAX
                ),
                interval,
            ));
        }

        array.value.push(value.to_owned());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn pop(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "pop() => primitive";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        match array.value.pop() {
            Some(literal) => Ok(literal),
            None => Err(ErrorInfo::new(
                "error: can't pop if array is empty".to_owned(),
                interval,
            )),
        }
    }

    fn remove_at(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "remove_at(index: int) => primitive";

        if args.len() != 1 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let index = match args.get(0) {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                Literal::get_value::<i64>(&res.primitive)?
            }
            _ => {
                return Err(ErrorInfo::new(
                    "error: index must be of type 'int'".to_owned(),
                    interval,
                ));
            }
        };

        check_index(*index, array.value.len() as i64, interval)?;

        Ok(array.value.remove(*index as usize))
    }

    fn shuffle(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "shuffle() => array";

        if args.len() != 0 {
            return Err(ErrorInfo::new(format!("usage: {}", usage), interval));
        }

        let mut vector = array.value.to_owned();

        vector.shuffle(&mut rand::thread_rng());

        Ok(PrimitiveArray::get_literal(&vector, interval))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn check_index(index: i64, length: i64, interval: Interval) -> Result<u8, ErrorInfo> {
    if index.is_negative() {
        return Err(ErrorInfo {
            message: "usage: index must be positive".to_owned(),
            interval,
        });
    }

    if index > length {
        return Err(ErrorInfo {
            message: "usage: index must be lower or equal than array.length()".to_owned(),
            interval,
        });
    }

    Ok(0)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveArray {
    pub fn new(value: &[Literal]) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn get_literal(vector: &[Literal], interval: Interval) -> Literal {
        let primitive = Box::new(PrimitiveArray::new(vector));

        Literal {
            content_type: "array".to_owned(),
            primitive,
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl Primitive for PrimitiveArray {
    fn do_exec(
        &mut self,
        name: &str,
        args: &[Literal],
        interval: Interval,
        _content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, interval)?;

            return Ok((res, *right));
        }

        Err(ErrorInfo {
            message: format!("unknown method '{}' for type Array", name),
            interval,
        })
    }

    fn is_eq(&self, other: &dyn Primitive) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value == other.value;
        }

        false
    }

    fn is_cmp(&self, other: &dyn Primitive) -> Option<Ordering> {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            return self.value.partial_cmp(&other.value);
        }

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
        PrimitiveType::PrimitiveArray
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        let mut vector: Vec<serde_json::Value> = Vec::new();

        for literal in self.value.iter() {
            vector.push(literal.primitive.to_json());
        }

        serde_json::Value::Array(vector)
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
        let vec = self.value.iter().fold(vec![], |mut acc, v| {
            acc.push(v.primitive.to_json());
            acc
        });
        Message {
            content_type,
            content: json!(vec),
        }
    }
}
