use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_error, ERROR_NUMBER_AS_IDENT, ERROR_RESERVED, ERROR_SIZE_IDENT};
use crate::parser::{
    parse_comments::comment,
    tools::get_interval,
    tools::{get_string, get_tag},
};
use nom::{error::ParseError, sequence::preceded, Err::*, *};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn form_idents(ident: String, position: Interval) -> Identifier {
    Expr::new_idents(ident, position)
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
        return Err(gen_nom_error(s, ERROR_RESERVED));
    }

    if var.len() > std::u8::MAX as usize {
        return Err(gen_nom_error(s, ERROR_SIZE_IDENT));
    }

    if var.parse::<f64>().is_ok() {
        return Err(gen_nom_error(s, ERROR_NUMBER_AS_IDENT));
    }

    Ok((s, form_idents(var.to_owned(), interval)))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_idents_usage<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    parse_idents(s, position, UTILISATION_RESERVED, &var)
}

pub fn parse_idents_assignation<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = get_interval(s)?;
    let (s, var) = preceded(comment, get_string)(s)?;

    parse_idents(s, position, ASSIGNATION_RESERVED, &var)
}

pub fn parse_idents_as<'a, E>(s: Span<'a>, expr: Expr) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let arg: IResult<Span<'a>, String, E> = preceded(comment, get_string)(s);

    match arg {
        Err(_) => Ok((s, expr)),
        Ok((s2, tmp)) => match preceded(get_tag(tmp, AS), parse_idents_assignation)(s2) {
            Ok((s, name)) => (Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr)))))),
            Err(err) => match err {
                Failure(err) => Err(Failure(err)),
                _ => Ok((s, expr)),
            },
        },
    }
}
