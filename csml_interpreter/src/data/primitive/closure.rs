use crate::data::error_info::ErrorInfo;
use crate::data::literal::ContentType;
use crate::data::position::Position;
use crate::data::primitive::boolean::PrimitiveBoolean;
use crate::data::primitive::string::PrimitiveString;

use crate::data::primitive::Right;
use crate::data::primitive::{Primitive, PrimitiveType};
use crate::data::{
    ast::{Expr, Interval},
    message::Message,
    Data, Literal, MessageData, MSG,
};
use crate::error_format::*;
use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::{collections::HashMap, sync::mpsc};

pub fn capture_variables(literal: &mut Literal, memories: HashMap<String, Literal>, flow_name: &str) {
    if literal.content_type == "closure" {
        let mut closure = Literal::get_mut_value::<PrimitiveClosure>(
            &mut literal.primitive,
            flow_name,
            literal.interval,
            format!(""),
        )
        .unwrap();
        closure.enclosed_variables = Some(memories);
    }
}

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURES
////////////////////////////////////////////////////////////////////////////////

type PrimitiveMethod = fn(
    int: &mut PrimitiveClosure,
    args: &HashMap<String, Literal>,
    data: &mut Data,
    interval: Interval,
) -> Result<Literal, ErrorInfo>;

lazy_static! {
    static ref FUNCTIONS: HashMap<&'static str, (PrimitiveMethod, Right)> = {
        let mut map = HashMap::new();

        map.insert(
            "is_number",
            (PrimitiveClosure::is_number as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_int",
            (PrimitiveClosure::is_int as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "is_float",
            (PrimitiveClosure::is_float as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "type_of",
            (PrimitiveClosure::type_of as PrimitiveMethod, Right::Read),
        );
        map.insert(
            "to_string",
            (PrimitiveClosure::to_string as PrimitiveMethod, Right::Read),
        );

        map
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveClosure {
    pub args: Vec<String>,
    pub func: Box<Expr>,
    pub enclosed_variables: Option<HashMap<String, Literal>>,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveClosure {
    fn is_number(
        _int: &mut PrimitiveClosure,
        args: &HashMap<String, Literal>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_number() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn is_int(
        _int: &mut PrimitiveClosure,
        args: &HashMap<String, Literal>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_int() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn is_float(
        _int: &mut PrimitiveClosure,
        args: &HashMap<String, Literal>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "is_float() => boolean";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveBoolean::get_literal(false, interval))
    }

    fn type_of(
        _int: &mut PrimitiveClosure,
        args: &HashMap<String, Literal>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "type_of() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal("closure", interval))
    }

    fn to_string(
        closure: &mut PrimitiveClosure,
        args: &HashMap<String, Literal>,
        data: &mut Data,
        interval: Interval,
    ) -> Result<Literal, ErrorInfo> {
        let usage = "to_string() => string";

        if !args.is_empty() {
            return Err(gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("usage: {}", usage),
            ));
        }

        Ok(PrimitiveString::get_literal(&closure.to_string(), interval))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl PrimitiveClosure {
    pub fn new(
        args: Vec<String>,
        func: Box<Expr>,
        enclosed_variables: Option<HashMap<String, Literal>>,
    ) -> Self {
        Self {
            args,
            func,
            enclosed_variables,
        }
    }

    pub fn get_literal(
        args: Vec<String>,
        func: Box<Expr>,
        interval: Interval,
        enclosed_variables: Option<HashMap<String, Literal>>,
    ) -> Literal {
        let primitive = Box::new(PrimitiveClosure::new(args, func, enclosed_variables));

        Literal {
            content_type: "closure".to_owned(),
            primitive,
            interval,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// TRAIT FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[typetag::serde]
impl Primitive for PrimitiveClosure {
    fn is_eq(&self, _other: &dyn Primitive) -> bool {
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
        PrimitiveType::PrimitiveClosure
    }

    fn to_json(&self) -> serde_json::Value {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("_closure".to_owned(), serde_json::json!(self));

        serde_json::Value::Object(map)
    }

    fn format_mem(&self, _content_type: &str, _first: bool) -> serde_json::Value {
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        map.insert("_closure".to_owned(), serde_json::json!(self));

        serde_json::Value::Object(map)
    }

    fn to_string(&self) -> String {
        "Null".to_owned()
    }

    fn as_bool(&self) -> bool {
        false
    }

    fn get_value(&self) -> &dyn std::any::Any {
        self
    }

    fn get_mut_value(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_box_clone(&self) -> Box<dyn Primitive> {
        Box::new((*self).clone())
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
        _content_type: &ContentType,
        data: &mut Data,
        _msg_data: &mut MessageData,
        _sender: &Option<mpsc::Sender<MSG>>,
    ) -> Result<(Literal, Right), ErrorInfo> {
        if let Some((f, right)) = FUNCTIONS.get(name) {
            let res = f(self, args, data, interval)?;

            return Ok((res, *right));
        }

        Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            format!("[{}] {}", name, ERROR_CLOSURE_UNKNOWN_METHOD),
        ))
    }
}
