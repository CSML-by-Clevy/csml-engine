use crate::data::{ast::*, tokens::*};
use crate::parser::{
    parse_idents::get_string, parse_var_types::parse_expr_list, tools::get_interval,
};
use nom::{error::*, IResult};

pub fn parse_functions<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, interval) = get_interval(s)?;
    let (s, name) = get_string(s)?;
    let (s, expr) = parse_expr_list(s)?;
    let func = Function {
        name,
        interval,
        args: Box::new(expr),
    };

    Ok((s, Expr::ObjectExpr(ObjectType::Normal(func))))
}
