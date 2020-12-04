use crate::data::position::Position;
use crate::data::{
    literal::ContentType,
    primitive::{
        Primitive, PrimitiveBoolean, PrimitiveInt, PrimitiveNull, PrimitiveString, PrimitiveType,
        Right,
    },
    tokens::TYPES,
};
use crate::data::{Interval, Literal, Message};
use crate::error_format::*;
use lazy_static::*;
use rand::seq::SliceRandom;
use rand::Rng;
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::usize;

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    array: &mut PrimitiveArray,
    args: &HashMap<String, Literal>,
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "is_number",
            (PrimitiveArray::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_int",
            (PrimitiveArray::is_int as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_float",
            (PrimitiveArray::is_float as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "type_of",
            (PrimitiveArray::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveArray::to_string as PrimitiveMethod, Right::Read),
        );

        map.insert(
            "find",
            (PrimitiveArray::find as PrimitiveMethod, Right::Read),
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
            "index_of",
            (PrimitiveArray::index_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "join",
            (PrimitiveArray::join as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "length",
            (PrimitiveArray::length as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "one_of",
            (PrimitiveArray::one_of as PrimitiveMethod, Right::Read),
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
            "remove_at",
            (PrimitiveArray::remove_at as PrimitiveMethod, Right::Write),
        );
        map.insert(
            "shuffle",
            (PrimitiveArray::shuffle as PrimitiveMethod, Right::Write),
        );

        map
    };
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveArray {
    pub value: Vec<Literal>,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn check_index(index: i64, length: i64, interval: Interval) -> Result<(), ErrorInfo> {
    if index.is_negative() {
        return Err(gen_error_info(
            Position::new(interval),
            ERROR_ARRAY_NEGATIVE.to_owned(),
        ));
    }

    if index > length {
        return Err(gen_error_info(
            Position::new(interval),
            ERROR_ARRAY_INDEX.to_owned(),
        ));
    }

    Ok(())
}

impl PrimitiveArray {
    fn is_number(
        _array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
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
        _array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
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
        _array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
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
        _array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("array", interval))
    }

    fn to_string(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&array.to_string(), interval))
    }
}

impl PrimitiveArray {
    fn find(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "find(value: primitive) => array";

        if array.value.len() + args.len() == usize::MAX {
            return Err(gen_error_info(
                Position::new(interval),
                format!("{} {}", ERROR_ARRAY_OVERFLOW, usize::MAX),
            ));
        }

        let value = match args.get("arg0") {
            Some(res) => res,
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ));
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

        Ok(PrimitiveArray::get_literal( &[], interval))
    }

    fn is_empty(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_empty() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = array.value.is_empty();

        Ok(PrimitiveBoolean::get_literal(result, interval))
    }

    fn insert_at(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "insert_at(index: int, value: primitive) => null";

        if args.len() != 2 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let index = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                Literal::get_value::<i64>(
                    &res.primitive,
                    interval,
                    ERROR_ARRAY_INSERT_AT.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_ARRAY_INSERT_AT.to_owned(),
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

        check_index(*index, array.value.len() as i64, interval)?;

        array.value.insert(*index as usize, value.clone());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn index_of(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "index_of(value: primitive) => int";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get("arg0") {
            Some(res) => res,
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ));
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
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "join(separator: string) => string";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let separator = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveString => {
                Literal::get_value::<String>(&res.primitive, interval, ERROR_ARRAY_JOIN.to_owned())?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_ARRAY_JOIN.to_owned(),
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
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "length() => int";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let result = array.value.len();

        Ok(PrimitiveInt::get_literal(result as i64, interval))
    }

    fn one_of(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "one_of() => primitive";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
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
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "push(value: primitive) => null";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let value = match args.get("arg0") {
            Some(res) => res,
            None => {
                return Err(gen_error_info(
                    Position::new(interval),
                    format!("usage: {}", usage),
                ));
            }
        };

        if array.value.len() + args.len() == usize::MAX {
            return Err(gen_error_info(
                Position::new(interval),
                format!("{} {}", ERROR_ARRAY_OVERFLOW, usize::MAX,),
            ));
        }

        array.value.push(value.to_owned());

        Ok(PrimitiveNull::get_literal(interval))
    }

    fn pop(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "pop() => primitive";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        match array.value.pop() {
            Some(literal) => Ok(literal),
            None => Err(gen_error_info(
                Position::new(interval),
                ERROR_ARRAY_POP.to_owned(),
            )),
        }
    }

    fn remove_at(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "remove_at(index: int) => primitive";

        if args.len() != 1 {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let index = match args.get("arg0") {
            Some(res) if res.primitive.get_type() == PrimitiveType::PrimitiveInt => {
                Literal::get_value::<i64>(
                    &res.primitive,
                    interval,
                    ERROR_ARRAY_REMOVE_AT.to_owned(),
                )?
            }
            _ => {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_ARRAY_REMOVE_AT.to_owned(),
                ));
            }
        };

        check_index(*index, array.value.len() as i64, interval)?;

        Ok(array.value.remove(*index as usize))
    }

    fn shuffle(
        array: &mut PrimitiveArray,
        args: &HashMap<String, Literal>,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "shuffle() => array";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval),
                format!("usage: {}", usage),
            ));
        }

        let mut vector = array.value.to_owned();

        vector.shuffle(&mut rand::thread_rng());

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

////////////////////////////////////////////////////////////////////////////////
// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[typetag::serde]
impl Primitive for PrimitiveArray {
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
        PrimitiveType::PrimitiveArray
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
    }

    fn to_json(&self) -> serde_json::Value {
        let mut vector: Vec<serde_json::Value> = Vec::new();

        for literal in self.value.iter() {
            let value = literal.primitive.to_json();

            if !TYPES.contains(&&(*literal.content_type)) {
                let mut map = serde_json::Map::new();
                map.insert(
                    "content_type".to_owned(),
                    serde_json::json!(literal.content_type),
                );
                map.insert("content".to_owned(), value);

                vector.push(serde_json::json!(map));
            } else {
                vector.push(value);
            }
        }

        serde_json::Value::Array(vector)
    }

    fn format_mem(&self, _content_type: &str, first: bool) -> serde_json::Value {
        let mut vector: Vec<serde_json::Value> = Vec::new();

        for literal in self.value.iter() {
            let content_type = &literal.content_type;
            let value = literal.primitive.format_mem(content_type, first);
            vector.push(value);
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

    fn do_exec(
        &mut self,
        name: &str,
        args: &HashMap<String, Literal>,
        interval: Interval,
        _content_type: &ContentType,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, interval)?;

            return Ok((res, *right));
        }

        Err(gen_error_info(
            Position::new(interval),
            format!("[{}] {}", name, ERROR_ARRAY_UNKNOWN_METHOD),
        ))
    }
}
