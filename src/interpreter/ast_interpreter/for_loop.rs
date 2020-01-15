use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::interpret_scope,
    data::Data,
    message::MSG,
    message::*,
    variable_handler::expr_to_literal::expr_to_literal,
};
use crate::parser::{ast::*, literal::Literal};
use std::sync::mpsc;

pub fn for_loop(
    ident: &Identifier,
    i: &Option<Identifier>,
    expr: &Expr,
    block: &Block,
    range: &RangeInterval,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: usize,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let s_lit = expr_to_literal(expr, data)?;
    let vec = match s_lit {
        Literal::ArrayLiteral { items, .. } => items,
        _ => {
            return Err(ErrorInfo {
                message: "invalid Expression in foreach loop, Expression is not iterable".to_owned(),
                interval: range.start.to_owned(),
            })
        }
    };

    for (value, elem) in vec.iter().enumerate() {
        data.step_vars.insert(ident.ident.to_owned(), elem.clone());
        if let Some(index) = i {
            data.step_vars.insert(
                index.ident.to_owned(),
                Literal::int(value as i64, elem.get_interval()),
            );
        };

        root = root + interpret_scope(block, data, instruction_index, sender)?;
        if root.exit_condition.is_some() {
            root.exit_condition = None;
            break;
        }
    }
    data.step_vars.remove(&ident.ident);
    if let Some(index) = i {
        data.step_vars.remove(&index.ident);
    };
    Ok(root)
}
