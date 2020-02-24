pub mod expr_to_literal;
pub mod gen_literal;
pub mod interval;
pub mod match_literals;
pub mod memory;
pub mod operations;

pub use expr_to_literal::expr_to_literal;

use crate::data::primitive::{
    null::PrimitiveNull, object::PrimitiveObject, string::PrimitiveString, PrimitiveType,
};
use crate::data::{
    ast::{Expr, Function, Identifier, Interval, PathExpr, PathLiteral},
    tokens::{EVENT, _METADATA},
    Data, Literal,
};
use crate::data::{MemoryType, MessageData, MSG};
use crate::error_format::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::match_functions,
    variable_handler::{
        gen_literal::gen_literal_form_event,
        memory::{save_literal_in_mem, search_in_memory_type, search_var_memory},
    },
};
use std::collections::HashMap;
use std::slice::Iter;
use std::sync::mpsc;

//TODO: return Warning or Error Component
pub fn get_literal(
    literal: &mut Literal,
    index: Option<Literal>,
) -> Result<&mut Literal, ErrorInfo> {
    let interval = literal.interval.to_owned();

    match (literal, index) {
        (literal_lhs, Some(literal_rhs))
            if literal_lhs.primitive.get_type() == PrimitiveType::PrimitiveArray
                && literal_rhs.primitive.get_type() == PrimitiveType::PrimitiveInt =>
        {
            let items =
                Literal::get_mut_value::<&mut Vec<Literal>>(&mut literal_lhs.primitive).unwrap();
            let value = Literal::get_value::<i64>(&literal_rhs.primitive).unwrap();

            match items.get_mut(*value as usize) {
                Some(lit) => Ok(lit),
                None => Err(ErrorInfo {
                    message: format!("Array don't have {} index", value),
                    interval: interval.to_owned(),
                }),
            }
        }
        (literal, None) => Ok(literal),
        (_, Some(_)) => Err(ErrorInfo {
            message: "value is not of type Array".to_owned(),
            interval,
        }),
    }
}

fn get_var_from_stepvar<'a>(
    name: &Identifier,
    data: &'a mut Data,
) -> Result<&'a mut Literal, ErrorInfo> {
    match data.step_vars.get_mut(&name.ident) {
        Some(var) => Ok(var),
        None => Err(ErrorInfo {
            message: format!("no variable named < {} > in memory", name.ident),
            interval: name.interval.to_owned(),
        }),
    }
}

pub fn get_at_index(lit: &mut Literal, index: usize) -> Option<&mut Literal> {
    match Literal::get_mut_value::<Vec<Literal>>(&mut lit.primitive) {
        Ok(vec) => vec.get_mut(index),
        Err(_) => None,
    }
}

pub fn get_value_from_key<'a>(lit: &'a mut Literal, key: &str) -> Option<&'a mut Literal> {
    match Literal::get_mut_value::<HashMap<String, Literal>>(&mut lit.primitive) {
        Ok(map) => map.get_mut(key),
        Err(_) => None,
    }
}

