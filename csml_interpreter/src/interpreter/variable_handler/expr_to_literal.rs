use crate::data::error_info::ErrorInfo;
use crate::data::literal::ContentType;
use crate::data::primitive::{closure::capture_variables, PrimitiveArray, PrimitiveObject};
use crate::data::Position;
use crate::data::{ast::*, ArgsType, Data, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::evaluate_condition,
    json_to_rust::interpolate,
    variable_handler::{
        exec_path_actions, get_string_from_complex_string, get_var, interval::interval_from_expr,
        resolve_csml_object::resolve_object, resolve_path,
        operations::{evaluate_postfix},
    },
};
use std::{collections::HashMap, sync::mpsc};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn exec_path_literal(
    literal: &mut Literal,
    condition: bool,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    msg_data: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    if let Some(path) = path {
        let path = resolve_path(path, condition, data, msg_data, sender)?;
        let (mut new_literal, ..) = exec_path_actions(
            literal,
            condition,
            None,
            &Some(path),
            &ContentType::get(&literal),
            data,
            msg_data,
            sender,
        )?;

        if new_literal.content_type == "string" {
            let string = serde_json::json!(new_literal.primitive.to_string());
            new_literal = interpolate(&string, new_literal.interval, data, msg_data, sender)?;
        }

        Ok(new_literal)
    } else {
        Ok(literal.to_owned())
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

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
            let value = expr_to_literal(var, condition, None, data, msg_data, sender)?;
            data.step_vars.insert(name.ident.to_owned(), value.clone());
            Ok(value)
        }
        Expr::PathExpr { literal, path } => {
            expr_to_literal(literal, condition, Some(path), data, msg_data, sender)
        }
        Expr::ObjectExpr(ObjectType::BuiltIn(Function {
            name,
            args,
            interval,
        })) => {
            let mut literal = resolve_object(&name, args, *interval, data, msg_data, sender)?;

            exec_path_literal(&mut literal, condition, path, data, msg_data, sender)
        }
        Expr::MapExpr {
            object,
            interval: range_interval,
            ..
        } => {
            let mut map = HashMap::new();

            for (key, value) in object.iter() {
                map.insert(
                    key.to_owned(),
                    expr_to_literal(&value, condition, None, data, msg_data, sender)?,
                );
            }
            let mut literal = PrimitiveObject::get_literal(&map, range_interval.to_owned());
            exec_path_literal(&mut literal, condition, path, data, msg_data, sender)
        }
        Expr::ComplexLiteral(vec, range_interval) => {
            let mut string = get_string_from_complex_string(
                vec,
                range_interval.to_owned(),
                data,
                msg_data,
                sender,
            )?;
            exec_path_literal(&mut string, condition, path, data, msg_data, sender)
        }
        Expr::VecExpr(vec, range_interval) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(
                    value, condition, None, data, msg_data, sender,
                )?)
            }
            let mut literal = PrimitiveArray::get_literal(&array, range_interval.to_owned());
            exec_path_literal(&mut literal, condition, path, data, msg_data, sender)
        }
        Expr::PostfixExpr(postfix, expr) => {
            let mut literal = evaluate_postfix(postfix, expr, data, msg_data, sender)?;
            exec_path_literal(&mut literal, condition, path, data, msg_data, sender)
        }
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            let mut literal = evaluate_condition(infix, exp_1, exp_2, data, msg_data, sender)?;
            exec_path_literal(&mut literal, condition, path, data, msg_data, sender)
        }
        Expr::LitExpr { literal, .. } => {
            let mut new_value = exec_path_literal(
                &mut literal.clone(),
                condition,
                path,
                data,
                msg_data,
                sender,
            )?;
            // only for closure capture the step variables
            let memory: HashMap<String, Literal> = data.get_all_memories();
            capture_variables(&mut &mut new_value, memory, &data.context.flow);
            Ok(new_value)
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
            Position::new(interval_from_expr(e), &data.context.flow),
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
                    Expr::ObjectExpr(ObjectType::Assign(_assign_type, name, var)) => {
                        let name = match **name {
                            Expr::IdentExpr(ref var, ..) => var,
                            _ => {
                                return Err(gen_error_info(
                                    Position::new(interval_from_expr(name), &data.context.flow),
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
                                Position::new(interval_from_expr(expr), &data.context.flow),
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
            Position::new(interval_from_expr(e), &data.context.flow),
            ERROR_EXPR_TO_LITERAL.to_owned(), //TODO: internal error fn args bad format
        )),
    }
}
