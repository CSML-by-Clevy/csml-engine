use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::PrimitiveObject;
use crate::data::{Interval, Literal};
use crate::interpreter::json_to_literal;

use nom::lib::std::collections::HashMap;
use std::collections::HashSet;

////////////////////////////////////////////////////////////////////////////////
// TRAIT IMPLEMENTATION
////////////////////////////////////////////////////////////////////////////////

trait ArithmeticOperation {
    fn add(
        lhs: &serde_json::Value,
        rhs: &serde_json::Value,
        interval: &Interval,
    ) -> Result<serde_json::Value, ErrorInfo>;
}

impl ArithmeticOperation for serde_json::Value {
    fn add(
        lhs: &serde_json::Value,
        rhs: &serde_json::Value,
        interval: &Interval,
    ) -> Result<serde_json::Value, ErrorInfo> {
        match (lhs, rhs) {
            (serde_json::Value::Null, serde_json::Value::Null) => Ok(serde_json::Value::Null),
            (serde_json::Value::Bool(lhs), serde_json::Value::Bool(rhs)) => {
                Ok(serde_json::Value::Bool(lhs | rhs))
            }
            (serde_json::Value::Number(lhs), serde_json::Value::Number(rhs)) => {
                if let (Some(lhs), Some(rhs)) = (lhs.as_i64(), rhs.as_i64()) {
                    if let Some(value) = lhs.checked_add(rhs) {
                        return Ok(serde_json::Value::Number(serde_json::Number::from(value)));
                    }
                }

                if let (Some(lhs), Some(rhs)) = (lhs.as_f64(), rhs.as_f64()) {
                    let a = lhs as i64;
                    let b = rhs as i64;

                    if let Some(value) = a.checked_add(b) {
                        if let Some(value) = serde_json::Number::from_f64(lhs + rhs) {
                            return Ok(serde_json::Value::Number(value));
                        }
                    }
                }

                if let (Some(lhs), Some(rhs)) = (lhs.as_i64(), rhs.as_f64()) {
                    let b = rhs as i64;

                    if let Some(value) = lhs.checked_add(b) {
                        if let Some(value) = serde_json::Number::from_f64(lhs as f64 + rhs) {
                            return Ok(serde_json::Value::Number(value));
                        }
                    }
                }

                if let (Some(lhs), Some(rhs)) = (lhs.as_f64(), rhs.as_i64()) {
                    let a = lhs as i64;

                    if let Some(value) = a.checked_add(rhs) {
                        if let Some(value) = serde_json::Number::from_f64(lhs + rhs as f64) {
                            return Ok(serde_json::Value::Number(value));
                        }
                    }
                }

                Err(ErrorInfo::new(
                    Position::new(*interval),
                    "Illegal operation: overflow".to_string(),
                ))
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
            (_, _) => Err(ErrorInfo::new(
                Position::new(*interval),
                "Illegal operation: arithmetic between two types".to_string(),
            )),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// TOOL FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn create_default_object(
    object: &serde_json::Map<String, serde_json::Value>,
    interval: &Interval,
) -> Result<serde_json::Value, ErrorInfo> {
    if let Some(serde_json::Value::String(result)) = object.get("type") {
        return match result.as_str() {
            "Null" => Ok(serde_json::Value::Null),
            "Bool" => Ok(serde_json::Value::Bool(false)),
            "Number" => Ok(serde_json::Value::Number(serde_json::Number::from(0))),
            "String" => Ok(serde_json::Value::String(String::default())),
            "Array" => Ok(serde_json::Value::Array(Vec::default())),
            "Object" => Ok(serde_json::Value::Object(serde_json::Map::default())),
            _ => Err(ErrorInfo::new(
                Position::new(*interval),
                format!("type '{}' is unknown", result),
            )),
        };
    }

    return Err(ErrorInfo::new(
        Position::new(*interval),
        "type value must exist on all keys".to_string(),
    ));
}

fn is_parameter_required(object: &serde_json::Map<String, serde_json::Value>) -> bool {
    let mut result = false;

    if let Some(serde_json::Value::Bool(value)) = object.get("required") {
        result = *value;
    }

    result
}

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

fn get_index_of_parameter(key: &str, array: &Vec<serde_json::Value>) -> Option<usize> {
    let mut result = 0;

    for object in array.iter() {
        if let Some(object) = object.as_object() {
            for value in object.keys() {
                if let Some(serde_json::Value::Object(object)) = object.get(value) {
                    if is_parameter_required(object) {
                        if key == value {
                            return Some(result);
                        }
                        result += 1;
                    }
                }
            }
        }
    }

    None
}

fn get_parameter(
    key: &str,
    array: &Vec<serde_json::Value>,
    args: &Literal,
    interval: &Interval,
) -> serde_json::Value {
    if let Some(index) = get_index_of_parameter(key, array) {
        if let Ok(array) =
            Literal::get_value::<Vec<Literal>>(&args.primitive, *interval, String::default())
        {
            if let Some(value) = array.get(index) {
                return value.primitive.to_json();
            }
        }
    }

    serde_json::Value::Null
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_result(name: &str, hashmap: &HashMap<String, Literal>, interval: Interval) -> Literal {
    let mut result = PrimitiveObject::get_literal(&hashmap, interval);

    result.set_content_type(name);

    result
}

fn get_default_object(
    key: &str,
    object: &serde_json::Map<String, serde_json::Value>,
    array: &Vec<serde_json::Value>,
    args: &Literal,
    interval: &Interval,
    memoization: &mut HashMap<String, serde_json::Value>,
    recursion: &mut HashSet<String>,
) -> Result<serde_json::Value, ErrorInfo> {
    // Create a default JSON value to be able to abstract all JSON types (STRING, OBJECT) and then apply the trait add to it.
    // Apply all rules if found of $_get and $_set, then launch recursion with key as the name of dependencie.
    // Eliminates circular dependencie by checking if we already visited this key
    // Eliminates recurrent recursion if object has already been created once.

    let mut result = create_default_object(object, interval)?;

    if let Some(serde_json::Value::Array(default_value)) = object.get(key) {
        for function in default_value.iter() {
            if let serde_json::Value::Object(function) = function {
                if let Some(serde_json::Value::String(dependencie)) = function.get("$_get") {
                    match memoization.get(dependencie) {
                        Some(value) => {
                            result = value.to_owned();
                        }
                        None => {
                            if recursion.contains(dependencie) {
                                return Err(ErrorInfo::new(
                                    Position::new(*interval),
                                    "CIRCULAR DEPENDECIE".to_string(),
                                ));
                            }

                            let value = &get_object(
                                dependencie,
                                array,
                                args,
                                interval,
                                memoization,
                                recursion,
                            )?;

                            memoization.insert(dependencie.to_string(), value.to_owned());

                            result = serde_json::Value::add(&result, value, interval)?;
                        }
                    }
                }
                if let Some(dependencie) = function.get("$_set") {
                    result = serde_json::Value::add(&result, dependencie, interval)?;
                }
            }
        }
    }

    Ok(result)
}

fn get_object(
    key: &str,
    array: &Vec<serde_json::Value>,
    args: &Literal,
    interval: &Interval,
    memoization: &mut HashMap<String, serde_json::Value>,
    recursion: &mut HashSet<String>,
) -> Result<serde_json::Value, ErrorInfo> {
    // Cache the key we visit
    // Find the key to work with, then construct the object like it's supposed to be.
    // Option 1: construct with parameters because required is true, so I need to find the right parameters and add all add_value function to it.
    // Option 2: construct with default_value function and like option 1, add all add_value function to it.

    if !recursion.insert(key.to_string()) {
        println!("SHOULD NEVER HAPPEN !");
        unreachable!();
    }

    if let Some(index) = get_index_of_key(key, array) {
        if let Some(serde_json::Value::Object(object)) = array[index].get(key) {
            return match is_parameter_required(object) {
                true => serde_json::Value::add(
                    &get_parameter(key, array, args, interval),
                    &get_default_object(
                        "add_value",
                        object,
                        array,
                        args,
                        interval,
                        memoization,
                        recursion,
                    )?,
                    interval,
                ),
                false => serde_json::Value::add(
                    &get_default_object(
                        "default_value",
                        object,
                        array,
                        args,
                        interval,
                        memoization,
                        recursion,
                    )?,
                    &get_default_object(
                        "add_value",
                        object,
                        array,
                        args,
                        interval,
                        memoization,
                        recursion,
                    )?,
                    interval,
                ),
            };
        }
    }

    Ok(serde_json::Value::Null)
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

// [TODO]: arithmetic operation on number

pub fn gen_generic_component(
    name: &str,
    interval: &Interval,
    args: &Literal,
    header: &serde_json::value::Value,
) -> Result<Literal, ErrorInfo> {
    // Dereferences the JSON Object, iterate on all key, and construct the component.
    // Create the hashmap that will be the result, and an hashmap for optimisation that will keep this module to make more than one equal computation.
    // Insert into the final result and eliminates recurrent recursion if object has already been created once.

    let mut hashmap: HashMap<String, Literal> = HashMap::new();
    let mut memoization: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(object) = header.as_object() {
        if let Some(serde_json::Value::Array(array)) = object.get("params") {
            for object in array.iter() {
                if let Some(object) = object.as_object() {
                    for key in object.keys() {
                        if let Some(result) = memoization.get(key) {
                            hashmap.insert(
                                key.to_owned(),
                                json_to_literal(&result.to_owned(), *interval)?,
                            );
                        } else {
                            let result = get_object(
                                key,
                                array,
                                args,
                                interval,
                                &mut memoization,
                                &mut HashSet::new(),
                            )?;

                            hashmap.insert(
                                key.to_owned(),
                                json_to_literal(&result.to_owned(), *interval)?,
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(get_result(name, &hashmap, *interval))
}
