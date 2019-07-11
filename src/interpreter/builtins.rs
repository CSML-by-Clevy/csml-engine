pub mod api_functions;
pub mod reserved_functions;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{data::Data, variable_handler::*};
use crate::parser::ast::*;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::hash::BuildHasher;

pub fn search_for_key_in_vec<'a>(key: &str, vec: &'a [Expr]) -> Result<&'a Expr, String> {
    for elem in vec.iter() {
        if let Expr::FunctionExpr(ReservedFunction::Assign(name, var)) = elem {
            if name == key {
                return Ok(var);
            }
        }
    }

    Err(" search_for_key_in_vec".to_owned())
}

pub fn create_submap<S: BuildHasher>(
    keys: &[&str],
    args: &HashMap<String, Literal, S>,
) -> Result<Map<String, Value>, String> {
    let mut map = Map::new();

    for elem in args.keys() {
        if keys.iter().find(|&&x| x == elem).is_none() {
            if let Some(value) = args.get(&*elem) {
                map.insert(elem.clone(), Value::String(value.to_string()));
            }
        }
    }
    Ok(map)
}

pub fn value_or_default(
    key: &str,
    vec: &[Expr],
    default: Option<String>,
    data: &mut Data,
) -> Result<String, String> {
    match (search_for_key_in_vec(key, vec), default) {
        (Ok(arg), ..) => Ok(get_var_from_ident(arg, data)?.to_string()),
        (Err(..), Some(string)) => Ok(string),
        (Err(..), None) => Err(format!("Error: no key {} found", key)),
    }
}

//see if it can be a generic macro
// fn get_vec_from_box(expr: &Expr) -> Result<&Vec<Expr> > {
//     if let Expr::VecExpr(vec) = expr {
//         Ok(vec)
//     } else {
//         Err(Error::new(ErrorKind::Other, " get_vec_from_box"))
//     }
// }
