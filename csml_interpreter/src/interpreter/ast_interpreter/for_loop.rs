use crate::data::primitive::PrimitiveInt;
use crate::data::{
    ast::*,
    hold::{
        hold_index_end_loop, hold_index_start_loop, hold_loop_decrs_index, hold_loop_incrs_index,
    },
    primitive::tools::get_array,
    Data, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::interpret_scope;
use crate::interpreter::variable_handler::expr_to_literal::expr_to_literal;
use crate::parser::ExitCondition;
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn for_loop(
    ident: &Identifier,
    index: &Option<Identifier>,
    expr: &Expr,
    block: &Block,
    _range_interval: &Interval,
    mut msg_data: MessageData,
    data: &mut Data,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let literal = expr_to_literal(expr, false, None, data, &mut msg_data, sender)?;
    let mut array = get_array(literal, &data.context.flow, ERROR_FOREACH.to_owned())?;

    let mut value_skipped = 0;
    let array = hold_index_start_loop(data, &mut array, &mut value_skipped);

    for (for_loop_index, elem) in array.iter().enumerate() {
        data.step_vars
            .insert(ident.ident.to_owned(), elem.to_owned());
        if let Some(index) = index {
            data.step_vars.insert(
                index.ident.to_owned(),
                PrimitiveInt::get_literal((for_loop_index + value_skipped) as i64, elem.interval.to_owned()),
            );
        };

        hold_loop_incrs_index(data, for_loop_index + value_skipped);
        msg_data = msg_data + interpret_scope(block, data, sender)?;
        hold_loop_decrs_index(data);

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

    hold_index_end_loop(data);
    data.step_vars.remove(&ident.ident);
    if let Some(index) = index {
        data.step_vars.remove(&index.ident);
    };
    Ok(msg_data)
}
