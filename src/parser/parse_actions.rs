use crate::parser::{
    ast::*,
    parse_comments::comment,
    parse_ident::parse_ident,
    parse_import::parse_import,
    parse_var_types::{parse_as_variable, parse_expr_list, parse_var_expr},
    tokens::*,
    // tools::get_interval,
    GotoType,
};
use nom::{
    branch::alt, bytes::complete::tag, combinator::complete, error::ParseError, sequence::preceded,
    *,
};

pub fn parse_assignation<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = parse_ident(s)?;
    let (s, _) = preceded(comment, tag(ASSIGN))(s)?;
    let (s, expr) = complete(alt((parse_as_variable, parse_var_expr)))(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Assign(name, Box::new(expr))),
    ))
}

fn get_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    let (s, ..) = preceded(comment, tag(STEP))(s)?;
    Ok((s, GotoType::Step))
}

fn get_sub_step<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    let (s, ..) = preceded(comment, tag("@"))(s)?;
    Ok((s, GotoType::SubStep))
}

fn get_flow<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    let (s, ..) = preceded(comment, tag(FLOW))(s)?;
    Ok((s, GotoType::Flow))
}

fn get_default<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, GotoType, E> {
    Ok((s, GotoType::Step))
}

fn parse_goto<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag(GOTO))(s)?;
    let (s, goto_type) = alt((get_step, get_flow, get_sub_step, get_default))(s)?;
    let (s, name) = match parse_ident(s) {
        Ok(vars) => vars,
        Err(Err::Error(err)) | Err(Err::Failure(err)) => {
            return Err(Err::Error(E::add_context(
                s,
                "missing step name after goto",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };
    Ok((s, Expr::ObjectExpr(ObjectType::Goto(goto_type, name))))
}

fn parse_say<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag(SAY))(s)?;
    let (s, expr) = complete(alt((parse_as_variable, parse_var_expr)))(s)?;
    Ok((s, Expr::ObjectExpr(ObjectType::Say(Box::new(expr)))))
}

fn parse_use<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag(USE))(s)?;
    let (s, expr) = complete(alt((parse_as_variable, parse_var_expr)))(s)?;
    Ok((s, Expr::ObjectExpr(ObjectType::Use(Box::new(expr)))))
}

// fn parse_sub_step<'a, E: ParseError<Span<'a> >>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
//     let (s, ..) = preceded(comment, tag("@"))(s)?;
//     let (s, start) = get_interval(s)?;
//     let (s, ident) = preceded(comment, complete(parse_ident))(s)?;
//     let (s, end) = get_interval(s)?;

//     Ok((s, Expr::Block{block_type: BlockType::SubStep(ident), arg: vec!(), range: RangeInterval{start, end}}))
// }

fn parse_remember<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, ..) = preceded(comment, tag(REMEMBER))(s)?;
    let (s, expr) = parse_var_expr(s)?;

    let (s, _) = match preceded(comment, tag(AS))(s) {
        Ok(vars) => vars,
        Err(Err::Error(err)) | Err(Err::Failure(err)) => {
            return Err(Err::Error(E::add_context(
                s,
                "missing as name after remember var",
                err,
            )))
        }
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    };
    let (s, ident) = preceded(comment, complete(parse_ident))(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Remember(ident, Box::new(expr))),
    ))
}

pub fn parse_actions<'a, E: ParseError<Span<'a>>>(s: Span<'a>) -> IResult<Span<'a>, Expr, E> {
    let (s, name) = parse_ident(s)?;
    let (s, expr) = parse_expr_list(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Normal(name, Box::new(expr))),
    ))
}

pub fn parse_root_functions<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, Expr, E> {
    // parse_sub_step,
    alt((
        parse_say,
        parse_remember,
        parse_import,
        parse_goto,
        parse_use,
    ))(s)
}
