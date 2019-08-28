use crate::comment;
use crate::parser::{
    ast::*, 
    parse_block, 
    parse_implicit_block, 
    tokens::*,
    tools::*,
};
use nom::*;

named!(pub parse_else_if<Span, Box<IfStatement>>, do_parse!(
    comment!(tag!(ELSE)) >>
    comment!(tag!(IF)) >>
    condition: parse_strict_condition_group >>
    block: alt!(parse_block | parse_implicit_block) >>
    opt: opt!(alt!( parse_else_if | parse_else)) >>

    (Box::new(
        IfStatement::IfStmt{
            cond: Box::new(condition),
            consequence: block,
            then_branch: opt,
        }
    ))
));

named!(pub parse_else<Span, Box<IfStatement>>, do_parse!(
    comment!(tag!(ELSE)) >>
    start: get_interval >>
    block: alt!(parse_block | parse_implicit_block) >>
    end: get_interval >>
    (Box::new(IfStatement::ElseStmt(block, RangeInterval{start, end})))
));

named!(pub parse_if<Span, Expr>, do_parse!(
    comment!(tag!(IF)) >>
    condition: parse_strict_condition_group >>
    block: alt!(parse_block | parse_implicit_block) >>
    opt: opt!(alt!( parse_else_if | parse_else)) >>

    (Expr::IfExpr(
        IfStatement::IfStmt{
            cond: Box::new(condition),
            consequence: block,
            then_branch: opt,
        } 
    ))
));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comment;
    use nom::types::*;

    named!(pub test_if<Span, Expr>, comment!(parse_if));

    #[test]
    fn ok_normal_if1() {
        let string = Span::new(CompleteByteSlice(
            "if ( event ) { say \"hola\" }".as_bytes(),
        ));
        match test_if(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_if2() {
        let string = Span::new(CompleteByteSlice(
            "if ( event ) { say \"hola\"  say event }".as_bytes(),
        ));
        match test_if(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn ok_normal_else_if1() {
        let string = Span::new(CompleteByteSlice(
            "if ( event ) { say \"hola\" } else if ( event ) { say \" hola 2 \" }".as_bytes(),
        ));
        match test_if(string) {
            Ok(..) => {}
            Err(e) => panic!("{:?}", e),
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
        let string = Span::new(CompleteByteSlice(
            "if ( event { say \"hola\"  say event }".as_bytes(),
        ));
        match test_if(string) {
            Ok(..) => panic!("need to fail"),
            Err(..) => {}
        }
    }
}
