use crate::data::literal::ContentType;
use crate::data::primitive::{
    tools::check_usage, Primitive, PrimitiveBoolean, PrimitiveInt, PrimitiveNull, PrimitiveString,
    PrimitiveType, Right,
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

        map.insert(
            "type_of",
            (PrimitiveArray::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveArray::to_string as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "push",
            (PrimitiveArray::push as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "pop",
            (PrimitiveArray::pop as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "clear",
            (PrimitiveArray::clear as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "length",
            (PrimitiveArray::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_empty",
            (PrimitiveArray::is_empty as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "insert_at",
            (PrimitiveArray::insert_at as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "remove_at",
            (PrimitiveArray::remove_at as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "one_of",
            (PrimitiveArray::one_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "join",
            (PrimitiveArray::join as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_number",
            (PrimitiveArray::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "shuffle",
            (PrimitiveArray::shuffle as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "index_of",
            (PrimitiveArray::index_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "find",
            (PrimitiveArray::find as PrimitiveMethod, Right::Read),
        );

        map
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveArray {
    pub value: Vec<Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
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
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveArray {
    fn type_of(
        _array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "type_of()", interval)?;

        Ok(PrimitiveString::get_literal("array", interval))
    }

    fn to_string(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "to_string()", interval)?;

        Ok(PrimitiveString::get_literal(&array.to_string(), interval))
    }

    fn push(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "push(Primitive<T>)", interval)?;

        if array.value.len() + args.len() == usize::MAX {
            return Err(ErrorInfo {
                message: format!(
                    "Cannot push inside array, since array length is equal to {}",
                    usize::MAX
                ),
                interval,
            });
        }

        for literal in args.iter() {
            array.value.push(literal.to_owned());
        }

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn pop(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "pop()", interval)?;

        match array.value.pop() {
            Some(literal) => Ok(literal),
            None => Err(ErrorInfo {
                message: "Cannot pop if array is empty".to_owned(),
                interval,
            }),
        }
    }

    fn clear(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "clear()", interval)?;

        array.value.clear();

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn length(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "length()", interval)?;

        let result = array.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn is_empty(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_empty()", interval)?;

        let result = array.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn insert_at(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 2, "insert_at(Primitive<Int>, Primitive<T>)", interval)?;

        let (literal, value) = match (args.get(0), args.get(1)) {
            (Some(lhs), Some(rhs)) => (lhs, rhs),
            _ => {
                return Err(ErrorInfo {
                    message: "usage: need to have two parameters".to_owned(),
                    interval,
                });
            }
        };

        match Literal::get_value::<i64>(&literal.primitive) {
            Ok(res) => {
                check_index(*res, array.value.len() as i64, interval)?;

                array.value.insert(*res as usize, value.clone());

                Ok(PrimitiveNull::get_literal(interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "usage: first parameter must be of type int".to_owned(),
                interval,
            }),
        }
    }

    fn remove_at(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "remove_at(Primitive<Int>)", interval)?;

        let index = match args.get(0) {
            Some(res) => res,
            _ => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        match Literal::get_value::<i64>(&index.primitive) {
            Ok(res) => {
                check_index(*res, array.value.len() as i64, interval)?;

                Ok(array.value.remove(*res as usize))
            }
            Err(_) => Err(ErrorInfo {
                message: "usage: parameter must be of type int".to_owned(),
                interval,
            }),
        }
    }

    fn one_of(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "one_of()", interval)?;

        if let Some(res) = array
            .value
            .get(rand::thread_rng().gen_range(0, array.value.len()))
        {
            return Ok(res.to_owned());
        }

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn join(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "join(Primitive<String>)", interval)?;

        let mut result = String::new();

        let literal = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        match Literal::get_value::<String>(&literal.primitive) {
            Ok(separater) => {
                let length = array.value.len();

                for (index, string) in array.value.iter().enumerate() {
                    result.push_str(&string.primitive.to_string());

                    if index + 1 != length {
                        result.push_str(separater);
                    }
                }

                if result.is_empty() {
                    return Ok(PrimitiveNull::get_literal(interval));
                }

                Ok(PrimitiveString::get_literal(&result, interval))
            }
            Err(_) => Err(ErrorInfo {
                message: "usage: first parameter must be of type string".to_owned(),
                interval,
            }),
        }
    }

    fn is_number(
        _array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "is_number()", interval)?;

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn shuffle(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 0, "shuffle()", interval)?;

        array.value.shuffle(&mut rand::thread_rng());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn index_of(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "index_of(Primitive<T>)", interval)?;

        let args = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        for (index, literal) in array.value.iter().enumerate() {
            if literal == args {
                return Ok(PrimitiveInt::get_literal(index as i64, interval));
            }
        }

        Ok(PrimitiveInt::get_literal(-1, interval))
    }

    fn find(
        array: &mut PrimitiveArray,
        args: &[Literal],
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        check_usage(args, 1, "find(Primitive<T>)", interval)?;

        let mut vector: Vec<Literal> = Vec::new();

        let args = match args.get(0) {
            Some(res) => res,
            None => {
                return Err(ErrorInfo {
                    message: "usage: need to have one parameter".to_owned(),
                    interval,
                });
            }
        };

        for literal in array.value.iter() {
            if literal == args {
                vector.push(literal.to_owned());
            }
        }

        if vector.is_empty() {
            return Ok(PrimitiveNull::get_literal(interval));
        }

        Ok(PrimitiveArray::get_literal(&vector, interval))
    }
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

    fn do_div(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Div: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_mul(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Mul: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_rem(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] Rem: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bit_and(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] BitAnd: Illegal operation".to_owned(),
            interval: Interval { column: 0, line: 0 },
        })
    }

    fn do_bit_or(&self, _other: &dyn Primitive) -> Result<Box<dyn Primitive>, ErrorInfo> {
        Err(ErrorInfo {
            message: "[!] BitOr: Illegal operation".to_owned(),
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
