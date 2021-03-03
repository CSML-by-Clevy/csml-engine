use crate::data::{ast::*, tokens::*, warnings::{WARNING_OBJECT, WARNING_FN}};
// use crate::linter::Linter;
use crate::data::warnings::Warnings;
use crate::parser::tools::get_string;
use crate::parser::{parse_var_types::parse_expr_list, tools::get_interval};
use nom::{error::*, IResult};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_built_in<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, name) = get_string(s)?;

    if name == "Object" {
        Warnings::add(WARNING_OBJECT, interval);
    }
    if name == FN {
        Warnings::add(WARNING_FN, interval);
    }

    let (s, expr) = parse_expr_list(s)?;

    let func = Function {
        name,
        interval,
        args: Box::new(expr),
    };

    Ok((s, Expr::ObjectExpr(ObjectType::Normal(func))))
}
