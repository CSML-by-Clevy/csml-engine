use crate::data::primitive::int::PrimitiveInt;
use crate::data::{ast::*, Data, Literal, MessageData, MSG};
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
    i: &Option<Identifier>,
    expr: &Expr,
    block: &Block,
    range: &RangeInterval,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let literal = expr_to_literal(expr, None, data, &mut root, sender)?;

    let array = match Literal::get_value::<Vec<Literal>>(&literal.primitive) {
        Some(res) => res,
        None => {
            return Err(gen_error_info(
                range.start.to_owned(),
                ERROR_FOREACH.to_owned(),
            ))
        }
    };

    for (value, elem) in array.iter().enumerate() {
        data.step_vars
            .insert(ident.ident.to_owned(), elem.to_owned());
        if let Some(index) = i {
            data.step_vars.insert(
                index.ident.to_owned(),
                PrimitiveInt::get_literal(value as i64, elem.interval.to_owned()),
            );
        };

        root = root + interpret_scope(block, data, instruction_index, sender)?;
        if let Some(ExitCondition::Break) = root.exit_condition {
            root.exit_condition = None;
            break;
        } else if root.exit_condition.is_some() {
            break;
        }
    }
    data.step_vars.remove(&ident.ident);
    if let Some(index) = i {
        data.step_vars.remove(&index.ident);
    };
    Ok(root)
}
