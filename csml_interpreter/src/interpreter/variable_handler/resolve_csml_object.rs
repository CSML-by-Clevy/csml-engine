use crate::data::{
    ast::*,
    data::{init_child_context, init_child_scope, Data},
    error_info::ErrorInfo,
    literal::create_error_info,
    primitive::PrimitiveClosure,
    tokens::*,
    warnings::DisplayWarnings,
    ArgsType, Literal, MemoryType, MessageData, Position, MSG,
};
use crate::error_format::*;
use crate::interpreter::{
    builtins::{match_builtin, match_native_builtin},
    function_scope::exec_fn_in_new_scope,
    variable_handler::resolve_fn_args,
    variable_handler::save_literal_in_mem,
};

use std::{collections::HashMap, sync::mpsc};

////////////////////////////////////////////////////////////////////////////////
// Local Struct
////////////////////////////////////////////////////////////////////////////////

enum ObjType {
    NativeComponent,
    BuiltIn,
    BuiltInWithoutWarnings,
    Function { fn_args: Vec<String>, scope: Expr },
    Import,
    Closure { fn_args: Vec<String>, scope: Expr },
    Error,
}
////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn get_function<'a>(
    flow: &'a Flow,
    fn_name: &str,
    original_name: &Option<String>,
) -> Option<(Vec<String>, Expr, &'a Flow)> {
    let name = match original_name {
        Some(original_name) => original_name.to_owned(),
        None => fn_name.to_owned(),
    };

    if let (InstructionScope::FunctionScope { name: _, args }, expr) = flow
        .flow_instructions
        .get_key_value(&InstructionScope::FunctionScope {
            name,
            args: Vec::new(),
        })?
    {
        return Some((args.to_owned(), expr.to_owned(), flow));
    }
    None
}

fn search_function<'a>(
    origin_flow_name: &str,
    bot_flows: &'a HashMap<String, Flow>,
    extern_flows: &'a HashMap<String, Flow>,
    import: &ImportScope,
) -> Result<(Vec<String>, Expr, &'a Flow), ErrorInfo> {
    match &import.from_flow {
        FromFlow::Normal(flow_name) => match bot_flows.get(flow_name) {
            Some(flow) => {
                let error_message = format!(
                    "function '{}' not found in '{}' flow",
                    import.name, flow_name
                );
                let error_info = create_error_info(&error_message, Interval::default());

                get_function(flow, &import.name, &import.original_name).ok_or(ErrorInfo {
                    position: Position::new(import.interval, origin_flow_name),
                    message: error_message,
                    additional_info: Some(error_info),
                })
            }
            None => {
                let error_message = format!(
                    "function '{}' not found in '{}' flow",
                    import.name, flow_name
                );
                let error_info = create_error_info(&error_message, Interval::default());

                Err(ErrorInfo {
                    position: Position::new(import.interval, origin_flow_name),
                    message: error_message,
                    additional_info: Some(error_info),
                })
            }
        },
        FromFlow::Extern(flow_name) => match extern_flows.get(flow_name) {
            Some(flow) => {
                let error_message = format!(
                    "function '{}' not found in '{}' flow",
                    import.name, flow_name
                );
                let error_info = create_error_info(&error_message, Interval::default());

                get_function(flow, &import.name, &import.original_name).ok_or(ErrorInfo {
                    position: Position::new(import.interval, origin_flow_name),
                    message: error_message,
                    additional_info: Some(error_info),
                })
            }
            None => {
                let error_message = format!(
                    "function '{}' not found in '{}' flow",
                    import.name, flow_name
                );
                let error_info = create_error_info(&error_message, Interval::default());

                Err(ErrorInfo {
                    position: Position::new(import.interval, origin_flow_name),
                    message: error_message,
                    additional_info: Some(error_info),
                })
            }
        },
        FromFlow::None => {
            for (_name, flow) in bot_flows.iter() {
                if let Some(values) = get_function(flow, &import.name, &import.original_name) {
                    return Ok(values);
                }
            }
            let error_message = format!("function '{}' not found in bot", import.name);
            let error_info = create_error_info(&error_message, Interval::default());

            Err(ErrorInfo {
                position: Position::new(import.interval, origin_flow_name),
                message: error_message,
                additional_info: Some(error_info),
            })
        }
    }
}

fn insert_args_in_scope_memory(
    new_scope_data: &mut Data,
    fn_args: &[String],
    args: &ArgsType,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) {
    for (index, name) in fn_args.iter().enumerate() {
        let value = args.get(name, index).unwrap();

        save_literal_in_mem(
            value.to_owned(),
            name.to_owned(),
            &MemoryType::Use,
            true,
            new_scope_data,
            msg_data,
            sender,
        );
    }
}

fn insert_memories_in_scope_memory(
    new_scope_data: &mut Data,
    memories: HashMap<String, Literal>,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) {
    for (name, value) in memories.iter() {
        save_literal_in_mem(
            value.to_owned(),
            name.to_owned(),
            &MemoryType::Use,
            true,
            new_scope_data,
            msg_data,
            sender,
        );
    }
}

fn check_for_function(name: &str, data: &Data) -> Option<(InstructionScope, Expr)> {
    match data
        .flow
        .flow_instructions
        .get_key_value(&InstructionScope::FunctionScope {
            name: name.to_owned(),
            args: Vec::new(),
        }) {
        Some((i, e)) => Some((i.to_owned(), e.to_owned())),
        None => None,
    }
}

