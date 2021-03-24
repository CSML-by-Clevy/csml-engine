pub mod expr_to_literal;
pub mod gen_generic_component;
pub mod gen_literal;
pub mod interval;
pub mod match_literals;
pub mod memory;
pub mod operations;
pub mod resolve_csml_object;

use crate::data::literal::ContentType;
pub use expr_to_literal::{expr_to_literal, resolve_fn_args};

use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{
    tools::get_array, PrimitiveNull, PrimitiveObject, PrimitiveString, PrimitiveType,
};
use crate::data::{
    ast::{Expr, Function, GotoValueType, Identifier, Interval, PathLiteral, PathState},
    tokens::{COMPONENT, EVENT, _ENV, _METADATA},
    data::{Data, init_child_context},
    ArgsType, Literal, MemoryType, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::variable_handler::{
    gen_literal::gen_literal_from_component,
    gen_literal::gen_literal_from_event,
    memory::{save_literal_in_mem, search_in_memory_type, search_var_memory},
};
use std::slice::Iter;
use std::{collections::HashMap, sync::mpsc};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_var_from_step_var<'a>(
    name: &Identifier,
    data: &'a mut Data,
) -> Result<&'a mut Literal, ErrorInfo> {
    match data.step_vars.get_mut(&name.ident) {
        Some(var) => Ok(var),
        None => Err(gen_error_info(
            Position::new(name.interval),
            format!("< {} > {}", name.ident, ERROR_STEP_MEMORY),
        )),
    }
}

