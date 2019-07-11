use crate::parser::{
    ast::{Interval, SmartIdent, Expr, Flow, InstructionType}
};

pub fn check_ident(expr: &str, name: &str) -> bool {
    match expr {
        string if string == name => true,
        _ => false,
    }
}

pub fn check_valid_flow(flow: &Flow) -> bool {
    let mut accept_flow = false;
    let mut start = false;

    if let Some(Expr::VecExpr(vec)) = flow
        .flow_instructions
        .get(&InstructionType::StartFlow)
    {
        if !vec.is_empty() {
            accept_flow = true;
        }
    }

    if flow
        .flow_instructions
        .get(&InstructionType::NormalStep(
            SmartIdent {
                ident: "start".to_string(),
                interval: Interval{line: 0, column: 0}
            }
        )).is_some()
    {
        start = true;
    }

    start && accept_flow
}
