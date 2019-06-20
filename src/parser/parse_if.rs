use crate::comment;
use crate::parser::{tools::*, parse_block, ast::*, tokens::*};
use nom::*;

named!(pub parse_if<Span, Expr>, do_parse!(
    comment!(tag!(IF)) >>
    condition: parse_strict_condition_group >>
    block: comment!(parse_block) >>
    (Expr::IfExpr{cond: Box::new(condition), consequence: block})
));

#[cfg(test)]
mod tests {
    use super::*;
    use nom::types::*;
    use crate::comment;

    named!(pub test_if<Span, Expr>, comment!(parse_if));

    #[test]
    fn ok_normal_if1() {
        let string = Span::new(CompleteByteSlice("if ( event ) { say \"hola\" }".as_bytes()));
        match test_if(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn ok_normal_if2() {
        let string = Span::new(CompleteByteSlice("if ( event ) { say \"hola\"  say event }".as_bytes()));
        match test_if(string) {
            Ok(..) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    #[test]
    fn err_normal_if1() {
        let string = Span::new(CompleteByteSlice("if ".as_bytes()));
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_normal_if2() {
        let string = Span::new(CompleteByteSlice("if ( event ) ".as_bytes()));
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }

    #[test]
    fn err_normal_if3() {
        let string = Span::new(CompleteByteSlice("if ( event { say \"hola\"  say event }".as_bytes()));
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
