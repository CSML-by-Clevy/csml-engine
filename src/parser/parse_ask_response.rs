use crate::comment;
use crate::parser::{
    ast::*,
    parse_scope::{parse_scope, parse_strick_scope},
    parse_ident::parse_ident,
    parse_functions::parse_root_functions,
    tokens::*,
    tools::*,
};
use nom::*;

fn get_option_memory(span: Span) -> IResult<Span, Option<Identifier> > {
    let (new_span, smart_ident) = parse_ident(span)?;
    if RESERVED.contains(&&*smart_ident.ident) {
        Ok((span, None))
    } else {
        Ok((new_span, Some(smart_ident)))
    }
}

named!(parse_ask<Span, (Expr, Option<Identifier>)>, do_parse!(
    comment!(tag!(ASK)) >>
    opt: opt!(parse_ident) >>
    // start: comment!(position!()) >>
    start: get_interval >>
    block: parse_scope >>
    end: get_interval >>
    (Expr::Block{block_type: BlockType::Ask, arg: block, range: RangeInterval{start, end} }, opt)
));

named!(parse_response<Span, Expr>, do_parse!(
    comment!(tag!(RESPONSE)) >>
    start: get_interval >>
    block: parse_strick_scope >>
    end: get_interval >>
    (Expr::Block{block_type: BlockType::Response, arg: block, range: RangeInterval{start, end}})
));

named!(normal_ask_response<Span, Expr>, do_parse!(
    start: get_interval >>
    ask: parse_ask  >>
    response: parse_response >>
    end: get_interval >>
    (Expr::Block{
        block_type: BlockType::AskResponse(ask.1),
        arg: vec![ask.0, response],
        range: RangeInterval{
            start,
            end
        }
    })
));

named!(short_ask_response<Span, Expr>, do_parse!(
    comment!(tag!(ASK)) >>
    ident: get_option_memory >>
    start_ask: get_interval >>
    ask: parse_root_functions >>
    end_ask: get_interval >>
    comment!(tag!(RESPONSE)) >>
    response: many0!(parse_root_functions) >>
    end_r: get_interval >>
    (Expr::Block{
        block_type: BlockType::AskResponse(ident),
        arg: vec![
            Expr::Block{
                block_type: BlockType::Ask, 
                arg: vec![ask],
                range: RangeInterval{start: start_ask.clone(), end: end_ask.clone()}
            },
            Expr::Block{
                block_type: BlockType::Response,
                arg: response,
                range: RangeInterval{start: end_ask, end: end_r.clone()}
            },
        ],
        range: RangeInterval{start: start_ask, end: end_r}
    })
));

named!(pub parse_ask_response<Span, Expr>, alt!(
    normal_ask_response | short_ask_response
));
