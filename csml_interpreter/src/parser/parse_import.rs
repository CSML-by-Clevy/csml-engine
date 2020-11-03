use crate::data::{ast::*, primitive::PrimitiveNull, tokens::*, Position};
use crate::error_format::{
    ERROR_IMPORT_ARGUMENT
};
use crate::parser::{
    get_interval, get_string, get_tag,
    parse_comments::comment,
    parse_idents::{parse_idents_as, parse_idents_assignation},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    error::{ParseError, ErrorKind},
    multi::separated_list,
    sequence::{preceded, terminated, tuple},
    IResult,
    Err
};

////////////////////////////////////////////////////////////////////////////////
//// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn parse_fn_name<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Expr, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, identifier) = parse_idents_assignation(s)?;

    parse_idents_as(s, Expr::IdentExpr(identifier))
}

fn parse_fn_name_as_vec<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, expr) = parse_fn_name(s)?;

    Ok((s, vec![expr]))
}

fn parse_group<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, (vec, ..)) = preceded(
        tag(L_BRACE),
        terminated(
            tuple((
                map(
                    separated_list(preceded(comment, tag(COMMA)), parse_fn_name),
                    |vec| vec.into_iter().map(|expr| expr).collect(),
                ),
                opt(preceded(comment, tag(COMMA))),
            )),
            preceded(comment, tag(R_BRACE)),
        ),
    )(s)?;

    Ok((s, vec))
}

fn parse_import_params<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E>
where
    E: ParseError<Span<'a>>,
{
    match alt((parse_group, parse_fn_name_as_vec))(s) {
        Ok(value) => Ok(value),
        Err(Err::Error(e)) => {
            return Err(Err::Failure(E::add_context(s, ERROR_IMPORT_ARGUMENT, e)))
        }
        Err(Err::Failure(e)) => return Err(Err::Failure(E::append(s, ErrorKind::Tag, e))),
        Err(Err::Incomplete(needed)) => return Err(Err::Incomplete(needed)),
    }
}

fn parse_from<'a, E>(s: Span<'a>) -> IResult<Span<'a>, String, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, FROM)(s)?;
    let (s, name) = preceded(comment, get_string)(s)?;

    Ok((s, name))
}

////////////////////////////////////////////////////////////////////////////////
//// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_import<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Instruction>, E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;

    let (s, ..) = get_tag(name, IMPORT)(s)?;

    let (s, start) = get_interval(s)?;
    let (s, fn_names) = preceded(comment, parse_import_params)(s)?;

    let (s, from_flow) = opt(parse_from)(s)?;

    let instructions = fn_names
        .iter()
        .map(|name| {
            let (name, original_name) = match name {
                Expr::IdentExpr(ident) => (ident.ident.to_owned(), None),
                Expr::ObjectExpr(ObjectType::As(name, expr)) => match &**expr {
                    Expr::IdentExpr(ident) => (name.ident.to_owned(), Some(ident.ident.to_owned())),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };

            Instruction {
                instruction_type: InstructionScope::ImportScope(ImportScope {
                    name,
                    original_name,
                    from_flow: from_flow.clone(),
                    position: Position::new(start.clone()),
                }),
                actions: Expr::LitExpr(PrimitiveNull::get_literal(start)),
            }
        })
        .collect();

    Ok((s, instructions))
}

////////////////////////////////////////////////////////////////////////////////
//// TEST FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use nom::error::ErrorKind;

//     pub fn test_import(s: Span) -> IResult<Span, ()> {
//         let var = parse_import(s);
//         if let Ok((s, v)) = var {
//             if s.fragment().len() != 0 {
//                 Err(Err::Error((s, ErrorKind::Tag)))
//             } else {
//                 Ok((s, ()))
//             }
//         } else {
//             var
//         }
//     }

//     #[test]
//     fn ok_step_import() {
//         let string = Span::new("import step hola");
//         match test_import(string) {
//             Ok(..) => {}
//             Err(e) => panic!("{:?}", e),
//         }
//     }

//     #[test]
//     fn ok_step_import_as() {
//         let string = Span::new("import step hola as test");
//         match test_import(string) {
//             Ok(..) => {}
//             Err(e) => panic!("{:?}", e),
//         }
//     }

//     #[test]
//     fn ok_step_import_as_from_file() {
//         let string = Span::new("import step hola as test FromFile filetest");
//         match test_import(string) {
//             Ok(..) => {}
//             Err(e) => panic!("{:?}", e),
//         }
//     }

//     #[test]
//     fn err_step_import1() {
//         let string = Span::new("import hola");
//         match test_import(string) {
//             Ok(..) => panic!("need to fail"),
//             Err(..) => {}
//         }
//     }

//     #[test]
//     fn err_step_import2() {
//         let string = Span::new("import step");
//         match test_import(string) {
//             Ok(..) => panic!("need to fail"),
//             Err(..) => {}
//         }
//     }

//     #[test]
//     fn err_step_import_as() {
//         let string = Span::new("import step hola as");
//         match test_import(string) {
//             Ok(..) => panic!("need to fail"),
//             Err(..) => {}
//         }
//     }

//     #[test]
//     fn err_step_import_as_from_file() {
//         let string = Span::new("import step hola as");
//         match test_import(string) {
//             Ok(..) => panic!("need to fail"),
//             Err(..) => {}
//         }
//     }
// }