fn loop_path(
    mut lit: &mut Literal,
    condition: bool,
    new: Option<Literal>,
    path: &mut Iter<(Interval, PathLiteral)>,
    content_type: &ContentType,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<(Literal, bool), ErrorInfo> {
    let mut tmp_update_var = false;
    // this is temporary until we find a better way, it helps restore the string in the
    // string index otherwise the string will be replaced by the char at the index
    let mut old_string = None;

    while let Some((interval, action)) = path.next() {
        match action {
            PathLiteral::VecIndex(index)
                if lit.primitive.get_type() == PrimitiveType::PrimitiveString =>
            {
                match get_string_index(lit.clone(), *index)? {
                    Some(new_lit) => {
                        old_string = Some((lit.clone(), *index));
                        *lit = new_lit
                    }
                    None => {
                        let err = gen_error_info(
                            Position::new(*interval),
                            format!("[{}] {}", index, ERROR_ARRAY_INDEX),
                        );
                        let null = match condition {
                            true => PrimitiveNull::get_literal(err.position.interval),
                            false => MSG::send_error_msg(&sender, msg_data, Err(err)),
                        };
                        return Ok((null, tmp_update_var));
                    }
                }
            }
            PathLiteral::VecIndex(index) => match get_at_index(lit, *index) {
                Some(new_lit) => lit = new_lit,
                None => {
                    let err = gen_error_info(
                        Position::new(*interval),
                        format!("[{}] {}", index, ERROR_ARRAY_INDEX),
                    );
                    let null = match condition {
                        true => PrimitiveNull::get_literal(err.position.interval),
                        false => MSG::send_error_msg(&sender, msg_data, Err(err)),
                    };
                    return Ok((null, tmp_update_var));
                }
            },
            PathLiteral::MapIndex(key) => {
                if let (Some(ref new), 0) = (&new, path.len()) {
                    let mut args = HashMap::new();

                    args.insert(
                        "arg0".to_owned(),
                        PrimitiveString::get_literal(key, interval.to_owned()),
                    );
                    args.insert("arg1".to_owned(), new.to_owned());

                    lit.primitive.exec(
                        "insert",
                        &args,
                        interval.to_owned(),
                        content_type,
                        &mut false,
                        data,
                        msg_data,
                        sender
                    )?;
                    return Ok((lit.to_owned(), true));
                } else {
                    match get_value_from_key(lit, key) {
                        Some(new_lit) => lit = new_lit,
                        None => {
                            let err = gen_error_info(
                                Position::new(*interval),
                                format!("[{}] {}", key, ERROR_OBJECT_GET),
                            );
                            let null = match condition {
                                true => PrimitiveNull::get_literal(err.position.interval),
                                false => MSG::send_error_msg(&sender, msg_data, Err(err)),
                            };
                            return Ok((null, tmp_update_var));
                        }
                    }
                };
            }
            PathLiteral::Func {
                name,
                interval,
                args,
            } => {
                let args = match args {
                    ArgsType::Normal(args) => args,
                    ArgsType::Named(_) => {
                        let err = gen_error_info(
                            Position::new(*interval),
                            format!("{}", ERROR_METHOD_NAMED_ARGS),
                        );
                        return Ok((
                            MSG::send_error_msg(&sender, msg_data, Err(err)),
                            tmp_update_var,
                        ));
                    }
                };

                if let Some((old_string, _)) = old_string {
                    *lit = old_string
                }

                let mut return_lit = match lit.primitive.exec(
                    name,
                    args,
                    *interval,
                    content_type,
                    &mut tmp_update_var,
                    data,
                    msg_data,
                    sender
                ) {
                    Ok(lit) => lit,
                    Err(err) => MSG::send_error_msg(sender, msg_data, Err(err)),
                };

                let content_type = ContentType::get(&return_lit);
                let (lit_new, ..) = loop_path(
                    &mut return_lit,
                    false,
                    None,
                    path,
                    &content_type,
                    data,
                    msg_data,
                    sender,
                )?;

                return Ok((lit_new, tmp_update_var));
            }
        }
    }

    match (new, old_string) {
        (Some(new), None) => {
            *lit = new;
            tmp_update_var = true;
        }
        (None, Some((old_string, _index))) => {
            let return_value = lit.clone();
            *lit = old_string;
            return Ok((return_value, tmp_update_var));
        }
        (Some(new), Some((old_string, index))) => {
            let interval = old_string.interval.to_owned();
            let old_string = Literal::get_value::<String>(
                &old_string.primitive,
                old_string.interval.to_owned(),
                ERROR_INDEXING.to_owned(),
            )?
            .to_owned();
            let add_string = new.primitive.to_string();

            let new_string: String = old_string
                .chars()
                .enumerate()
                .fold(vec![], |mut acc, (index_1, value)| {
                    if index == index_1 {
                        for val in add_string.chars() {
                            acc.push(val);
                        }
                    } else {
                        acc.push(value);
                    }
                    acc
                })
                .into_iter()
                .collect();

            *lit = PrimitiveString::get_literal(&new_string, interval);
            tmp_update_var = true;
        }
        _ => {}
    }

    Ok((lit.to_owned(), tmp_update_var))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

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
            let items = Literal::get_mut_value::<&mut Vec<Literal>>(
                &mut literal_lhs.primitive,
                literal_lhs.interval,
                ERROR_ARRAY_TYPE.to_owned(),
            )?;
            let value = Literal::get_value::<i64>(
                &literal_rhs.primitive,
                literal_rhs.interval,
                ERROR_ARRAY_INDEX_TYPE.to_owned(),
            )?;

            match items.get_mut(*value as usize) {
                Some(lit) => Ok(lit),
                None => Err(gen_error_info(
                    Position::new(interval),
                    format!("{} {}", value, ERROR_ARRAY_INDEX_EXIST.to_owned()),
                )),
            }
        }
        (literal, None) => Ok(literal),
        (_, Some(_)) => Err(gen_error_info(
            Position::new(interval),
            ERROR_ARRAY_TYPE.to_owned(),
        )),
    }
}

pub fn get_string_index(lit: Literal, index: usize) -> Result<Option<Literal>, ErrorInfo> {
    let array = get_array(lit, ERROR_INDEXING.to_owned())?;

    match array.get(index) {
        Some(value) => Ok(Some(value.to_owned())),
        None => Ok(None),
    }
}

pub fn get_at_index(lit: &mut Literal, index: usize) -> Option<&mut Literal> {
    let vec = Literal::get_mut_value::<Vec<Literal>>(
        &mut lit.primitive,
        lit.interval,
        ERROR_ARRAY_TYPE.to_owned(),
    )
    .ok()?;
    vec.get_mut(index)
}

pub fn get_value_from_key<'a>(lit: &'a mut Literal, key: &str) -> Option<&'a mut Literal> {
    let map = Literal::get_mut_value::<HashMap<String, Literal>>(
        &mut lit.primitive,
        lit.interval,
        ERROR_OBJECT_TYPE.to_owned(),
    )
    .ok()?;
    map.get_mut(key)
}

