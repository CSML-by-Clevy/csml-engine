use crate::data::primitive::{array::PrimitiveArray, object::PrimitiveObject};
use crate::data::{ast::*, tokens::*, Data, Literal, MessageData, MSG};
use crate::error_format::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::evaluate_condition,
    builtins::match_builtin,
    variable_handler::{get_string_from_complexstring, get_var, interval::interval_from_expr},
};
use std::{collections::HashMap, sync::mpsc};

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
            return Err(ErrorInfo {
                message: format!("Object attributes {:?} bad format", args),
                interval: interval_from_expr(args),
            })
        }
    };

    for elem in vec.iter() {
        match elem {
            Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
                let value = expr_to_literal(var, data, root, sender)?;
                obj.insert(var_name.ident.to_owned(), value);
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
                let value = expr_to_literal(elem, data, root, sender)?;
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
        Err(ErrorInfo {
            message: format!("Unknown function {}", name),
            interval: interval.to_owned(),
        })
    }
}

pub fn expr_to_literal(
    expr: &Expr,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::As(name, var)) => {
            let value = expr_to_literal(var, data, root, sender)?;
            data.step_vars.insert(name.ident.to_owned(), value.clone());
            Ok(value)
        }
        Expr::ObjectExpr(ObjectType::Normal(Function {
            name,
            args,
            interval,
        })) => {
            let (_name, literal) =
                normal_object_to_literal(&name, args, *interval, data, root, sender)?;

            Ok(literal)
        }
        Expr::MapExpr(map, RangeInterval { start, .. }) => {
            let mut object = HashMap::new();

            for (key, value) in map.iter() {
                object.insert(key.to_owned(), expr_to_literal(&value, data, root, sender)?);
            }
            Ok(PrimitiveObject::get_literal(
                "object",
                &object,
                start.to_owned(),
            ))
        }
        Expr::ComplexLiteral(vec, RangeInterval { start, .. }) => Ok(
            get_string_from_complexstring(vec, start.to_owned(), data, root, sender),
        ),
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data, root, sender)?)
            }

            Ok(PrimitiveArray::get_literal(
                "array",
                &array,
                range.start.to_owned(),
            ))
        }
        Expr::IdentExpr(var, ..) => Ok(get_var(var.to_owned(), data, root, sender)?),
        Expr::LitExpr(literal) => Ok(literal.clone()),
        // TODO: duplicate of get_var_from_ident in variable_handler
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            evaluate_condition(infix, exp_1, exp_2, data, root, sender)
        }
        e => Err(ErrorInfo {
            // expr
            message: "Expr can't be converted to Literal".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}
