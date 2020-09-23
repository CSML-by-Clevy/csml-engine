use crate::data::{ast::*, tokens::*};
use crate::error_format::{
    gen_nom_error, gen_nom_failure, ERROR_NUMBER_AS_IDENT, ERROR_RESERVED, ERROR_SIZE_IDENT,
};
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

fn validate_string<'a, E>(s: Span<'a>, reserved: &[&str], var: &str) -> IResult<Span<'a>, (), E>
where
    E: ParseError<Span<'a>>,
{
    if reserved.contains(&&(*var.to_ascii_lowercase())) {
        return Err(gen_nom_failure(s, ERROR_RESERVED));
    }

    if var.len() > std::u8::MAX as usize {
        return Err(gen_nom_failure(s, ERROR_SIZE_IDENT));
    }

    if var.parse::<f64>().is_ok() {
        return Err(gen_nom_error(s, ERROR_NUMBER_AS_IDENT));
    }

    Ok((s, ()))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn parse_string_usage<'a, E>(s: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, var) = get_string(s)?;
    let (s, ..) = validate_string(s, UTILISATION_RESERVED, &var)?;

    Ok((s, var))
}

pub fn parse_string_assignation<'a, E>(s: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, var) = get_string(s)?;
    let (s, ..) = validate_string(s, ASSIGNATION_RESERVED, &var)?;

    Ok((s, var))
}

pub fn parse_idents_usage<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = preceded(comment, get_interval)(s)?;

    let (s, var) = parse_string_usage(s)?;

    Ok((s, form_idents(var.to_owned(), position)))
}

pub fn parse_idents_assignation<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, position) = preceded(comment, get_interval)(s)?;

    let (s, var) = parse_string_assignation(s)?;

    Ok((s, form_idents(var.to_owned(), position)))
}

pub fn parse_idents_as<'a, E>(s: Span<'a>, expr: Expr) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let arg: IResult<Span<'a>, String, E> = preceded(comment, get_string)(s);

    match arg {
        Err(_) => Ok((s, expr)),
        Ok((s2, tmp)) => match preceded(get_tag(tmp, AS), parse_idents_assignation)(s2) {
            Ok((s, name)) => Ok((s, Expr::ObjectExpr(ObjectType::As(name, Box::new(expr))))),
            Err(err) => match err {
                Failure(err) => Err(Failure(err)),
                _ => Ok((s, expr)),
            },
        },
    }
}
