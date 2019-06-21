use crate::comment;
use crate::parser::{parse_ident::*, tools::*, ast::*, tokens::*};
use nom::*;

named!(parse_import_opt<Span, Expr>, do_parse!(
    step_name: do_parse!(
            comment!(parse_import_step) >>
            name: parse_ident >>
            (name)
    ) >>
    as_name: opt!(
        do_parse!(
            comment!(tag!(AS)) >>
            name: parse_ident >>
            (name)
        )
    ) >>
    file_path: opt!(
        do_parse!(
            comment!(tag!(FROMEFILE)) >>
            file_path: parse_ident >>
            (file_path)
        )
    ) >>
    (Expr::FunctionExpr(ReservedFunction::Import{step_name, as_name, file_path}))
));

named!(parse_import<Span, Expr>, do_parse!(
    comment!(tag!(IMPORT)) >>
    name: parse_import_opt >>
    (name)
));
