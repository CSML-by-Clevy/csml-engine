use crate::parser::ast::*;
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    data::Data,
    message::*,
    ast_interpreter::interpret_scope,
    variable_handler::{
        gen_literal::gen_literal_form_event
    },
};

fn match_response(
    args: &[Expr],
    mut root: MessageData, 
    data: &mut Data,
    opt: &Option<Identifier>,
    range: RangeInterval
) -> Result<MessageData, ErrorInfo> {
    if let Some(Identifier{ident, interval, index}) = opt {
        if let Some(..) = index {
            return Err(
                ErrorInfo{
                    message: "Error: Ask/Response default value is not an Array".to_owned(),
                    interval: range.start
                }
            )
        };
        root = root.add_to_memory(
            ident.to_owned(),
            gen_literal_form_event(data.event, interval.to_owned())?,
        );
    }
    Ok(root + interpret_scope(args, data)?)
}

pub fn match_ask_response(
    vec: &[Expr],
    root: MessageData,
    data: &mut Data,
    opt: &Option<Identifier>,
    range: RangeInterval,
) -> Result<MessageData, ErrorInfo> {
    for block in vec.iter() {
        match (block, data.event, data.memory.is_initial_step) {
            (
                Expr::Block {
                    block_type: BlockType::Response,
                    arg: args,
                    ..
                },
                Some(..),
                false
            ) => return match_response(args, root, data, opt, range),
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                None,
                false
            ) => return Ok(root + interpret_scope(args, data)?),
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                Some(..),
                true
            ) => return Ok(root + interpret_scope(args, data)?),
            (..) => continue,
        }
    }
    Err(
        ErrorInfo{
            message: "Error fail to find the correct action block bettween Ask/Response".to_owned(),
            interval: range.start
        }
    )
}
