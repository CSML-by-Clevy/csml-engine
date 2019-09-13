use crate::parser::ast::Interval;
use crate::parser::ast::{Flow, InstructionType};
use crate::error_format::data::ErrorInfo;

pub fn check_ident(expr: &str, name: &str) -> bool {
    match expr {
        string if string == name => true,
        _ => false,
    }
}

pub fn check_valid_flow(flow: &Flow) -> Result<&Flow, ErrorInfo> {
    if flow
        .flow_instructions
        .get(&InstructionType::NormalStep("start".to_owned()))
        .is_none()
    {
        return Err(ErrorInfo {
            interval: Interval { line: 0, column: 0 },
            message: "ERROR: Flow need to have a start step ".to_string(),
        });
    }

    Ok(flow)
}
