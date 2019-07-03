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

named!(pub parse_import<Span, Expr>, do_parse!(
    comment!(tag!(IMPORT)) >>
    name: parse_import_opt >>
    (name)
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::*;

    named!(pub test_import<Span, Expr>, exact!(parse_import));

    #[test]
    fn ok_step_import() {
        let string = Span::new(CompleteByteSlice("import step hola".as_bytes()));
        match test_import(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn ok_step_import_as() {
        let string = Span::new(CompleteByteSlice("import step hola as test".as_bytes()));
        match test_import(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn ok_step_import_as_from_file() {
        let string = Span::new(CompleteByteSlice("import step hola as test FromFile filetest".as_bytes()));
        match test_import(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn err_step_import1() {
        let string = Span::new(CompleteByteSlice("import hola".as_bytes()));
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import2() {
        let string = Span::new(CompleteByteSlice("import step".as_bytes()));
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import_as() {
        let string = Span::new(CompleteByteSlice("import step hola as".as_bytes()));
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_step_import_as_from_file() {
        let string = Span::new(CompleteByteSlice("import step hola as".as_bytes()));
        match test_import(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
