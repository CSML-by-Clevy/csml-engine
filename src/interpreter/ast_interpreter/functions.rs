use crate::data::{ast::*, Data, Literal, MessageData, MSG};
use crate::error_format::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::evaluate_condition,
    variable_handler::{
        expr_to_literal, get_string_from_complexstring, get_var, interval::interval_from_expr,
    },
};
use std::sync::mpsc;

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

pub fn match_functions(
    action: &Expr,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match action {
        Expr::ObjectExpr(ObjectType::As(name, expr)) => {
            let lit = match_functions(expr, data, root, sender)?;
            data.step_vars.insert(name.ident.to_owned(), lit.clone());
            Ok(lit)
        }
        Expr::ComplexLiteral(vec, RangeInterval { start, .. }) => Ok(
            get_string_from_complexstring(vec, start.to_owned(), data, root, sender),
        ),
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            Ok(evaluate_condition(infix, exp_1, exp_2, data, root, sender)?)
        }
        Expr::IdentExpr(ident) => match get_var(ident.to_owned(), data, root, sender) {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        },
        Expr::ObjectExpr(ObjectType::Normal(..))
        | Expr::MapExpr(..)
        | Expr::LitExpr { .. }
        | Expr::VecExpr(..) => Ok(expr_to_literal(action, data, root, sender)?),
        e => Err(ErrorInfo {
            message: format!("invalid function {:?}", e),
            interval: interval_from_expr(e),
        }),
    }
}
