
// use crate::comment;
// use crate::parser::{parse_ident::*, tools::*, parse_block, ast::*, tokens::*};
// use nom::*;


// // named!(pub lex_from_file<Span, String>, do_parse!(
// //     position: position!() >>
// //     tag!(FROM)  >>
// //     tag!(FILE)  >>
// //     (Token::FromFile(position))
// // ));

// named!(parse_import_opt<Span, (Option<String>, Option<String>, Option<String>)>, do_parse!(
//     step_name: opt!(
//         do_parse!(
//             tag!(STEP) >>
//             name: parse_ident >>
//             (name)
//         )
//     ) >>
//     as_name: opt!(
//         do_parse!(
//             tag!(AS) >>
//             name: parse_ident >>
//             (name)
//         )
//     ) >>
//     file_path: opt!(
//         do_parse!(
//             tag!(FROMEFILE) >>
//             file_path: parse_ident >>
//             (file_path)
//         )
//     ) >>
//     ((step_name, as_name, file_path))
// ));

// #[allow(dead_code)]
// fn gen_function_expr(name: &str, expr: Expr) -> Expr {
//     Expr::FunctionExpr(ReservedFunction::Import(name.to_owned()), Box::new(expr))
// }

// #[allow(dead_code)]
// fn gen_builder_expr(expr1: Expr, expr2: Expr) -> Expr {
//     Expr::BuilderExpr(Box::new(expr1), Box::new(expr2))
// }

// #[allow(dead_code)]
// fn format_step_options(step_name: String, as_name: Option<String>, file_path: Option<String>) -> Expr {
//     match (as_name, file_path) {
//         (Some(name), Some(file))    => {
//             gen_builder_expr(
//                 gen_function_expr(STEP, 
//                     gen_function_expr(AS, Expr::IdentExpr(name))
//                 ),
//                 gen_function_expr(FILE, Expr::IdentExpr(file))
//             )
//         },
//         (Some(name), None)          => {
//             gen_function_expr(STEP, 
//                 gen_function_expr(AS, Expr::IdentExpr(name))
//             )
//         },
//         (None, Some(file))          => {
//             gen_builder_expr(
//                 gen_function_expr(STEP, Expr::IdentExpr(step_name)),
//                 gen_function_expr(FILE, Expr::IdentExpr(file))
//             )
//         },
//         (None, None)                => gen_function_expr(STEP, Expr::IdentExpr(step_name)),
//     }
// }

// #[allow(dead_code)]
// fn format_import_opt(span: Span) -> IResult<Span , Expr> {
//     match parse_import_opt(span) {
//         Ok((_, (Some(step), as_name, file_path)))   => Ok((span, format_step_options(step, as_name, file_path))),
//         Ok((_, (None, None, Some(file_path))))      => Ok((span, gen_function_expr(FILE, Expr::IdentExpr(file_path)))),
//         Err(e)                                      => Err(e),
//         _                                           => Err(Err::Failure(Context::Code(span, NomError::Custom(42)))),
//     }
// }

// named!(parse_import_from<Span, Expr>, do_parse!(
//     expr: format_import_opt >>
//     (expr)
// ));

// named!(parse_import<Span, Expr>, do_parse!(
//     tag!(IMPORT) >>
//     name: parse_import_from >>
//     (Expr::FunctionExpr(Ident(IMPORT.to_owned()), Box::new(name)))
// ));
