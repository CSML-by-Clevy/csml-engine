use crate::data::{ast::*, tokens::*};
use crate::error_format::{gen_nom_failure, ERROR_IMPORT_STEP};
use crate::parser::{parse_comments::comment, get_tag, StateContext, get_string, parse_idents::{parse_idents_as, parse_idents_assignation}};

use nom::{
    bytes::complete::tag,
    bytes::complete::take_till1,
    combinator::{cut, map, opt},
    branch::alt,
    error::{context, ParseError},
    multi::separated_list,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};


// // _params_
// test
// {test, ...}
// test as plop
// // *

// // --------------------------------------

// import _params_
// // import _params_ from fn_flow


////////////////////////////////////////////////////////////////////////////////
//// TOOL FUNCTION
////////////////////////////////////////////////////////////////////////////////

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

    Ok((s, vec!(expr)))
}

fn parse_group<'a, E>(s: Span<'a>) -> IResult<Span<'a>, Vec<Expr>, E> 
where
    E: ParseError<Span<'a>>,
{
    let (s, (vec, ..) ) = preceded(
        tag(L_BRACE),
        terminated(
            tuple((
                map(
                    separated_list(preceded(comment, tag(COMMA)), parse_fn_name),
                    |vec| {
                        vec
                            .into_iter()
                            .map(|expr| expr)
                            .collect()
                    },
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
    alt((
        parse_group,
        parse_fn_name_as_vec
    ))(s)
}

////////////////////////////////////////////////////////////////////////////////
//// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn parse_import<'a, E>(s: Span<'a>) -> IResult<Span<'a>, (), E>
where
    E: ParseError<Span<'a>>,
{
    let (s, name) = preceded(comment, get_string)(s)?;
    let (s, ..) = get_tag(name, IMPORT)(s)?;

    let (s, vec) = preceded(comment, parse_import_params)(s)?;

    Ok((s, ()))
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
