use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::PrimitiveObject;
use crate::data::{Interval, Literal};
use crate::interpreter::json_to_literal;

use nom::lib::std::collections::HashMap;
use std::collections::HashSet;

// [!] serde_json::Value store the object into alphabetic order, so we cannot rely on order of object
// [!] parameters of the Component or not a hashmap but a vector, parsing fail if ex: title = "Title"

// [+] We may not have use case for _primary
// [+] Do we need multiple type accept ?

// [*] Default_value is ok but we must have an arguments that i required true !
// [*] For now if squeleton is malformated, null is return

// [!] Maybe reformart format_error to handle different type of errors

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn create_generic_component(
    hashmap: &HashMap<String, Literal>,
    interval: Interval,
    name: &str,
) -> Literal {
    let mut literal = PrimitiveObject::get_literal(hashmap, interval);

    literal.set_content_type(name);

    literal
}

fn get_value_from_header<'a>(
    key: &'a str,
    value: &'a serde_json::Value,
    args: &Literal,
    hashset: &mut std::collections::HashSet<&'a str>,
) -> Result<serde_json::Value, ErrorInfo> {
    // Check if we already pass by that node, if yes we are in a circular recursion and we don't want to SO !
    // If a default value apply all the rules, and create all component dependecies
    // If required is equal to true, get the arguments from args

    // Otherwirse return NULL
    
    if let Some(serde_json::Value::Object(object)) = value.get(key) {
        if !hashset.insert(key) {
            return Err(ErrorInfo::new(Position::new(Interval::new_as_u32(0, 0)), "Circular dependencies".to_string()));
        }

        if let Some(serde_json::Value::Object(default_value)) = object.get("default_value") {
            for (key, val) in default_value.iter() {
                if key == "$_get" {
                    if let Some(key) = val.as_str() {
                        let value = get_value_from_header(key, value, args, hashset)?;

                        return match object.get("type") {
                            Some(serde_json::Value::String(result)) if result == "Array" => Ok(serde_json::Value::Array(vec![value])),
                            _ => Ok(value),
                        }
                    }
                }
            }
        }

        if let Some(serde_json::Value::Bool(required)) = object.get("required") {
            if *required == true {
                return Ok(serde_json::Value::String("parameters".to_string()));
            }
        }
    }

    Ok(serde_json::Value::Null)
}

// {
//     "Button": {
//         "_primary": "title",
//         "title": {
//             "required": true,
//             "type": "String"
//         },
//         "payload": {
//             "required": false,
//             "type": "String",
//             "default_value": {
//                 "$_get": "title"
//             }
//         },
//         "accepts": {
//             "type": "Array",
//             "add_values": [
//                 {
//                     "$_get": "title"
//                 },
//                 {
//                     "$_get": "payload"
//                 },
//                 {
//                     "$_set": "poepoe"
//                 },
//                 "machin"
//             ],
//             "default_value": [
//                 "tototot",
//                 "tototot",
//                 "tototot",
//                 "tototot",
//                 "tototot",
//                 "tototot"
//             ]
//         }
//     }
// }

// Use default value if no named parameters is equal to key and _primary is not equal to key
// Use null if default value is not given and no parameters

fn get_object<'a>(
    key: &'a str,
    value: &'a serde_json::Value,
    args: &Literal,
    hashset: &mut std::collections::HashSet<&'a str>,
) -> Result<serde_json::Value, ErrorInfo> {
    if let Some(serde_json::Value::Object(object)) = value.get(key) {
        if !hashset.insert(key) {
            return Err(ErrorInfo::new(Position::new(Interval::new_as_u32(0, 0)), "Circular dependencies".to_string()));
        }


        
        if let Some(serde_json::Value::String(t)) = object.get("type") {
            let mut object = serde_json::Value::Array(vec![]);

            object.as_array_mut().unwrap().append(get_object(key, value, args, hashset)



            return Ok(object);
        }


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
            println!("[+] key: {}", key);

            let object = get_object(key, header, args, &mut HashSet::new())?;

            dbg!(object);
            println!();

            // hashmap.insert(
            //     key.to_owned(),
            //     json_to_literal(
            //         &get_value_from_header(key, header, args, &mut HashSet::new())?,
            //         Interval::default(),
            //     )?,
            // );
        }
    }

    println!();
    unimplemented!();

    Ok(create_generic_component(&hashmap, *interval, name))
}
