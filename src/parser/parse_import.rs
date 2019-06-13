// named!(pub lex_from_file<Span, Token>, do_parse!(
//     position: position!() >>
//     tag!(FROM)  >>
//     tag!(FILE)  >>
//     (Token::FromFile(position))
// ));

// named!(parse_import_opt<Tokens, (Option<Ident>, Option<Ident>, Option<Ident>)>, do_parse!(
//     step_name: opt!(
//         do_parse!(
//             tag_token!(Token::Step) >>
//             name: parse_ident!() >>
//             (name)
//         )
//     ) >>
//     as_name: opt!(
//         do_parse!(
//             tag_token!(Token::As) >>
//             name: parse_ident!() >>
//             (name)
//         )
//     ) >>
//     file_path: opt!(
//         do_parse!(
//             tag_token!(Token::FromFile) >>
//             file_path: parse_ident!() >>
//             (file_path)
//         )
//     ) >>
//     ((step_name, as_name, file_path))
// ));

// #[allow(dead_code)]
// fn gen_function_expr(name: &str, expr: Expr) -> Expr {
//     Expr::FunctionExpr(Ident(name.to_owned()), Box::new(expr))
// }

// #[allow(dead_code)]
// fn gen_builder_expr(expr1: Expr, expr2: Expr) -> Expr {
//     Expr::BuilderExpr(Box::new(expr1), Box::new(expr2))
// }

// #[allow(dead_code)]
// fn format_step_options(step_name: Ident, as_name: Option<Ident>, file_path: Option<Ident>) -> Expr{
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
// fn format_import_opt(tokens: Tokens) -> IResult<Tokens , Expr> {
//     match parse_import_opt(tokens) {
//         Ok((_, (Some(step), as_name, file_path)))   => Ok((tokens, format_step_options(step, as_name, file_path))),
//         Ok((_, (None, None, Some(file_path))))      => Ok((tokens, gen_function_expr(FILE, Expr::IdentExpr(file_path)))),
//         Err(e)                                      => Err(e),
//         _                                           => Err(Err::Failure(Context::Code(tokens, NomError::Custom(42)))),
//     }
// }

// named!(parse_import_from<Tokens, Expr>, do_parse!(
//     expr: format_import_opt >>
//     (expr)
// ));

// named!(parse_import<Tokens, Expr>, do_parse!(
//     tag_token!(Token::Import) >>
//     name: parse_import_from >>
//     (Expr::FunctionExpr(Ident(IMPORT.to_owned()), Box::new(name)))
// ));
