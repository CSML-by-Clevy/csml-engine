use crate::data::{ast::*, tokens::*};
use crate::parser::operator::parse_operator;
use crate::parser::tools::get_string;
use crate::parser::{
    parse_comments::comment, parse_var_types::parse_expr_list, tools::get_interval,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    error::ParseError,
    multi::many1,
    sequence::{preceded, terminated},
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

// ["string"]
// [ number ]
// [ number + number ]
fn parse_index<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Interval, PathExpr), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;

    let (s, path) = terminated(
        preceded(tag(L_BRACKET), parse_operator),
        preceded(comment, tag(R_BRACKET)),
    )(s)?;

    Ok((s, (interval, PathExpr::ExprIndex(path))))
}

//.string
//.func (expr, ..)
fn parse_dot_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Interval, PathExpr), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(DOT)(s)?;
    let (s, interval) = get_interval(s)?;
    let (s, name) = get_string(s)?;
    let tmp: IResult<Span<'a>, Expr, E> = parse_expr_list(s);
    match tmp {
        Ok((s, args)) => Ok((
            s,
            (
                interval,
                PathExpr::Func(Function {
                    name,
                    interval,
                    args: Box::new(args),
                }),
            ),
        )),
        _ => Ok((s, (interval, PathExpr::StringIndex(name)))),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<(Interval, PathExpr)>, E>
where
    E: ParseError<Span<'a>>,
{
    many1(alt((parse_index, parse_dot_path)))(s)
}
