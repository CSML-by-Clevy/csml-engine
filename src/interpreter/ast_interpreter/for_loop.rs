use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::interpret_scope, data::Data, message::*,
    variable_handler::expr_to_literal::expr_to_literal,
};
use crate::parser::{ast::*, literal::Literal};

pub fn for_loop(
    ident: &Identifier,
    i: &Option<Identifier>,
    expr: &Expr,
    block: &[Expr],
    range: &RangeInterval,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    let s_lit = expr_to_literal(expr, data)?;
    let vec = match s_lit {
        Literal::ArrayLiteral { items, .. } => items,
        _ => {
            return Err(ErrorInfo {
                message: "Error in for loop, Expression is not itrerable".to_owned(),
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
        root = root + interpret_scope(block, data)?;
    }
    data.step_vars.remove(&ident.ident);
    if let Some(index) = i {
        data.step_vars.remove(&index.ident);
    };
    Ok(root)
}
