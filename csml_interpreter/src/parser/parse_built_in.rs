use crate::data::{ast::*, tokens::*};
// use crate::linter::Linter;
use crate::parser::tools::get_string;
use crate::parser::{
    parse_comments::comment, parse_var_types::parse_expr_list, tools::get_interval,
};
use nom::{error::*, sequence::preceded, IResult};

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_built_in<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;
    let (s, name) = get_string(s)?;

    let (s, expr) = preceded(comment, parse_expr_list)(s)?;

    let func = Function {
        name,
        interval,
        args: Box::new(expr),
    };

    Ok((s, Expr::ObjectExpr(ObjectType::BuiltIn(func))))
}