pub fn resolve_path(
    mut path: Vec<(Interval, PathExpr)>,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Vec<(Interval, PathLiteral)>, ErrorInfo> {
    let mut new_path = vec![];

    for (inter, node) in path.drain(0..) {
        match node {
            PathExpr::ExprIndex(expr) => {
                let lit = match_functions(&expr, data, root, sender)?;
                if let Ok(val) = Literal::get_value::<i64>(&lit.primitive) {
                    new_path.push((inter, PathLiteral::VecIndex(*val as usize)))
                } else if let Ok(val) = Literal::get_value::<String>(&lit.primitive) {
                    new_path.push((inter, PathLiteral::MapIndex(val.to_owned())))
                } else {
                    return Err(ErrorInfo {
                        message:
                            "index must be of type int or string  => var.[42] or var.[\"key\"]"
                                .to_owned(),
                        interval: inter.to_owned(),
                    });
                }
            }
            PathExpr::Func(Function {
                name,
                interval,
                args,
            }) => new_path.push((
                inter,
                PathLiteral::Func {
                    name,
                    interval,
                    args: match_functions(&args, data, root, sender)?,
                },
            )),
            PathExpr::StringIndex(key) => new_path.push((inter, PathLiteral::MapIndex(key))),
        }
    }
    Ok(new_path)
}

fn loop_path(
    mut lit: &mut Literal,
    new: Option<Literal>,
    path: &mut Iter<(Interval, PathLiteral)>,
    mem_type: &MemoryType,
) -> Result<(Literal, bool), ErrorInfo> {
    let mut tmp_update_var = false;
    while let Some((interval, action)) = path.next() {
        match action {
            PathLiteral::VecIndex(index) => match get_at_index(lit, *index) {
                Some(new_lit) => lit = new_lit,
                None => {
                    return Ok((
                        PrimitiveNull::get_literal("null", interval.to_owned()),
                        tmp_update_var,
                    ))
                }
            },
            PathLiteral::MapIndex(key) => {
                if let (Some(ref new), 0) = (&new, path.len()) {
                    let args = [
                        PrimitiveString::get_literal("string", key, interval.to_owned()),
                        new.to_owned(),
                    ];

                    lit.primitive.exec(
                        "insert",
                        &args,
                        interval.to_owned(),
                        mem_type,
                        &mut false,
                    )?;
                    return Ok((lit.to_owned(), true));
                } else {
                    match get_value_from_key(lit, key) {
                        Some(new_lit) => lit = new_lit,
                        None => {
                            return Ok((
                                PrimitiveNull::get_literal("null", interval.to_owned()),
                                tmp_update_var,
                            ))
                        }
                    }
                };
            }
            PathLiteral::Func {
                name,
                interval,
                args,
            } => {
                // TODO: change to args
                // TODO: need to pass interval
                // TODO: Warning msg element is unmutable ?
                let args = Literal::get_value::<Vec<Literal>>(&args.primitive).unwrap();
                let mut return_lit =
                    lit.primitive
                        .exec(name, args, *interval, mem_type, &mut tmp_update_var)?;
                let (lit_new, ..) = loop_path(&mut return_lit, None, path, mem_type)?;
                return Ok((lit_new, tmp_update_var));
            }
        }
    }
    if let Some(new) = new {
        *lit = new;
        tmp_update_var = true;
    }
    Ok((lit.to_owned(), tmp_update_var))
}

//TODO: Add Warning for nonexisting key
pub fn exec_path_actions(
    lit: &mut Literal,
    new: Option<Literal>,
    path: &Option<Vec<(Interval, PathLiteral)>>,
    mem_type: &MemoryType,
) -> Result<(Literal, bool), ErrorInfo> {
    if let Some(vec) = path {
        let mut path = vec.iter();
        let (return_lit, update) = loop_path(lit, new, &mut path, mem_type)?;

        Ok((return_lit, update))
    } else {
        let mut tmp_update_var = false;
        if let Some(new) = new {
            *lit = new;
            tmp_update_var = true;
        }
        Ok((lit.to_owned(), tmp_update_var))
    }
}

pub fn get_literal_form_metadata(
    path: &[(Interval, PathLiteral)],
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    let mut lit = match path.get(0) {
        Some((inter, PathLiteral::MapIndex(name))) => match data.memory.metadata.get(name) {
            Some(lit) => lit.to_owned(),
            None => PrimitiveNull::get_literal("null", inter.to_owned()),
        },
        Some((inter, _)) => {
            return Err(ErrorInfo {
                message: "_metadata expect key => _metadata.key or _metadata.[\"key\"]".to_owned(),
                interval: inter.to_owned(),
            })
        }
        None => unreachable!(),
    };
    let (lit, _tmp_mem_update) = exec_path_actions(
        &mut lit,
        None,
        &Some(path[1..].to_owned()),
        &MemoryType::Metadata,
    )?;
    Ok(lit)
}

//TODO: make error if try to change _metadata
pub fn get_var(
    var: Identifier,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let interval = &var.interval;
    match var.ident {
        // name if name == RETRIES =>
        name if name == EVENT => gen_literal_form_event(*interval, var.path, data, root, sender),
        name if name == _METADATA => match var.path {
            Some(path) => {
                let path = resolve_path(path, data, root, sender)?;
                get_literal_form_metadata(&path, data)
            }
            None => Ok(PrimitiveObject::get_literal(
                "object",
                &data.memory.metadata,
                interval.to_owned(),
            )),
        },
        _ => match get_var_from_mem(var.to_owned(), data, root, sender) {
            Ok((lit, name, mem_type, path)) => {
                let (new_literal, update_mem) = exec_path_actions(lit, None, &path, &mem_type)?;
                save_literal_in_mem(
                    lit.to_owned(),
                    name,
                    &mem_type,
                    update_mem,
                    data,
                    root,
                    sender,
                );
                Ok(new_literal)
            }
            Err(_) => Ok(PrimitiveNull::get_literal("null", interval.to_owned())),
        },
    }
}

pub fn get_var_from_mem<'a>(
    name: Identifier,
    data: &'a mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<
    (
        &'a mut Literal,
        String,
        MemoryType,
        Option<Vec<(Interval, PathLiteral)>>,
    ),
    ErrorInfo,
> {
    let path = if let Some(p) = name.path.clone() {
        Some(resolve_path(p, data, root, sender)?)
    } else {
        None
    };

    match search_in_memory_type(&name, data)? {
        var if var == "use" => {
            let lit = get_var_from_stepvar(&name, data)?;
            Ok((lit, name.ident, MemoryType::Use, path))
        }
        _ => {
            let lit = search_var_memory(name.clone(), data)?;
            Ok((lit, name.ident, MemoryType::Remember, path))
        }
    }
}

pub fn get_string_from_complexstring(
    exprs: &[Expr],
    interval: Interval,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Literal {
    let mut new_string = String::new();

    //TODO: log error with span
    for elem in exprs.iter() {
        match match_functions(elem, data, root, sender) {
            Ok(var) => new_string.push_str(&var.primitive.to_string()),
            Err(_) => {
                let literal = PrimitiveNull::get_literal("text", interval.to_owned());
                new_string.push_str(&literal.primitive.to_string());
            }
        }
    }

    PrimitiveString::get_literal("text", &new_string, interval)
}
