use crate::parser::{
    ast::*,
    // parse_object::string,
    // parse_literal::get_int,
    expressions_evaluation::operator_precedence,
    parse_comments::comment,
    parse_var_types::parse_expr_list,
    tokens::*,
    tools::get_interval,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    combinator::opt,
    error::{ErrorKind, ParseError},
    multi::many1,
    sequence::{preceded, terminated},
    *,
};

// fn index_number<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, PathExpr, E> {
//     let (s, int) = preceded(comment, get_int)(s)?;
//     Ok((s, PathExpr::ExprIndex(int as usize)))
// }

// fn index_string<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, PathExpr, E> {
//     let (s, string) = preceded(comment, string)(s)?;
//     Ok((s, PathExpr::StringIndex(String::from(string.fragment))))
// }

// ["string"]
// [ number ]
// [ number + number ]
fn parse_index<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Interval, PathExpr), E> {
    let (s, interval) = get_interval(s)?;

    let (s, path) = terminated(
        preceded(tag(L_BRACKET), operator_precedence),
        preceded(comment, tag(R_BRACKET)),
    )(s)?;

    Ok((s, (interval, PathExpr::ExprIndex(path))))
}

//.string
//.func (expr, ..)
fn parse_dot_path<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Interval, PathExpr), E> {
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

fn parse_path<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Vec<(Interval, PathExpr)>, E> {
    many1(alt((parse_index, parse_dot_path)))(s)
}

pub fn is_valid_char(input: char) -> bool {
    input != UNDERSCORE && !input.is_alphanumeric()
}

pub fn form_idents(
    ident: String,
    path: Option<Vec<(Interval, PathExpr)>>,
    position: Interval,
) -> Identifier {
    Expr::new_idents(ident, position, path)
}

pub fn get_tag<I, E: ParseError<I>>(
    var: String,
    tag: &str,
) -> impl Fn(I) -> IResult<I, (), E> + '_ {
    move |input: I| {
        if var == tag {
            Ok((input, ()))
        } else {
            Err(Err::Error(E::from_error_kind(input, ErrorKind::Tag)))
        }
    }
}

pub fn get_string<'a, E>(s: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, val) = take_till1(is_valid_char)(s)?;

    // TODO: see if return String can be &str ?
    Ok((s, val.fragment.to_owned()))
}

pub fn parse_idents_no_check<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;
    let (s, path) = opt(parse_path)(s)?;

    Ok((s, form_idents(var, path, position)))
}

pub fn parse_idents<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    // TODO: change check to another fn ??
    if RESERVED.contains(&&(*var)) {
        return Err(Err::Failure(E::add_context(
            s,
            "reserved keyword can't be used as identifier",
            E::from_error_kind(s, ErrorKind::Tag),
        )));
    }
    let (s, path) = opt(parse_path)(s)?;

    Ok((s, form_idents(var, path, position)))
}

pub fn parse_idents_expr<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, idents) = parse_idents_no_check(s)?;
    Ok((s, Expr::IdentExpr(idents)))
}

// // ??????????
// fn parse_box_expr<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Box<Expr>, E>
// where
//     E: ParseError<Span<'a>>,
// {
//     let (s, expr) = parse_var_expr(s)?;
//     Ok((s, Box::new(expr)))
// }
