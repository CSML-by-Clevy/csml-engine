use crate::data::{ast::*, tokens::*};
use crate::parser::operator::parse_operator;
use crate::parser::tools::get_string;
use crate::parser::{
    parse_comments::comment, parse_var_types::parse_expr_list, tools::get_interval,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    error::{ParseError, ContextError},
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
fn parse_index<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Interval, PathState), E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let (s, interval) = get_interval(s)?;

    let (s, path) = terminated(
        preceded(tag(L_BRACKET), parse_operator),
        preceded(comment, tag(R_BRACKET)),
    )(s)?;

    Ok((s, (interval, PathState::ExprIndex(path))))
}

fn parse_dot_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Interval, PathState), E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    // let (s, found) = take_while(|c| "\n".contains(c))(s)?;
    // let (s, _) = match found.fragment().is_empty() {
    //     true => (s, Span::new("")),
    //     false => take_while(|c| WHITE_SPACE.contains(c))(s)?,
    // };

    let (s, _) = tag(DOT)(s)?;
    let (s, interval) = get_interval(s)?;
    let (s, name) = get_string(s)?;
    match parse_expr_list(s) as IResult<Span<'a>, Expr, E> {
        Ok((s, args)) => Ok((
            s,
            (
                interval,
                PathState::Func(Function {
                    name,
                    interval,
                    args: Box::new(args),
                }),
            ),
        )),
        _ => Ok((s, (interval, PathState::StringIndex(name)))),
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_path<'a, E>(s: Span<'a>, expr: Expr) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>> + ContextError<Span<'a>>,
{
    let path: IResult<Span<'a>, Vec<(Interval, PathState)>, E> = many1(
        alt((
            parse_index,
            preceded(comment, parse_dot_path)
        ))
    )(s);

    match path {
        Ok((s, path)) => Ok((
            s,
            Expr::PathExpr {
                literal: Box::new(expr),
                path,
            },
        )),
        Err(Err::Error(..)) | Err(Err::Failure(..)) => Ok((s, expr)),
        Err(Err::Incomplete(e)) => Err(Err::Incomplete(e)),
    }
}
