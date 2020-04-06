use crate::data::literal::ContentType;
use crate::data::primitive::{array::PrimitiveArray, object::PrimitiveObject};
use crate::data::{ast::*, tokens::*, Data, Literal, MessageData, MSG};
use crate::data::error_info::ErrorInfo;
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::evaluate_condition,
    builtins::match_builtin,
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

fn format_function_args(
    args: &Expr,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<HashMap<String, Literal>, ErrorInfo> {
    let mut obj: HashMap<String, Literal> = HashMap::new();
    let vec = match args {
        Expr::VecExpr(vec, ..) => vec,
        _e => {
            return Err(ErrorInfo::new(
                interval_from_expr(args),
                ERROR_FUNCTIONS_ARGS.to_owned(),
            ))
        }
    };

    for elem in vec.iter() {
        match elem {
            Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
                let value = expr_to_literal(var, None, data, root, sender)?;

                // TODO: Add tow Assign types in ObjectType ?
                let ident = match **var_name {
                    Expr::IdentExpr(ref ident) => ident.ident.to_owned(),
                    _ => {
                        return Err(ErrorInfo::new(
                            interval_from_expr(var),
                            ERROR_ASSIGN_IDENT.to_owned(),
                        ))
                    }
                };
                obj.insert(ident, value);
            }
            Expr::ObjectExpr(ObjectType::Normal(Function {
                name,
                interval,
                args,
            })) => {
                let (_, literal) =
                    normal_object_to_literal(&name, args, *interval, data, root, sender)?;

                obj.insert(DEFAULT.to_owned(), literal);
            }
            _ => {
                let value = expr_to_literal(elem, None, data, root, sender)?;
                obj.insert(DEFAULT.to_owned(), value);
            }
        }
    }

    Ok(obj)
}

fn normal_object_to_literal(
    name: &str,
    value: &Expr,
    interval: Interval,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<(String, Literal), ErrorInfo> {
    let obj = format_function_args(value, data, root, sender)?;

    if BUILT_IN.contains(&name) {
        Ok((
            name.to_owned(),
            match_builtin(&name, obj, interval.to_owned(), data)?,
        ))
    } else {
        Err(ErrorInfo::new(
            interval.to_owned(),
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
            let (_name, literal) =
                normal_object_to_literal(&name, args, *interval, data, root, sender)?;

            exec_path_literal(&mut literal.clone(), path, data, root, sender)
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
                get_string_from_complex_string(vec, start.to_owned(), data, root, sender);
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
        e => Err(ErrorInfo::new(
            interval_from_expr(e),
            ERROR_EXPR_TO_LITERAL.to_owned(),
        )),
    }
}
