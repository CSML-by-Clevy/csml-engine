use crate::data::{ast::*, tokens::*};
use crate::parser::{parse_comments::comment, parse_idents::*, StateContext};
use nom::{bytes::complete::tag, combinator::opt, error::ParseError, sequence::preceded, *};

////////////////////////////////////////////////////////////////////////////////
/// TOOL FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn parse_import_step<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Span<'a>, E>
where
    E: ParseError<Span<'a>>,
{
    match tag(STEP)(s) {
        Ok((rest, val)) => Ok((rest, val)),
        Err(Err::Error((input, err))) | Err(Err::Failure((input, err))) => {
            let err = E::from_error_kind(input, err);
            Err(Err::Failure(E::add_context(input, "ImportStepError", err)))
        }
        Err(Err::Incomplete(needed)) => Err(Err::Incomplete(needed)),
    }
}

////////////////////////////////////////////////////////////////////////////////
/// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn step_name<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, parse_import_step)(s)?;
    let (s, name) = parse_idents_assignation_without_path(s)?;
    Ok((s, name))
}

fn as_name<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(AS))(s)?;
    let (s, name) = parse_idents_assignation_without_path(s)?;
    Ok((s, name))
}

fn file_path<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Identifier, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = preceded(comment, tag(FROM_FILE))(s)?;
    let (s, name) = parse_idents_assignation_without_path(s)?;
    Ok((s, name))
}

fn parse_import_opt<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, step_name) = step_name(s)?;
    let (s, as_name) = opt(as_name)(s)?;
    let (s, file_path) = opt(file_path)(s)?;
    Ok((
        s,
        Expr::ObjectExpr(ObjectType::Import {
            step_name,
            as_name,
            file_path,
        }),
    ))
}

////////////////////////////////////////////////////////////////////////////////
/// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_import<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (Expr, InstructionInfo), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, _) = tag(IMPORT)(s)?;
    let (s, name) = parse_import_opt(s)?;

    let instruction_info = InstructionInfo {
        index: StateContext::get_index(),
        total: 0,
    };

    StateContext::inc_index();

    Ok((s, (name, instruction_info)))
}

////////////////////////////////////////////////////////////////////////////////
/// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    pub fn test_import(s: Span) -> IResult<Span, (Expr, InstructionInfo)> {
        let var = parse_import(s);
        if let Ok((s, v)) = var {
            if s.fragment().len() != 0 {
                Err(Err::Error((s, ErrorKind::Tag)))
            } else {
                Ok((s, v))
            }
        } else {
            var
        }
    }

    #[test]
    fn ok_step_import() {
        let string = Span::new("import step hola");
        match test_import(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_step_import_as() {
        let string = Span::new("import step hola as test");
        match test_import(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_step_import_as_from_file() {
        let string = Span::new("import step hola as test FromFile filetest");
        match test_import(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn err_step_import1() {
        let string = Span::new("import hola");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import2() {
        let string = Span::new("import step");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import_as() {
        let string = Span::new("import step hola as");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import_as_from_file() {
        let string = Span::new("import step hola as");
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
