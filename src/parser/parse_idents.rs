use crate::data::{ast::*, tokens::*};
use crate::parser::parse_path::parse_path;
use crate::parser::tools::get_string;
use crate::parser::tools::get_tag;
use crate::parser::{parse_comments::comment, tools::get_interval};
use nom::Err::Failure;
use nom::{
    combinator::opt,
    error::{ErrorKind, ParseError},
    sequence::preceded,
    *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn form_idents(
    ident: String,
    path: Option<Vec<(Interval, PathExpr)>>,
    position: Interval,
) -> Identifier {
    Expr::new_idents(ident, position, path)
}

fn parse_idents<'a, E>(
    s: Span<'a>,
    interval: Interval,
    reserved: &[&str],
    var: &str,
) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    if reserved.contains(&&(*var.to_ascii_lowercase())) {
        return Err(Err::Error(E::add_context(
            s,
            "reserved keyword can't be used as identifier",
            E::from_error_kind(s, ErrorKind::Tag),
        )));
    }

    if var.parse::<f64>().is_ok() {
        return Err(Err::Error(E::add_context(
            s,
            "int/float can't be used as identifier",
            E::from_error_kind(s, ErrorKind::Tag),
        )));
    }

    Ok((s, form_idents(var.to_owned(), None, interval)))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_idents_utilisation<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    parse_idents(s, position, UTILISATION_RESERVED, &var)?;

    let (s, path) = opt(parse_path)(s)?;

    Ok((s, form_idents(var, path, position)))
}

pub fn parse_idents_assignation_without_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    parse_idents(s, position, ASSIGNATION_RESERVED, &var)?;

    Ok((s, form_idents(var, None, position)))
}

pub fn parse_idents_assignation_with_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    parse_idents(s, position, ASSIGNATION_RESERVED, &var)?;

    let (s, path) = opt(parse_path)(s)?;

    Ok((s, form_idents(var, path, position)))
}

pub fn parse_idents_as<'a, E>(s: Span<'a>, expr: Expr) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let arg: IResult<Span<'a>, String, E> = preceded(comment, get_string)(s);

    match arg {
        Err(_) => Ok((s, expr)),
        Ok((s2, tmp)) => {
            match preceded(get_tag(tmp, AS), parse_idents_assignation_without_path)(s2) {
                Ok((s, name)) => (Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr)))))),
                Err(err) => match err {
                    Failure(err) => Err(Failure(err)),
                    _ => Ok((s, expr)),
                },
            }
        }
    }
}
