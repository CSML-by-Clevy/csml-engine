use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::match_builtin,
    data::Data,
    variable_handler::{
        get_string_from_complexstring, get_var, get_var_from_ident, interval::interval_from_expr,
    },
};
use crate::parser::{ast::*, literal::Literal, tokens::*};
use std::collections::HashMap;

fn format_object_attributes(
    expr: &Expr,
    data: &mut Data,
) -> Result<HashMap<String, Literal>, ErrorInfo> {
    let mut obj: HashMap<String, Literal> = HashMap::new();
    let vec = match expr {
        Expr::VecExpr(vec, ..) => vec,
        _e => {
            return Err(ErrorInfo {
                message: format!("ERROR: Object attributes {:?} bad format", expr),
                interval: interval_from_expr(expr),
            })
        }
    };

    for elem in vec.iter() {
        match elem {
            Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
                let value = expr_to_literal(var, data)?;
                obj.insert(var_name.ident.to_owned(), value);
            }
            Expr::ObjectExpr(ObjectType::Normal(name, value)) => {
                let interval = interval_from_expr(elem);
                let (_, literal) = normal_object_to_literal(&name.ident, value, &interval, data)?;

                obj.insert(DEFAULT.to_owned(), literal);
            }
            _ => {
                let value = expr_to_literal(elem, data)?;
                obj.insert(DEFAULT.to_owned(), value);
            }
        }
    }

    Ok(obj)
}

fn normal_object_to_literal(
    name: &str,
    value: &Expr,
    interval: &Interval,
    data: &mut Data,
) -> Result<(String, Literal), ErrorInfo> {
    let obj = format_object_attributes(value, data)?;

    if BUILT_IN.contains(&name) {
        Ok((
            name.to_owned(),
            match_builtin(&name, obj, interval.to_owned(), data)?,
        ))
    } else {
        Err(ErrorInfo {
            message: format!("ERROR: unknown function {}", name),
            interval: interval.to_owned(),
        })
    }
}

pub fn expr_to_literal(expr: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::As(name, var)) => {
            let value = expr_to_literal(var, data)?;
            data.step_vars.insert(name.ident.to_owned(), value.clone());
            Ok(value)
        }
        Expr::ObjectExpr(ObjectType::Normal(name, value)) => {
            let interval = interval_from_expr(expr);
            let (_name, literal) =
                normal_object_to_literal(&name.ident, value, &interval, data)?;

            Ok(literal)
        }
        Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
            let value = expr_to_literal(var, data)?;
            Ok(Literal::name_object(
                var_name.ident.to_owned(),
                &value,
                interval_from_expr(expr),
            ))
        }
        Expr::BuilderExpr(..) => get_var_from_ident(expr, data),
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data)?)
            }

            Ok(Literal::array(array, range.start.to_owned()))
        }
        Expr::IdentExpr(var, ..) => Ok(get_var(var.to_owned(), data)?),
        Expr::LitExpr(literal) => Ok(literal.clone()),
        e => Err(ErrorInfo {
            message: format!("ERROR: Expr {:?} can't be converted to Literal", expr),
            interval: interval_from_expr(e),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::json_to_rust::{Client, Context, Event};
    use multimap::MultiMap;

    fn gen_context() -> Context {
        Context {
            past: MultiMap::new(),
            current: MultiMap::new(),
            metadata: MultiMap::new(),
            retries: 0,
            is_initial_step: false,
            client: Client {
                bot_id: "none".to_owned(),
                channel_id: "none".to_owned(),
                user_id: "none".to_owned(),
            },
            fn_endpoint: "none".to_owned(),
        }
    }

    fn gen_flow() -> Flow {
        Flow {
            flow_instructions: HashMap::new(),
        }
    }

    fn gen_data<'a>(flow: &'a Flow, context: &'a Context, event: &'a Option<Event>) -> Data<'a> {
        Data::<'a> {
            ast: flow,
            memory: context,
            event,
            step_vars: HashMap::new(),
        }
    }

    fn gen_interval() -> Interval {
        Interval { line: 0, column: 0 }
    }

    fn gen_range_interval() -> RangeInterval {
        RangeInterval {
            start: gen_interval(),
            end: gen_interval(),
        }
    }

    fn gen_int_literal(val: i64) -> Expr {
        Expr::LitExpr(Literal::int(val, gen_interval()))
    }

    fn gen_str_literal(val: &str) -> Expr {
        Expr::LitExpr(Literal::string(val.to_owned(), gen_interval()))
    }

    fn gen_array_expr(val: Vec<Expr>) -> Expr {
        Expr::VecExpr(val, gen_range_interval())
    }

    #[test]
    fn ok_complex_literal() {
        let expr = Expr::ComplexLiteral(
            vec![
                gen_int_literal(42),
                gen_str_literal(" != "),
                gen_int_literal(43),
            ],
            gen_range_interval(),
        );
        let context = gen_context();
        let flow = gen_flow();
        let mut data = gen_data(&flow, &context, &None);

        match &expr_to_literal(&expr, &mut data) {
            Ok(Literal::StringLiteral { value, .. }) if value == "42 != 43" => {}
            e => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_objectexpr_literal() {
        let expr = Expr::ObjectExpr(ObjectType::Normal(
            Identifier {
                ident: "Object".to_owned(),
                interval: gen_interval(),
                index: None,
            },
            Box::new(gen_array_expr(vec![gen_int_literal(42)])),
        ));
        let context = gen_context();
        let flow = gen_flow();
        let mut data = gen_data(&flow, &context, &None);

        match &expr_to_literal(&expr, &mut data) {
            Ok(Literal::ObjectLiteral { properties, .. }) => match properties.get(DEFAULT) {
                Some(Literal::IntLiteral { value: 42, .. }) => {}
                e => panic!(" 2-> {:?}", e),
            },
            e => panic!(" 1-> {:?}", e),
        }
    }

}
