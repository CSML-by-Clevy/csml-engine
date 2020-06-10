use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::PrimitiveObject;
use crate::data::{Interval, Literal};
use crate::interpreter::json_to_literal;
use crate::data::primitive::PrimitiveArray;

use nom::lib::std::collections::HashMap;
use std::collections::HashSet;

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
        match (lhs, rhs) {
            (serde_json::Value::Null, serde_json::Value::Null) => Ok(serde_json::Value::Null),
            (serde_json::Value::Bool(lhs), serde_json::Value::Bool(rhs)) => {
                Ok(serde_json::Value::Bool(lhs | rhs))
            }
            (serde_json::Value::Number(lhs), serde_json::Value::Number(rhs)) => { // TODO
                unimplemented!();
            }
            (serde_json::Value::String(lhs), serde_json::Value::String(rhs)) => {
                Ok(serde_json::Value::String(lhs.to_string() + rhs))
            }
            (serde_json::Value::Array(lhs), serde_json::Value::Array(rhs)) => {
                let mut lhs = lhs.to_owned();

                lhs.extend(rhs.to_owned());

                Ok(serde_json::Value::Array(lhs))
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
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_index_of_key(key: &str, array: &Vec<serde_json::Value>) -> Option<usize> {
    for (index, object) in array.iter().enumerate() {
        if let Some(object) = object.as_object() {
            for value in object.keys() {
                if key == value {
                    return Some(index);
                }
            }
        }
    }

    None
}

fn get_index_of_parameter(key: &str, array: &Vec<serde_json::Value>) -> Result<usize, ErrorInfo> {
    let mut result = 0;

    for object in array.iter() {
        if let Some(object) = object.as_object() {
            for value in object.keys() {
                if let Some(serde_json::Value::Object(object)) = object.get(value){
                    if is_parameter_required(object) {
                        if key == value {
                            return Ok(result);
                        }
                        result += 1;
                    }

                }
            }
        }
    }

    unimplemented!();
}


fn get_parameter(args: &Literal, index: usize) -> Result<serde_json::Value, ErrorInfo> {
    let array = Literal::get_value::<Vec<Literal>>(
        &args.primitive,
        Interval::new_as_u32(0, 0),
        "".to_owned(),
    )?;

    match array.get(index) {
        Some(result) => Ok(result.primitive.to_json()),
        None => {
            println!("ERROR: array.get at index");
            unimplemented!();
        }
    }
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

fn get_default_object(
    key: &str,
    object: &serde_json::Map<String, serde_json::Value>,
    array: &Vec<serde_json::Value>,
    args: &Literal,
    hashset: &mut HashSet<String>,
) -> Result<serde_json::Value, ErrorInfo> {
    let mut result = create_default_object(object)?;

    if let Some(serde_json::Value::Array(default_value)) = object.get(key) {
        for function in default_value.iter() {
            if let serde_json::Value::Object(function) = function {
                if let Some(serde_json::Value::String(dependencie)) = function.get("$_get") {
                    result = serde_json::Value::add(
                        &result,
                        &get_object(dependencie, array, args, hashset)?,
                    )?;
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

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_object(
    key: &str,
    array: &Vec<serde_json::Value>,
    args: &Literal,
    hashset: &mut HashSet<String>,
) -> Result<serde_json::Value, ErrorInfo> {
    if let Some(index) =  get_index_of_key(key, array) {
        if let Some(serde_json::Value::Object(object)) = array[index].get(key) {
            if is_parameter_required(object) {
                return serde_json::Value::add(
                    &get_parameter(args, get_index_of_parameter(key, array)?)?,
                    &get_default_object("add_value", object, array, args, hashset)?,
                );
            }
            else {
                return serde_json::Value::add(
                    &get_default_object("default_value", object, array, args, hashset)?,
                    &get_default_object("add_value", object, array, args, hashset)?,
                );
            }
        }
    }

    println!("ERROR: key not found");
    unimplemented!();
}

fn get_result(name: &str, hashmap: &HashMap<String, Literal>, interval: Interval) -> Literal {
    let mut result = PrimitiveObject::get_literal(&hashmap, interval);

    result.set_content_type(name);

    result
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

// THIS MODULE CAN BE HEAVILY OPTIMISED, WITH MEMOIZATION
// A LOT OF COMPUTATION IS THE SAME, WE COULD KEEP THAT IN MEMORY

pub fn gen_generic_component(
    name: &str,
    interval: &Interval,
    args: &Literal,
    header: &serde_json::value::Value,
) -> Result<Literal, ErrorInfo> {
    let mut hashmap: HashMap<String, Literal> = HashMap::new();

    if let Some(object) = header.as_object() {
        if let Some(serde_json::Value::Array(array)) = object.get("params") {
            for object in array.iter() {
                if let Some(object) = object.as_object() {
                    for key in object.keys() {
                        hashmap.insert(
                            key.to_owned(),
                            json_to_literal(
                                &get_object(key, array, args, &mut HashSet::new())?,
                                *interval,
                            )?,
                        );
                    }
                }
            }
        }

    }

    println!();

    Ok(get_result(name, &hashmap, *interval))
}