pub fn resolve_path(
    path: &[(Interval, PathState)],
    condition: bool,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Vec<(Interval, PathLiteral)>, ErrorInfo> {
    let mut new_path = vec![];

    for (interval, node) in path.iter() {
        match node {
            PathState::ExprIndex(expr) => {
                let lit = expr_to_literal(&expr, condition, None, data, msg_data, sender)?;
                if let Ok(val) = Literal::get_value::<i64>(
                    &lit.primitive,
                    lit.interval,
                    ERROR_UNREACHABLE.to_owned(),
                ) {
                    new_path.push((interval.to_owned(), PathLiteral::VecIndex(*val as usize)))
                } else if let Ok(val) = Literal::get_value::<String>(
                    &lit.primitive,
                    lit.interval,
                    ERROR_UNREACHABLE.to_owned(),
                ) {
                    new_path.push((interval.to_owned(), PathLiteral::MapIndex(val.to_owned())))
                } else {
                    return Err(gen_error_info(
                        Position::new(*interval),
                        ERROR_FIND_BY_INDEX.to_owned(),
                    ));
                }
            }
            PathState::Func(Function {
                name,
                interval,
                args,
            }) => new_path.push((
                interval.to_owned(),
                PathLiteral::Func {
                    name: name.to_owned(),
                    interval: interval.to_owned(),
                    args: resolve_fn_args(&args, data, msg_data, sender)?,
                },
            )),
            PathState::StringIndex(key) => {
                new_path.push((interval.to_owned(), PathLiteral::MapIndex(key.to_owned())))
            }
        }
    }
    Ok(new_path)
}

//TODO: Add Warning for nonexisting key
pub fn exec_path_actions(
    lit: &mut Literal,
    condition: bool,
    new: Option<Literal>,
    path: &Option<Vec<(Interval, PathLiteral)>>,
    content_type: &ContentType,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<(Literal, bool), ErrorInfo> {
    if let Some(vec) = path {
        let mut path = vec.iter();
        let (return_lit, update) = loop_path(
            lit,
            condition,
            new,
            &mut path,
            content_type,
            data,
            msg_data,
            sender,
        )?;

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

pub fn get_literal_from_metadata(
    path: &[(Interval, PathLiteral)],
    condition: bool,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let mut lit = match path.get(0) {
        Some((interval, PathLiteral::MapIndex(name))) => match data.context.metadata.get(name) {
            Some(lit) => lit.to_owned(),
            None => PrimitiveNull::get_literal(interval.to_owned()),
        },
        Some((interval, _)) => {
            return Err(gen_error_info(
                Position::new(*interval),
                ERROR_FIND_BY_INDEX.to_owned(),
            ));
        }
        None => unreachable!(),
    };

    let content_type = ContentType::get(&lit);
    let (lit, _tmp_mem_update) = exec_path_actions(
        &mut lit,
        condition,
        None,
        &Some(path[1..].to_owned()),
        &content_type,
        data,
        msg_data,
        sender,
    )?;
    Ok(lit)
}

pub fn get_var(
    var: Identifier,
    condition: bool, // TODO: find better method than this
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let interval = &var.interval;

    match var.ident {
        name if name == COMPONENT => {
            gen_literal_from_component(*interval, path, data, msg_data, sender)
        }
        name if name == EVENT => {
            gen_literal_from_event(*interval, condition, path, data, msg_data, sender)
        }
        name if name == _ENV => match path {
            Some(path) => {
                let path = resolve_path(path, condition, data, msg_data, sender)?;

                let content_type = ContentType::get(&data.env);
                let (lit, _tmp_mem_update) = exec_path_actions(
                    &mut data.env.clone(),
                    condition,
                    None,
                    &Some(path.to_owned()),
                    &content_type,
                    data,
                    msg_data,
                    sender,
                )?;
                Ok(lit)
            }
            None => Ok(data.env.clone()),
        },
        name if name == _METADATA => match path {
            Some(path) => {
                let path = resolve_path(path, condition, data, msg_data, sender)?;
                get_literal_from_metadata(&path, condition, data, msg_data, sender)
            }
            None => Ok(PrimitiveObject::get_literal(
                &data.context.metadata,
                interval.to_owned(),
            )),
        },
        _ => {
            // ######################
            let mut context_tmp = init_child_context(&data);
            let flows_tmp = data.flows.clone();
            let flow_tmp = data.flow.clone();
            let event_tmp = data.event.clone();
            let env_tmp = data.env.clone();
            let loop_indexs_tmp = data.loop_indexs.clone();
            let loop_index_tmp = data.loop_index.clone();
            let step_vars_tmp = data.step_vars.clone();
            let custom_component_tmp = data.custom_component.clone();
            let native_component_tmp = data.native_component.clone();
            let mut new_scope_data = Data {
                flows: &flows_tmp,
                flow: &flow_tmp,
                context: &mut context_tmp,
                event: &event_tmp,
                env: &env_tmp,
                loop_indexs: loop_indexs_tmp,
                loop_index: loop_index_tmp,
                step_vars: step_vars_tmp,
                custom_component: &custom_component_tmp,
                native_component: &native_component_tmp,
            };
            // #####################

            match get_var_from_mem(var.to_owned(), condition, path, data, msg_data, sender) {
                Ok((lit, name, mem_type, path)) => {

                    let result = exec_path_actions(
                        lit,
                        condition,
                        None,
                        &path,
                        &ContentType::get(&lit),
                        &mut new_scope_data,
                        msg_data,
                        sender,
                    );
                    let (new_literal, update_mem) = match result {
                        Ok((lit, update)) => (lit, update),
                        Err(err) => (MSG::send_error_msg(&sender, msg_data, Err(err)), false),
                    };

                    save_literal_in_mem(
                        lit.to_owned(),
                        name,
                        &mem_type,
                        update_mem,
                        data,
                        msg_data,
                        sender,
                    );
                    Ok(new_literal)
                }
                Err(err) => {
                    // if value does not exist in memory we create a null value and we apply all the path actions
                    // if we are not in a condition an error message is created and send
                    let mut null = match condition {
                        true => PrimitiveNull::get_literal(err.position.interval),
                        false => MSG::send_error_msg(&sender, msg_data, Err(err)),
                    };

                    let path = if let Some(p) = path {
                        Some(resolve_path(p, condition, data, msg_data, sender)?)
                    } else {
                        None
                    };
                    let content_type = ContentType::get(&null);
                    let (new_literal, ..) = exec_path_actions(
                        &mut null,
                        condition,
                        None,
                        &path,
                        &content_type,
                        data,
                        msg_data,
                        sender,
                    )?;
                    Ok(new_literal)
                }
            }
        },
    }
}

pub fn get_var_from_mem<'a>(
    name: Identifier,
    condition: bool, // TODO: find better method than this
    path: Option<&[(Interval, PathState)]>,
    data: &'a mut Data,
    msg_data: &mut MessageData,
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
    let path = if let Some(p) = path {
        Some(resolve_path(p, condition, data, msg_data, sender)?)
    } else {
        None
    };

    match search_in_memory_type(&name, data)? {
        var if var == "use" => {
            let lit = get_var_from_step_var(&name, data)?;
            Ok((lit, name.ident, MemoryType::Use, path))
        }
        _ => {
            let lit = search_var_memory(name.clone(), data)?;
            Ok((lit, name.ident, MemoryType::Remember, path))
        }
    }
}

pub fn search_goto_var_memory<'a>(
    var: &GotoValueType,
    msg_data: &mut MessageData,
    data: &'a mut Data,
) -> Result<String, ErrorInfo> {
    match var {
        GotoValueType::Name(ident) => Ok(ident.ident.clone()),
        GotoValueType::Variable(ident) => {
            let (literal, ..) =
                get_var_from_mem(ident.clone(), false, None, data, msg_data, &None)?;

            Ok(Literal::get_value::<String>(
                &literal.primitive,
                literal.interval,
                format!("< {} > {}", ident.ident, ERROR_FIND_MEMORY),
            )?
            .to_owned())
        }
    }
}

pub fn get_string_from_complex_string(
    exprs: &[Expr],
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let mut new_string = String::new();

    //TODO: log error with span
    for elem in exprs.iter() {
        match expr_to_literal(elem, false, None, data, msg_data, sender) {
            Ok(var) => new_string.push_str(&var.primitive.to_string()),
            Err(err) => {
                return Err(err);
            }
        }
    }

    let mut result = PrimitiveString::get_literal(&new_string, interval);
    result.set_content_type("text");

    Ok(result)
}
