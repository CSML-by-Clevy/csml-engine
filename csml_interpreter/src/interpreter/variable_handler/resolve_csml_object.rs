use crate::data::{
    ast::*, error_info::ErrorInfo, tokens::*, ArgsType, Context, Data, Literal, MemoryType,
    MessageData, Position, MSG,
};
use crate::error_format::*;
use crate::imports::search_function;
use crate::interpreter::{
    builtins::{match_builtin, match_native_builtin},
    function_scope::exec_fn_in_new_scope,
    variable_handler::resolve_fn_args,
    variable_handler::save_literal_in_mem,
};

use std::{collections::HashMap, sync::mpsc};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn init_new_scope<'a>(data: &'a Data, context: &'a mut Context) -> Data<'a> {
    Data::new(
        &data.flows,
        &data.flow,
        context,
        &data.event,
        HashMap::new(),
        &data.custom_component,
        &data.native_component,
    )
}

fn insert_args_in_scope_memory(
    new_scope_data: &mut Data,
    fn_args: &Vec<String>,
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
            from_flow: None,
            position: Position::new(interval.clone()),
        })) {
        Some((InstructionScope::ImportScope(import), _expr)) => {
            match search_function(data.flows, import) {
                Ok((fn_args, expr, new_flow)) => Some((fn_args, expr, new_flow)), // if new_flow == data.flow {
                _err => None,
            }
        }
        _ => None,
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn resolve_object(
    name: &str,
    args: &Expr,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let args = resolve_fn_args(args, data, msg_data, sender)?;

    let function = check_for_function(name, data);
    let import = check_for_import(name, interval, data);

    match (
        data.native_component.contains_key(name),
        BUILT_IN.contains(&name),
        function,
        import,
    ) {
        (true, ..) => {
            let value = match_native_builtin(&name, args, interval.to_owned(), data);
            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        (_, true, ..) => {
            let value = match_builtin(&name, args, interval.to_owned(), data, msg_data, sender);

            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        (
            ..,
            Some((
                InstructionScope::FunctionScope {
                    name: _,
                    args: fn_args,
                },
                expr,
            )),
            _,
        ) => {
            if fn_args.len() > args.len() {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_FN_ARGS.to_owned(),
                ));
            }

            let mut context = Context {
                current: HashMap::new(),
                metadata: HashMap::new(),
                api_info: data.context.api_info.clone(),
                hold: None,
                step: data.context.step.clone(),
                flow: data.context.flow.clone(),
            };

            let mut new_scope_data = init_new_scope(data, &mut context);

            insert_args_in_scope_memory(&mut new_scope_data, &fn_args, &args, msg_data, sender);

            exec_fn_in_new_scope(expr, &mut new_scope_data, msg_data, sender)
        }

        (.., Some((fn_args, expr, new_flow))) => {
            if fn_args.len() > args.len() {
                return Err(gen_error_info(
                    Position::new(interval),
                    ERROR_FN_ARGS.to_owned(),
                ));
            }

            let mut context = Context {
                current: HashMap::new(),
                metadata: HashMap::new(),
                api_info: data.context.api_info.clone(),
                hold: None,
                step: data.context.step.clone(),
                flow: data.context.flow.clone(),
            };

            let mut new_scope_data = init_new_scope(data, &mut context);
            new_scope_data.flow = new_flow;

            insert_args_in_scope_memory(&mut new_scope_data, &fn_args, &args, msg_data, sender);

            exec_fn_in_new_scope(expr, &mut new_scope_data, msg_data, sender)
        }

        _ => {
            let err = gen_error_info(
                Position::new(interval),
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