fn check_for_import<'a>(
    name: &str,
    interval: Interval,
    data: &'a Data,
) -> Option<(Vec<String>, Expr, &'a Flow)> {
    match data
        .flow
        .flow_instructions
        .get_key_value(&InstructionScope::ImportScope(ImportScope {
            name: name.to_owned(),
            original_name: None,
            from_flow: FromFlow::None,
            interval: interval.clone(),
        })) {
        Some((InstructionScope::ImportScope(import), _expr)) => {
            match search_function(&data.context.flow, data.flows, data.extern_flows, import) {
                Ok((fn_args, expr, new_flow)) => Some((fn_args, expr, new_flow)), // if new_flow == data.flow {
                _err => None,
            }
        }
        _ => None,
    }
}

fn check_for_closure<'a>(
    name: &str,
    interval: Interval,
    data: &'a Data,
) -> Option<(Vec<String>, Expr)> {
    match data.step_vars.get(name) {
        Some(lit) => {
            let val = Literal::get_value::<PrimitiveClosure>(
                &lit.primitive,
                &data.context.flow,
                interval,
                "expect Literal of type [Closure]".to_owned(),
            )
            .ok()?
            .to_owned();
            Some((val.args, *val.func))
        }
        None => None,
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn get_type<'a>(name: &'a str, interval: Interval, data: &'a Data) -> ObjType {
    if data.native_component.contains_key(name) {
        return ObjType::NativeComponent;
    }

    if BUILT_IN.contains(&name) {
        return ObjType::BuiltIn;
    }

    if BUILT_IN_WITHOUT_WARNINGS.contains(&name) {
        return ObjType::BuiltInWithoutWarnings;
    }

    if let Some((
        InstructionScope::FunctionScope {
            name: _,
            args: fn_args,
        },
        scope,
    )) = check_for_function(name, data)
    {
        return ObjType::Function { fn_args, scope };
    }

    if let Some((_fn_args, _expr, _new_flow)) = check_for_import(name, interval, data) {
        return ObjType::Import;
    }

    if let Some((fn_args, scope)) = check_for_closure(name, interval, data) {
        return ObjType::Closure { fn_args, scope };
    }

    ObjType::Error
}

pub fn resolve_object(
    name: &str,
    args: &Expr,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match get_type(name, interval, data) {
        ObjType::NativeComponent => {
            let resolved_args =
                resolve_fn_args(args, data, msg_data, &DisplayWarnings::On, sender)?;

            let value = match_native_builtin(&name, resolved_args, interval.to_owned(), data);
            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        ObjType::BuiltIn => {
            let resolved_args =
                resolve_fn_args(args, data, msg_data, &DisplayWarnings::On, sender)?;

            let value = match_builtin(
                &name,
                resolved_args,
                interval.to_owned(),
                data,
                msg_data,
                sender,
            );

            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        ObjType::BuiltInWithoutWarnings => {
            let resolved_args =
                resolve_fn_args(args, data, msg_data, &DisplayWarnings::Off, sender)?;

            let value = match_builtin(
                &name,
                resolved_args,
                interval.to_owned(),
                data,
                msg_data,
                sender,
            );

            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        ObjType::Function { fn_args, scope } => {
            let resolved_args =
                resolve_fn_args(args, data, msg_data, &DisplayWarnings::On, sender)?;
            exec_fn(
                &scope,
                &fn_args,
                resolved_args,
                None,
                interval,
                data,
                msg_data,
                sender,
            )
        }

        ObjType::Import => {
            let resolved_args =
                resolve_fn_args(args, data, msg_data, &DisplayWarnings::On, sender)?;

            let error = gen_error_info(
                Position::new(interval, &data.context.flow),
                ERROR_FN_ARGS.to_owned(),
            );

            let (fn_args, expr, new_flow) =
                check_for_import(name, interval, data).ok_or(error.clone())?;

            if fn_args.len() > resolved_args.len() {
                return Err(error);
            }

            let mut context = init_child_context(&data);
            let mut step_count = data.step_count.clone();
            let mut new_scope_data = init_child_scope(data, &mut context, &mut step_count);
            new_scope_data.flow = new_flow;

            insert_args_in_scope_memory(
                &mut new_scope_data,
                &fn_args,
                &resolved_args,
                msg_data,
                sender,
            );

            exec_fn_in_new_scope(&expr, &mut new_scope_data, msg_data, sender)
        }

        ObjType::Closure { fn_args, scope } => {
            let resolved_args =
                resolve_fn_args(args, data, msg_data, &DisplayWarnings::On, sender)?;
            exec_fn(
                &scope,
                &fn_args,
                resolved_args,
                None,
                interval,
                data,
                msg_data,
                sender,
            )
        }

        ObjType::Error => {
            let err = gen_error_info(
                Position::new(interval, &data.context.flow),
                format!("{} [{}]", ERROR_BUILTIN_UNKNOWN, name),
            );

            Ok(MSG::send_error_msg(
                &sender,
                msg_data,
                Err(err) as Result<Literal, ErrorInfo>,
            ))
        }
    }
}

pub fn exec_fn(
    scope: &Expr,
    fn_args: &[String],
    args: ArgsType,
    memories_to_insert: Option<HashMap<String, Literal>>,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    if fn_args.len() > args.len() {
        return Err(gen_error_info(
            Position::new(interval, &data.context.flow),
            ERROR_FN_ARGS.to_owned(),
        ));
    }

    let mut context = init_child_context(&data);
    let mut step_count = data.step_count.clone();
    let mut new_scope_data = init_child_scope(data, &mut context, &mut step_count);
    insert_args_in_scope_memory(&mut new_scope_data, fn_args, &args, msg_data, sender);
    if let Some(memories) = memories_to_insert {
        insert_memories_in_scope_memory(&mut new_scope_data, memories, msg_data, sender);
    }

    exec_fn_in_new_scope(scope, &mut new_scope_data, msg_data, sender)
}
