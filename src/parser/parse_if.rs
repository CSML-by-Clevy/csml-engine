use crate::parser::{
    ast::*,
    context::*,
    parse_comments::comment,
    parse_scope::{parse_implicit_scope, parse_scope},
    tokens::*,
    tools::*,
};
use nom::{
    branch::alt, bytes::complete::tag, combinator::opt, error::ParseError, sequence::preceded, *,
};

pub fn parse_else_if<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Box<IfStatement>, InstructionInfo), E> {
    let (s, _) = preceded(comment, tag(ELSE))(s)?;
    let (s, _) = preceded(comment, tag(IF))(s)?;

    let index = Context::get_index();

    Context::inc_index();

    let (s, condition) = parse_strict_condition_group(s)?;
    let (s, block) = alt((parse_scope, parse_implicit_scope))(s)?;
    let (s, opt) = opt(alt((parse_else_if, parse_else)))(s)?;

    let new_index = Context::get_index() - 1;
    let instruction_info = InstructionInfo {
        index,
        total: new_index - index,
    };

    Ok((
        s,
        (
            Box::new(IfStatement::IfStmt {
                cond: Box::new(condition),
                consequence: block,
                then_branch: opt,
            }),
            instruction_info,
        ),
    ))
}

pub fn parse_else<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Box<IfStatement>, InstructionInfo), E> {
    let (s, _) = preceded(comment, tag(ELSE))(s)?;
    let (s, start) = get_interval(s)?;

    let index = Context::get_index();

    Context::inc_index();

    let (s, block) = alt((parse_scope, parse_implicit_scope))(s)?;
    let (s, end) = get_interval(s)?;

    let new_index = Context::get_index() - 1;
    let instruction_info = InstructionInfo {
        index,
        total: new_index - index,
    };

    Ok((
        s,
        (
            Box::new(IfStatement::ElseStmt(block, RangeInterval { start, end })),
            instruction_info,
        ),
    ))
}

pub fn parse_if<'a, E: ParseError<Span<'a>>>(
    s: Span<'a>,
) -> IResult<Span<'a>, (Expr, InstructionInfo), E> {
    let (s, _) = preceded(comment, tag(IF))(s)?;
    let (s, condition) = parse_strict_condition_group(s)?;

    let index = Context::get_index();

    Context::inc_index();

    let (s, block) = alt((parse_scope, parse_implicit_scope))(s)?;
    let (s, opt) = opt(alt((parse_else_if, parse_else)))(s)?;

    let new_index = Context::get_index() - 1;
    let instruction_info = InstructionInfo {
        index,
        total: new_index - index,
    };

    Ok((
        s,
        (
            Expr::IfExpr(IfStatement::IfStmt {
                cond: Box::new(condition),
                consequence: block,
                then_branch: opt,
            }),
            instruction_info,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_if(s: Span) -> IResult<Span, (Expr, InstructionInfo)> {
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
