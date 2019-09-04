use std::collections::HashMap;
use crate::parser::{ast::*, tokens::*};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    ast_interpreter::match_builtin,
    variable_handler::{
        get_var,
        get_var_from_ident,
        get_string_from_complexstring,
        interval::{
            interval_from_expr,
        },
    },
};

fn format_object_attributes(expr: &Expr, data: &mut Data) -> Result<HashMap<String, Literal>, ErrorInfo> {
    let mut obj: HashMap<String, Literal> = HashMap::new();
    let vec = match expr {
        Expr::VecExpr(vec, ..) => vec,
        _e                     => return Err(
            ErrorInfo{
                message: format!("ERROR: Object attributes {:?} bad format", expr),
                interval: interval_from_expr(expr)
            }
        )
    };

    for elem in vec.iter() {
        match elem {
            Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
                let value = expr_to_literal(var, data)?;
                obj.insert(var_name.ident.to_owned(), value);
            }
            Expr::ObjectExpr(ObjectType::Normal(name, value)) => {
                let interval = interval_from_expr(elem);                
                let (name, literal) = normal_object_to_literal(&name.ident, value, interval, data)?;

                obj.insert(name, literal);
            }
            _ => {
                let value = expr_to_literal(elem, data)?;
                obj.insert("default".to_owned(), value);
            }
        }
    }

    Ok(obj)
}

fn normal_object_to_literal(name: &str, value: &Expr, interval: Interval , data: &mut Data) -> Result<(String, Literal), ErrorInfo> {
    let obj = format_object_attributes(value, data)?;

    if BUILT_IN.contains(&name) {
        Ok(
            (name.to_owned(), match_builtin(&name, obj, interval.to_owned(), data)?)
        )
    } 
    else {
        Err( 
            ErrorInfo{
                message: format!("ERROR: unknown function {}", name),
                interval: interval.to_owned()
            }
        )
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
            let (_name, literal) = normal_object_to_literal(&name.ident, value, interval.to_owned(), data)?;
            
            Ok(literal)
        }
        Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
            let value = expr_to_literal(var, data)?;
            Ok(Literal::name_object(var_name.ident.to_owned(), &value, interval_from_expr(expr)))
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
        e => Err(
            ErrorInfo{
                message: format!("ERROR: Expr {:?} can't be converted to Literal", expr),
                interval: interval_from_expr(e)
            }
        )
    }
}
