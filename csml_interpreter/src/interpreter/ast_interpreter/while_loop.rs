use crate::data::{
    ast::*,
    // hold::{
    //     hold_index_end_loop, hold_index_start_loop, hold_loop_decrs_index, hold_loop_incrs_index,
    // },
    // primitive::tools::get_array,
    Data, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::{
    ast_interpreter::if_statement::valid_condition,
    interpret_scope
};
use crate::parser::ExitCondition;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn while_loop(
    cond: &Expr,
    block: &Block,
    _range_interval: &Interval,
    mut msg_data: MessageData,
    data: &mut Data,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {

    while valid_condition(cond, data, &mut msg_data, sender) {

        msg_data = msg_data + interpret_scope(block, data, sender)?;

        match msg_data.exit_condition {
            Some(ExitCondition::Break) => {
                msg_data.exit_condition = None;
                break;
            }
            Some(ExitCondition::Continue) => msg_data.exit_condition = None,
            Some(_) => break,
            None => {}
        }
    }

    Ok(msg_data)
}
