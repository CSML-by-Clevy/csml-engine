pub mod api_functions;
pub mod reserved_functions;

use std::io::{Error, ErrorKind, Result};
use serde_json::{Value, Map};

use crate::parser::{ast::{Expr, ReservedFunction}};
use crate::interpreter:: {
    json_to_rust::*,
    variable_handler::*,
};

pub fn search_for_key_in_vec<'a>(key: &str, vec: &'a [Expr]) -> Result<&'a Expr> {
    for elem in vec.iter() {
        if let Expr::FunctionExpr(ReservedFunction::Assign(name), var) = elem {
            if name == key {
                return Ok(var);
            } 
        }
    }

    Err(Error::new(ErrorKind::Other, " search_for_key_in_vec"))
}

pub  fn create_submap<'a>(keys: &[&str], vec: &'a [Expr], memory: &Memory, event: &Option<Event>) -> Result<Map<String, Value> > {
    let mut map = Map::new();

    for elem in vec.iter() {
        if let Expr::FunctionExpr(ReservedFunction::Assign(name), var) = elem {
            if keys.iter().find(|&&x| x == name).is_none() {
                let value = get_var_from_ident(memory, event, var)?.to_string();
                map.insert(name.clone(), Value::String(value));
            }
        }
    }

    Ok(map)
}

fn expr_to_vec(expr: &Expr) -> Result<&Vec<Expr> > {
    match expr {
        Expr::VecExpr(vec)  => Ok(vec),
        _                   => Err(Error::new(ErrorKind::Other, " expr_to_vec"))
    }
}

pub  fn value_or_default(key: &str, vec: &[Expr], default: Option<String>, memory: &Memory, event: &Option<Event>) -> Result<String> {
    match (search_for_key_in_vec(key, vec), default) {
        (Ok(arg), ..)           => Ok(get_var_from_ident(memory, event, arg)?.to_string()),
        (Err(..), Some(string)) => Ok(string),
        (Err(..), None)         => Err(Error::new(ErrorKind::Other, format!("Error: no key {} found", key)))
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
