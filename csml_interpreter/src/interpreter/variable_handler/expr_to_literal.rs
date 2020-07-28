use crate::data::error_info::ErrorInfo;
use crate::data::literal::ContentType;
use crate::data::position::Position;
use crate::data::primitive::{array::PrimitiveArray, object::PrimitiveObject};
use crate::data::{ast::*, tokens::*, ArgsType, Data, Literal, MessageData, MSG};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::evaluate_condition,
    builtins::{match_builtin, match_native_builtin},
    variable_handler::{
        exec_path_actions, get_string_from_complex_string, get_var, interval::interval_from_expr,
        resolve_path,
    },
};
use std::{collections::HashMap, sync::mpsc};

fn exec_path_literal(
    literal: &mut Literal,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    if let Some(path) = path {
        let path = resolve_path(path, data, root, sender)?;
        let (new_literal, ..) =
            exec_path_actions(literal, None, &Some(path), &ContentType::get(&literal))?;
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
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<(String, Literal), ErrorInfo> {
    let args = resolve_fn_args(args, data, root, sender)?;

    if data.native_component.contains_key(name) {
        // add fn type error
        Ok((
            name.to_owned(),
            match_native_builtin(&name, args, interval.to_owned(), data)?,
        ))
    } else if BUILT_IN.contains(&name) {
        Ok((
            name.to_owned(),
            match_builtin(&name, args, interval.to_owned(), data)?,
        ))
    } else {
        Err(gen_error_info(
            Position::new(interval),
            format!("{} [{}]", name, ERROR_BUILTIN_UNKNOWN),
        ))
    }
}

pub fn expr_to_literal(
    expr: &Expr,
    path: Option<&[(Interval, PathState)]>,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::As(name, var)) => {
            let value = expr_to_literal(var, None, data, root, sender)?;
            data.step_vars.insert(name.ident.to_owned(), value.clone());
            Ok(value)
        }
        Expr::PathExpr { literal, path } => {
            expr_to_literal(literal, Some(path), data, root, sender)
        }
        Expr::ObjectExpr(ObjectType::Normal(Function {
            name,
            args,
            interval,
        })) => {
            let (_name, mut literal) =
                normal_object_to_literal(&name, args, *interval, data, root, sender)?;

            exec_path_literal(&mut literal, path, data, root, sender)
        }
        Expr::MapExpr(map, RangeInterval { start, .. }) => {
            let mut object = HashMap::new();

            for (key, value) in map.iter() {
                object.insert(
                    key.to_owned(),
                    expr_to_literal(&value, None, data, root, sender)?,
                );
            }
            let mut literal = PrimitiveObject::get_literal(&object, start.to_owned());
            exec_path_literal(&mut literal, path, data, root, sender)
        }
        Expr::ComplexLiteral(vec, RangeInterval { start, .. }) => {
            let mut string =
                get_string_from_complex_string(vec, start.to_owned(), data, root, sender)?;
            exec_path_literal(&mut string, path, data, root, sender)
        }
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, None, data, root, sender)?)
            }
            let mut literal = PrimitiveArray::get_literal(&array, range.start.to_owned());
            exec_path_literal(&mut literal, path, data, root, sender)
        }
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            evaluate_condition(infix, exp_1, exp_2, data, root, sender)
        }
        Expr::LitExpr(literal) => exec_path_literal(&mut literal.clone(), path, data, root, sender),
        Expr::IdentExpr(var, ..) => Ok(get_var(var.to_owned(), path, data, root, sender)?),
        e => Err(gen_error_info(
            Position::new(interval_from_expr(e)),
            ERROR_EXPR_TO_LITERAL.to_owned(),
        )),
    }
}

pub fn resolve_fn_args(
    expr: &Expr,
    data: &mut Data,
    root: &mut MessageData,
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

                        let literal = expr_to_literal(var, None, data, root, sender)?;
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
                        let literal = expr_to_literal(expr, None, data, root, sender)?;
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
