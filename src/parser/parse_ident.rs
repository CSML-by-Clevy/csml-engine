use crate::comment;
use crate::parser::{
    ast::*,
    parse_var_types::{parse_as_variable, parse_var_expr},
    tokens::*,
    tools::get_interval,
    ParserErrorType,
};
use nom::*;
use nom::{Err, ErrorKind as NomError};

named!(parse_box_expr<Span, Box<Expr> >, do_parse!(
    expr: alt!(parse_as_variable | parse_var_expr) >>
    (Box::new(expr))
));

fn parse_string<'a>(input: Span<'a>) -> IResult<Span<'a>, String> {
    let (rest, val) = take_till1!(input, is_valid_char)?;

    match String::from_utf8(val.fragment.to_vec()) {
        Ok(string) => Ok((rest, string)),
        Err(_) => Err(Err::Failure(Context::Code(
            rest,
            NomError::Custom(ParserErrorType::NoAscii as u32),
        ))),
    }
}

named!(pub parse_ident<Span, Identifier>, do_parse!(
    position: get_interval >>
    var: comment!(parse_string) >>
    index: opt!(
         delimited!(
            comment!(tag!(L_BRACKET)),
            parse_box_expr,
            comment!(tag!(R_BRACKET))
        )
    ) >>
    (forma_ident(
        var,
        index, 
        position
    ))
));

pub fn is_valid_char(input: u8) -> bool {
    let var = input as char;
    input != b'_' && !var.is_alphanumeric()
}

pub fn forma_ident(ident: String, index: Option<Box<Expr>>, position: Interval) -> Identifier {
    Expr::new_ident(ident, position, index)
}
