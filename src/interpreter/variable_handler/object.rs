use crate::error_format::data::ErrorInfo;
use crate::interpreter::variable_handler::get_literal;
use crate::parser::{
    ast::{Interval, Path},
    literal::Literal,
};
use std::collections::HashMap;

fn get_properties_form_object<'a>(
    literal: &'a mut Literal,
    interval: &Interval,
) -> Result<&'a mut HashMap<String, Literal>, Literal> {
    match literal {
        Literal::ObjectLiteral { properties, .. } => Ok(properties),
        Literal::FunctionLiteral { value, .. } => {
            let lit: &mut Literal = value;
            match lit {
                Literal::ObjectLiteral { properties, .. } => Ok(properties),
                _ => Err(Literal::null(interval.to_owned())),
            }
        }
        _ => Err(Literal::null(interval.to_owned())),
    }
}

fn search_ident_in_obj<'a>(
    map: &'a mut HashMap<String, Literal>,
    name: &str,
    index: Option<Literal>,
    interval: Interval,
    last: bool,
    new_lit: &Option<Literal>,
) -> Result<Option<&'a mut Literal>, ErrorInfo> {
    match map.contains_key(name) {
        true => Ok(Some(get_literal(map.get_mut(name).unwrap(), index)?)),
        false => {
            if let (Some(lit), true) = (new_lit, last) {
                map.insert(name.to_owned(), lit.to_owned());
                Ok(None)
            } else {
                Err(ErrorInfo {
                    message: format!("{} does not exist in Object", name),
                    interval: interval.to_owned(),
                })
            }
        }
    }
}

pub fn get_value_in_object(
    literal: &Literal,
    path: &[Path],
    interval: &Interval,
) -> Result<Literal, ErrorInfo> {
    let mut index_lit: &mut Literal = &mut literal.to_owned();
    for node in path.iter() {
        match node {
            Path::Normal(name) => {
                let map = match get_properties_form_object(index_lit, interval) {
                    Ok(val) => val,
                    //TODO: return warning if literal is not a object
                    Err(..) => return Ok(Literal::null(interval.clone())),
                };
                match search_ident_in_obj(map, name, None, interval.to_owned(), false, &None)? {
                    Some(lit) => index_lit = lit,
                    None => return Ok(Literal::null(interval.clone())),
                };
            }
            Path::AtIndex(name, index) => {
                let map = match get_properties_form_object(index_lit, interval) {
                    Ok(val) => val,
                    //TODO: return warning if literal is not a object
                    Err(..) => return Ok(Literal::null(interval.clone())),
                };
                match search_ident_in_obj(
                    map,
                    name,
                    Some(index.to_owned()),
                    interval.to_owned(),
                    false,
                    &None,
                )? {
                    Some(lit) => index_lit = lit,
                    None => return Ok(Literal::null(interval.clone())),
                };
            }
            Path::Exec(name, vars) => {
                index_lit.exec(name, vars.to_owned())?;
            }
        }
    }
    Ok(index_lit.to_owned())
}

pub fn update_value_in_object(
    mut literal: &mut Literal,
    new_lit: Option<Literal>,
    path: &[Path],
    interval: &Interval,
) -> Result<(), ErrorInfo> {
    let last = path.len() - 1;
    for (i, node) in path.iter().enumerate() {
        match node {
            Path::Normal(name) => {
                let map = match get_properties_form_object(literal, interval) {
                    Ok(val) => val,
                    //TODO: return warning if literal is not a object
                    Err(..) => {
                        return Err(ErrorInfo {
                            message: format!("{} does not exist in Object", name),
                            interval: interval.to_owned(),
                        })
                    }
                };
                match search_ident_in_obj(
                    map,
                    name,
                    None,
                    interval.to_owned(),
                    i == last,
                    &new_lit,
                )? {
                    Some(lit) => literal = lit,
                    None => return Ok(()),
                };
            }
            Path::AtIndex(name, index) => {
                let map = match get_properties_form_object(literal, interval) {
                    Ok(val) => val,
                    //TODO: return warning if literal is not a object
                    Err(..) => return Ok(()),
                };
                match search_ident_in_obj(
                    map,
                    name,
                    Some(index.to_owned()),
                    interval.to_owned(),
                    i == last,
                    &new_lit,
                )? {
                    Some(lit) => literal = lit,
                    None => return Ok(()),
                };
            }
            Path::Exec(name, vars) => {
                literal.exec(name, vars.to_owned())?;
            }
        }
    }
    if let Some(new_lit) = new_lit {
        *literal = new_lit;
    }
    Ok(())
}
