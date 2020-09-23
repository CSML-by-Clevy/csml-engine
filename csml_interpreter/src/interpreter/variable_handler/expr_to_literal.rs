use crate::data::error_info::ErrorInfo;
use crate::data::literal::ContentType;
use crate::data::position::Position;
use crate::data::primitive::{PrimitiveArray, PrimitiveObject};
use crate::data::{
    ast::*, tokens::*, ArgsType, Context, Data, Literal, MemoryType, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::evaluate_condition,
    builtins::{match_builtin, match_native_builtin},
    interpret_function_scope,
    json_to_rust::interpolate,
    variable_handler::{
        exec_path_actions, get_string_from_complex_string, get_var, interval::interval_from_expr,
        resolve_path, save_literal_in_mem,
    },
};
use std::{collections::HashMap, sync::mpsc};

fn exec_path_literal(
    literal: &mut Literal,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    if let Some(path) = path {
        let path = resolve_path(path, data, msg_data, sender)?;
        let (mut new_literal, ..) = exec_path_actions(
            literal,
            None,
            &Some(path),
            &ContentType::get(&literal),
            msg_data,
            sender,
        )?;

        //TODO: remove this condition when 'root' and 'sender' can be access anywhere in the code
        if new_literal.content_type == "string" {
            let string = serde_json::json!(new_literal.primitive.to_string());
            new_literal = interpolate(&string, new_literal.interval, data, msg_data, sender)?;
        }

        Ok(new_literal)
    } else {
        Ok(literal.to_owned())
    }
}

fn normal_object_to_literal(
    name: &str,
    args: &Expr,
    interval: Interval,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    let args = resolve_fn_args(args, data, msg_data, sender)?;

    let result = match data
        .flow
        .flow_instructions
        .get_key_value(&InstructionType::FunctionStep {
            name: name.to_owned(),
            args: Vec::new(),
        }) {
        Some((i, e)) => Some((i.to_owned(), e.to_owned())),
        None => None,
    };

    match (
        data.native_component.contains_key(name),
        BUILT_IN.contains(&name),
        result,
    ) {
        (true, ..) => {
            let value = match_native_builtin(&name, args, interval.to_owned(), data);
            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        (_, true, _) => {
            let value = match_builtin(&name, args, interval.to_owned(), data, msg_data, sender);

            Ok(MSG::send_error_msg(&sender, msg_data, value))
        }

        (
            ..,
            Some((
                InstructionType::FunctionStep {
                    name: _,
                    args: fn_args,
                },
                expr,
            )),
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
            let mut new_scope_data = Data::new(
                &data.flow,
                &mut context,
                &data.event,
                HashMap::new(),
                &data.custom_component,
                &data.native_component,
            );

            for (index, name) in fn_args.iter().enumerate() {
                let value = args.get(name, index).unwrap();

                save_literal_in_mem(
                    value.to_owned(),
                    name.to_owned(),
                    &MemoryType::Use,
                    true,
                    &mut new_scope_data,
                    msg_data,
                    sender,
                );
            }

            match expr {
                Expr::Scope {
                    block_type: BlockType::Function,
                    scope,
                    range: RangeInterval { start, .. },
                } => interpret_function_scope(&scope, &mut new_scope_data, start),
                _ => panic!("error in parsing need to be expr scope"),
            }
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

pub fn expr_to_literal(
    expr: &Expr,
    condition: bool,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::As(name, var)) => {
            let value = expr_to_literal(var, false, None, data, msg_data, sender)?;
            data.step_vars.insert(name.ident.to_owned(), value.clone());
            Ok(value)
        }
        Expr::PathExpr { literal, path } => {
            expr_to_literal(literal, false, Some(path), data, msg_data, sender)
        }
        Expr::ObjectExpr(ObjectType::Normal(Function {
            name,
            args,
            interval,
        })) => {
            let mut literal =
                normal_object_to_literal(&name, args, *interval, data, msg_data, sender)?;

            exec_path_literal(&mut literal, path, data, msg_data, sender)
        }
        Expr::MapExpr(map, RangeInterval { start, .. }) => {
            let mut object = HashMap::new();

            for (key, value) in map.iter() {
                object.insert(
                    key.to_owned(),
                    expr_to_literal(&value, false, None, data, msg_data, sender)?,
                );
            }
            let mut literal = PrimitiveObject::get_literal(&object, start.to_owned());
            exec_path_literal(&mut literal, path, data, msg_data, sender)
        }
        Expr::ComplexLiteral(vec, RangeInterval { start, .. }) => {
            let mut string =
                get_string_from_complex_string(vec, start.to_owned(), data, msg_data, sender)?;
            exec_path_literal(&mut string, path, data, msg_data, sender)
        }
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, false, None, data, msg_data, sender)?)
            }
            let mut literal = PrimitiveArray::get_literal(&array, range.start.to_owned());
            exec_path_literal(&mut literal, path, data, msg_data, sender)
        }
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            evaluate_condition(infix, exp_1, exp_2, data, msg_data, sender)
        }
        Expr::LitExpr(literal) => {
            exec_path_literal(&mut literal.clone(), path, data, msg_data, sender)
        }
        Expr::IdentExpr(var, ..) => Ok(get_var(
            var.to_owned(),
            condition,
            path,
            data,
            msg_data,
            sender,
        )?),
        e => Err(gen_error_info(
            Position::new(interval_from_expr(e)),
            ERROR_EXPR_TO_LITERAL.to_owned(),
        )),
    }
}

pub fn resolve_fn_args(
    expr: &Expr,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<ArgsType, ErrorInfo> {
    match expr {
        Expr::VecExpr(vec, ..) => {
            let mut map = HashMap::new();
            let mut first = 0;
            let mut named_args = false;

            for (index, value) in vec.iter().enumerate() {
                match value {
                    Expr::ObjectExpr(ObjectType::Assign(name, var)) => {
                        let name = match **name {
                            Expr::IdentExpr(ref var, ..) => var,
                            _ => {
                                return Err(gen_error_info(
                                    Position::new(interval_from_expr(name)),
                                    "key must be of type string".to_owned(),
                                ))
                            }
                        };
                        named_args = true;

                        let literal = expr_to_literal(var, false, None, data, msg_data, sender)?;
                        map.insert(name.ident.to_owned(), literal);
                    }
                    expr => {
                        first += 1;
                        if named_args && first > 1 {
                            return Err(gen_error_info(
                                Position::new(interval_from_expr(expr)),
                                ERROR_EXPR_TO_LITERAL.to_owned(), // TODO: error mix of named args and anonymous args
                            ));
                        }
                        let literal = expr_to_literal(expr, false, None, data, msg_data, sender)?;
                        map.insert(format!("arg{}", index), literal);
                    }
                }
            }

            match named_args {
                true => Ok(ArgsType::Named(map)),
                false => Ok(ArgsType::Normal(map)),
            }
        }
        e => Err(gen_error_info(
            Position::new(interval_from_expr(e)),
            ERROR_EXPR_TO_LITERAL.to_owned(), //TODO: internal error fn args bad format
        )),
    }
}
