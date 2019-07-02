pub mod api_functions;
pub mod reserved_functions;

use std::collections::HashMap;
use serde_json::{Value, Map};
use crate::parser::{ast::*};
use crate::interpreter:: {
    variable_handler::*,
    data::Data,
};

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

pub fn create_submap(keys: &[&str], args: &HashMap<String, Literal>, data: &mut Data) -> Result<Map<String, Value>, String> {
    let mut map = Map::new();

    for elem in args.keys() {
        if keys.iter().find(|&&x| x == elem).is_none() {
            match args.get(&*elem) {
                Some(value) => {map.insert(elem.clone(), Value::String(value.to_string()));},
                None        => {}
            }
        }
    }
    Ok(map)
}

fn expr_to_vec(expr: &Expr) -> Result<&Vec<Expr>, String> {
    match expr {
        Expr::VecExpr(vec)  => Ok(vec),
        err                 => Err(format!(" expr_to_vec {:?} ", err).to_owned())
    }
}

pub fn value_or_default(key: &str, vec: &[Expr], default: Option<String>, data: &mut Data) -> Result<String, String> {
    match (search_for_key_in_vec(key, vec), default) {
        (Ok(arg), ..)           => Ok(get_var_from_ident(arg, data)?.to_string()),
        (Err(..), Some(string)) => Ok(string),
        (Err(..), None)         => Err(format!("Error: no key {} found", key))
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
