use crate::data::error_info::ErrorInfo;
// use crate::data::position::Position;
use crate::data::primitive::PrimitiveObject;
use crate::data::{Interval, Literal};
use crate::interpreter::json_to_literal;

use nom::lib::std::collections::HashMap;
// use std::collections::HashSet;

trait ArithmeticOperation {
    fn add(
        lhs: &serde_json::Value,
        rhs: &serde_json::Value,
    ) -> Result<serde_json::Value, ErrorInfo>;
}

impl ArithmeticOperation for serde_json::Value {
    fn add(
        lhs: &serde_json::Value,
        rhs: &serde_json::Value,
    ) -> Result<serde_json::Value, ErrorInfo> {
        dbg!(lhs);
        dbg!(rhs);
        println!();

        match (lhs, rhs) {
            (serde_json::Value::Null, serde_json::Value::Null) => Ok(serde_json::Value::Null),
            (serde_json::Value::Bool(lhs), serde_json::Value::Bool(rhs)) => {
                Ok(serde_json::Value::Bool(lhs | rhs))
            }
            (serde_json::Value::Number(lhs), serde_json::Value::Number(rhs)) => {
                unimplemented!();
            }
            (serde_json::Value::String(lhs), serde_json::Value::String(rhs)) => {
                Ok(serde_json::Value::String(lhs.to_string() + rhs))
            }
            (serde_json::Value::Array(lhs), serde_json::Value::Array(rhs)) => {
                Ok(serde_json::Value::Array([&lhs[..], &rhs[..]].concat()))
            }
            (serde_json::Value::Object(lhs), serde_json::Value::Object(rhs)) => {
                let mut lhs = lhs.to_owned();

                lhs.extend(rhs.to_owned());

                Ok(serde_json::Value::Object(lhs))
            }
            (_, _) => {
                println!("ERROR: Illegal operation");
                unimplemented!();
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_parameter() -> Option<serde_json::Value> {
    None
}

fn is_type_valid() -> bool {
    false
}

fn is_parameter_required(object: &serde_json::Map<String, serde_json::Value>) -> bool {
    if let Some(serde_json::Value::Bool(result)) = object.get("required") {
        if *result == true {
            return true;
        }
    }

    false
}

fn create_default_object(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<serde_json::Value, ErrorInfo> {
    if let Some(serde_json::Value::String(result)) = object.get("type") {
        return match result.as_str() {
            "Null" => Ok(serde_json::Value::Null),
            "Bool" => Ok(serde_json::Value::Bool(false)),
            "Number" => Ok(serde_json::Value::Number(serde_json::Number::from(0))),
            "String" => Ok(serde_json::Value::String(String::default())),
            "Array" => Ok(serde_json::Value::Array(Vec::default())),
            "Object" => Ok(serde_json::Value::Object(serde_json::Map::default())),
            _ => {
                println!("ERROR: type not handled");
                unimplemented!();
            }
        };
    }

    println!("ERROR: type must exist !");
    unimplemented!();
}

fn get_default_object<'a>(
    object: &serde_json::Map<String, serde_json::Value>,
    value: &'a serde_json::Value,
    args: &Literal,
) -> Result<serde_json::Value, ErrorInfo> {
    let mut result = create_default_object(object)?;

    if let Some(default_value) = object.get("default_value") {
        for function in default_value
            .as_array()
            .unwrap_or(&vec![serde_json::Value::default()])
            .iter()
        {
            if let serde_json::Value::Object(function) = function {
                if let Some(serde_json::Value::String(dependencie)) = function.get("$_get") {
                    result =
                        serde_json::Value::add(&result, &get_object(dependencie, value, args)?)?;
                }
                if let Some(dependencie) = function.get("$_set") {
                    result = serde_json::Value::add(&result, dependencie)?;
                }
            } else {
                println!("ERROR: function must be inside object");
                unimplemented!();
            }
        }
    }

    Ok(result)
}

fn get_object<'a>(
    key: &'a str,
    value: &'a serde_json::Value,
    args: &Literal,
) -> Result<serde_json::Value, ErrorInfo> {
    if let Some(serde_json::Value::Object(object)) = value.get(key) {
        // If a named parameter is given, or a parameter exist and self is _primary
        if let Some(parameter) = get_parameter() {
            if !is_type_valid() {
                // error
            }
        }
        // This will be the possible recursive call.
        else {
            if is_parameter_required(object) {
                println!("ERROR: no parameters has been given");
                unimplemented!();
            }

            return get_default_object(object, value, args);
        }

        unimplemented!()
    }

    Ok(serde_json::Value::Null)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn gen_generic_component(
    name: &str,
    interval: &Interval,
    args: &Literal,
    header: &serde_json::value::Value,
) -> Result<Literal, ErrorInfo> {
    let mut hashmap: HashMap<String, Literal> = HashMap::new();

    if let Some(object) = header.as_object() {
        for key in object.keys().skip_while(|key| *key == "_primary") {
            hashmap.insert(
                key.to_owned(),
                json_to_literal(&get_object(key, header, args)?, *interval)?,
            );
        }
    }

    println!();

    let mut result = PrimitiveObject::get_literal(&hashmap, *interval);

    result.set_content_type(name);

    Ok(result)
}
