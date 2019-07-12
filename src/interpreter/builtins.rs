pub mod api_functions;
pub mod reserved_functions;

// use crate::interpreter::{data::Data, variable_handler::*};
use crate::error_format::data::ErrorInfo;
use crate::parser::ast::*;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::hash::BuildHasher;

// pub fn search_for_key_in_vec<'a>(key: &str, vec: &'a [Expr]) -> Result<&'a Expr, ErrorInfo> {
//     for elem in vec.iter() {
//         if let Expr::FunctionExpr(ReservedFunction::Assign(SmartIdent{ident: name, ..}, var)) = elem {
//             if name == key {
//                 return Ok(var);
//             }
//         }
//     }

// Err(
//      ErrorInfo{
//          message: "search_for_key_in_vec".to_owned(),
//          interval: interval_from_expr(e)
//      }
//  )
// }

pub fn create_submap<S: BuildHasher>(
    keys: &[&str],
    args: &HashMap<String, Literal, S>,
) -> Result<Map<String, Value>, ErrorInfo> {
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

// pub fn value_or_default(
//     key: &str,
//     vec: &[Expr],
//     default: Option<String>,
//     data: &mut Data,
// ) -> Result<String, ErrorInfo> {
//     match (search_for_key_in_vec(key, vec), default) {
//         (Ok(arg), ..) => Ok(get_var_from_ident(arg, data)?.literal.to_string()),
//         (Err(..), Some(string)) => Ok(string),
//         (Err(..), None) => Err(format!("Error: no key {} found", key)),
//     }
// }

//see if it can be a generic macro
// fn get_vec_from_box(expr: &Expr) -> Result<&Vec<Expr> > {
//     if let Expr::VecExpr(vec) = expr {
//         Ok(vec)
//     } else {
//         Err(Error::new(ErrorKind::Other, " get_vec_from_box"))
//     }
// }
