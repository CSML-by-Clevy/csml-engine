use crate::data::{ast::*, tokens::*};
use crate::parser::operator::parse_operator::parse_operator;
use crate::parser::parse_parenthesis::parse_l_parentheses;
use crate::parser::parse_parenthesis::parse_r_parentheses;
use crate::parser::{
    parse_comments::comment,
    parse_scope::{parse_implicit_scope, parse_scope},
    tools::*,
};
use nom::{
    branch::alt, bytes::complete::tag, combinator::opt, error::ParseError, sequence::delimited,
    sequence::preceded, *,
};

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_strict_condition_group<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    delimited(parse_l_parentheses, parse_operator, parse_r_parentheses)(s)
}

fn parse_else_if<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Box<IfStatement>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(ELSE))(s)?;
    let (s, _) = preceded(comment, tag(IF))(s)?;

    let (s, condition) = parse_strict_condition_group(s)?;

    let (s, block) = alt((parse_scope, parse_implicit_scope))(s)?;

    let (s, opt) = opt(alt((parse_else_if, parse_else)))(s)?;

    Ok((
        s,
        Box::new(IfStatement::IfStmt {
            cond: Box::new(condition),
            consequence: block,
            then_branch: opt,
            last_action_index: 0, // this wil be update in parse_root
        }),
    ))
}

fn parse_else<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Box<IfStatement>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(ELSE))(s)?;
    let (s, mut interval) = get_interval(s)?;

    let (s, block) = alt((parse_scope, parse_implicit_scope))(s)?;

    let (s, end) = get_interval(s)?;
    interval.add_end(end);

    Ok((
        s,
        Box::new(IfStatement::ElseStmt(block, interval)),
    ))
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_if<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(IF))(s)?;
    let (s, condition) = parse_strict_condition_group(s)?;

    let (s, block) = alt((parse_scope, parse_implicit_scope))(s)?;

    let (s, opt) = opt(alt((parse_else_if, parse_else)))(s)?;

    Ok((
        s,
        Expr::IfExpr(IfStatement::IfStmt {
            cond: Box::new(condition),
            consequence: block,
            then_branch: opt,
            last_action_index: 0, // this wil be update in parse_root
        }),
    ))
}

////////////////////////////////////////////////////////////////////////////////
// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_if(s: Span) -> IResult<Span, Expr> {
        preceded(comment, parse_if)(s)
    }

    #[test]
    fn ok_normal_if1() {
        let string = Span::new("if ( event ) { say \"hola\" }");
        match test_if(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_if2() {
        let string = Span::new("if ( event ) { say \"hola\"  say event }");
        match test_if(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_else_if1() {
        let string =
            Span::new("if ( event ) { say \"hola\" } else if ( event ) { say \" hola 2 \" }");
        match test_if(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_normal_if1() {
        let string = Span::new("if ");
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_normal_if2() {
        let string = Span::new("if ( event ) ");
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_normal_if3() {
        let string = Span::new("if ( event { say \"hola\"  say event }");
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
