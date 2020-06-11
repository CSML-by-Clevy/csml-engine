use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::PrimitiveObject;
use crate::data::{Interval, Literal};
use crate::interpreter::json_to_literal;
use crate::data::primitive::PrimitiveArray;

use nom::lib::std::collections::HashMap;
use std::collections::HashSet;

////////////////////////////////////////////////////////////////////////////////
// TRAIT IMPLEMENTATION
////////////////////////////////////////////////////////////////////////////////

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

fn create_default_object(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<serde_json::Value, ErrorInfo> {
    // Create a default value of any JSON that will be used to abstract the JSON type and apply trait add to them.
    // Type must be written in the Component declaration !

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

fn is_parameter_required(object: &serde_json::Map<String, serde_json::Value>) -> bool {
    let mut result = false;

    if let Some(serde_json::Value::Bool(value)) = object.get("required") {
        result = *value;
    }

    result
}

fn get_index_of_key(key: &str, array: &Vec<serde_json::Value>) -> Option<usize> {
    // To keep the order of the Component declaration we had to create a vector params
    // Index of the key will help me to find the key that I need to work with

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
    // This index is needed to get the right parameter for a dependecies.

    let mut result = 0;

    for object in array.iter() {
        if let Some(object) = object.as_object() {
            for value in object.keys() {
                if let Some(serde_json::Value::Object(object)) = object.get(value){
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
    // Try to get the parameters given to component as an array
    // If an error occur, Null is return. This could be change in the future for stricter rules.

    if let Some(index) = get_index_of_parameter(key, array) {
        if let Ok(array) = Literal::get_value::<Vec<Literal>>(&args.primitive, *interval, String::default()) {
            if let Some(value) =  array.get(index) {
                return value.primitive.to_json();
            }
        }
    }

    serde_json::Value::Null
}

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_default_object(
    key: &str,
    object: &serde_json::Map<String, serde_json::Value>,
    array: &Vec<serde_json::Value>,
    args: &Literal,
    interval: &Interval,
    memoization: &mut HashMap<String, serde_json::Value>,
) -> Result<serde_json::Value, ErrorInfo> {
    // Create a default JSON value to be able to abstract all JSON types (STRING, OBJECT) and then apply the trait add to it.
    // Apply all rules if found of $_get and $_set, then launch recursion with key as the name of dependencie

    let mut result = create_default_object(object)?;

    if let Some(serde_json::Value::Array(default_value)) = object.get(key) {
        for function in default_value.iter() {
            if let serde_json::Value::Object(function) = function {
                if let Some(serde_json::Value::String(dependencie)) = function.get("$_get") {
                    result = serde_json::Value::add(
                        &result,
                        &get_object(dependencie, array, args, interval, memoization)?,
                    )?;
                }

                if let Some(dependencie) = function.get("$_set") {
                    result = serde_json::Value::add(&result, dependencie)?;
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
) -> Result<serde_json::Value, ErrorInfo> {
    // Find the key to work with, then construct the object like it's supposed to be.
    // Option 1: construct with parameters because required is true, so I need to find the right parameters and add all add_value function to it.
    // Option 2: construct with default_value function and like option 1, add all add_value function to it.

    if let Some(index) = get_index_of_key(key, array) {
        if let Some(serde_json::Value::Object(object)) = array[index].get(key) {
            match is_parameter_required(object) {
                true => {
                    let value = serde_json::Value::add(
                        &get_parameter(key, array, args, interval),
                        &get_default_object("add_value", object, array, args, interval, memoization)?,
                    )?;

                    memoization.insert(key.to_string(), value.to_owned());

                    return Ok(value);
                }
                false => {
                    let value = serde_json::Value::add(
                        &get_default_object("default_value", object, array, args, interval, memoization)?,
                        &get_default_object("add_value", object, array, args, interval, memoization)?,
                    )?;

                    memoization.insert(key.to_string(), value.to_owned());

                    return Ok(value);
                }
            }
        }
    }

    Ok(serde_json::Value::Null)
}

fn get_result(name: &str, hashmap: &HashMap<String, Literal>, interval: Interval) -> Literal {
    let mut result = PrimitiveObject::get_literal(&hashmap, interval);

    result.set_content_type(name);

    result
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

// [+] handle circular dependecie

pub fn gen_generic_component(
    name: &str,
    interval: &Interval,
    args: &Literal,
    header: &serde_json::value::Value,
) -> Result<Literal, ErrorInfo> {
    // Create the hashmap that will be the result, and an hashmap for optimisation that will keep this module to make more than one equal computation.
    // Dereferences the JSON Object, iterate on all key, and construct the component.

    let mut hashmap: HashMap<String, Literal> = HashMap::new();
    let mut memoization: HashMap<String, serde_json::Value> = HashMap::new();

    if let Some(object) = header.as_object() {
        if let Some(serde_json::Value::Array(array)) = object.get("params") {
            for object in array.iter() {
                if let Some(object) = object.as_object() {
                    for key in object.keys() {
                        if let Some(result) = memoization.get(key) {
                            hashmap.insert(key.to_owned(), json_to_literal(&result.to_owned(), *interval)?);
                        }
                        else {
                            let result = get_object(key, array, args, interval, &mut memoization)?;

                            hashmap.insert(key.to_owned(), json_to_literal(&result.to_owned(), *interval)?);
                        }
                    }
                }
            }
        }

    }

    println!();

    Ok(get_result(name, &hashmap, *interval))
}
