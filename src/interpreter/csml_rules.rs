use crate::parser::ast::Interval;
use crate::parser::ast::{Expr, Flow, InstructionType};
use crate::error_format::data::ErrorInfo;

pub fn check_ident(expr: &str, name: &str) -> bool {
    match expr {
        string if string == name => true,
        _ => false,
    }
}

pub fn check_valid_flow(flow: &Flow) -> Result<&Flow, ErrorInfo> {
    match flow.flow_instructions.get(&InstructionType::StartFlow) {
        Some(Expr::VecExpr(vec, ..)) if vec.is_empty() => {
            return Err(ErrorInfo {
                interval: Interval { line: 0, column: 0 },
                message: "ERROR: Flow need to have at least one valid flow starter | flow(\"hello\" )".to_string(),
            });
        },
        None => return Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: Flow need to have at least one valid flow starter | flow(\"hello\" )".to_string(),
        }),
        _ => {}
    }

    if !flow
        .flow_instructions
        .get(&InstructionType::NormalStep("start".to_owned()))
        .is_some()
    {
        return Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: Flow need to have a start step ".to_string(),
        });
    }

    Ok(flow)
}
