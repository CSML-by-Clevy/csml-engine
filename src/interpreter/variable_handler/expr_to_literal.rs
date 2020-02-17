use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::{if_statment::evaluate_condition, match_builtin},
    data::Data,
    message::{MessageData, MSG},
    variable_handler::{get_string_from_complexstring, get_var, interval::interval_from_expr},
};
use crate::parser::{ast::*, literal::Literal, tokens::*};
use crate::primitive::{array::PrimitiveArray, object::PrimitiveObject};
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::interpreter::json_to_rust::{Client, Context, Event};
//     use curl::easy::Easy;
//     use std::collections::HashMap;

//     fn gen_context() -> Context {
//         Context {
//             past: HashMap::new(),
//             current: HashMap::new(),
//             metadata: HashMap::new(),
//             retries: 0,
//             is_initial_step: false,
//             client: Client {
//                 bot_id: "none".to_owned(),
//                 channel_id: "none".to_owned(),
//                 user_id: "none".to_owned(),
//             },
//             fn_endpoint: "none".to_owned(),
//         }
//     }

//     fn gen_flow() -> Flow {
//         Flow {
//             flow_instructions: HashMap::new(),
//             flow_type: FlowType::Normal,
//         }
//     }

//     fn gen_data<'a>(
//         flow: &'a Flow,
//         context: &'a mut Context,
//         event: &'a Option<Event>,
//     ) -> Data<'a> {
//         Data::<'a> {
//             ast: flow,
//             memory: context,
//             event,
//             curl: Easy::new(),
//             step_vars: HashMap::new(),
//         }
//     }

//     fn gen_interval() -> Interval {
//         Interval { line: 0, column: 0 }
//     }

//     fn gen_range_interval() -> RangeInterval {
//         RangeInterval {
//             start: gen_interval(),
//             end: gen_interval(),
//         }
//     }

//     fn gen_int_literal(val: i64) -> Expr {
//         Expr::LitExpr(PrimitiveInt(val, gen_interval()))
//         // Expr::LitExpr(Literal::int(val, gen_interval()))
//     }

//     fn gen_str_literal(val: &str) -> Expr {
//         Expr::LitExpr(Literal::string(val.to_owned(), gen_interval()))
//     }

//     fn gen_array_expr(val: Vec<Expr>) -> Expr {
//         Expr::VecExpr(val, gen_range_interval())
//     }

//     #[test]
//     fn ok_complex_literal() {
//         let expr = Expr::ComplexLiteral(
//             vec![
//                 gen_int_literal(42),
//                 gen_str_literal(" != "),
//                 gen_int_literal(43),
//             ],
//             gen_range_interval(),
//         );
//         let mut context = gen_context();
//         let flow = gen_flow();
//         let mut data = gen_data(&flow, &mut context, &None);

//         match &expr_to_literal(&expr, &mut data) {
//             Ok(Literal::StringLiteral { value, .. }) if value == "42 != 43" => {}
//             e => panic!("{:?}", e),
//         }
//     }

//     #[test]
//     fn ok_objectexpr_literal() {
//         let expr = Expr::ObjectExpr(ObjectType::Normal(
//             Identifier {
//                 idents: "Object".to_owned(),
//                 interval: gen_interval(),
//                 index: None,
//             },
//             Box::new(gen_array_expr(vec![gen_int_literal(42)])),
//         ));
//         let mut context = gen_context();
//         let flow = gen_flow();
//         let mut data = gen_data(&flow, &mut context, &None);

//         match &expr_to_literal(&expr, &mut data) {
//             Ok(Literal::ObjectLiteral { properties, .. }) => match properties.get(DEFAULT) {
//                 Some(Literal::IntLiteral { value: 42, .. }) => {}
//                 e => panic!(" 2-> {:?}", e),
//             },
//             e => panic!(" 1-> {:?}", e),
//         }
//     }
// }
