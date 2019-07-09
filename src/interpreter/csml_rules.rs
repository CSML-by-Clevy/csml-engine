use crate::parser::{
    ast::{Expr, Flow, InstructionType},
    tokens::*,
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
        .get(&InstructionType::StartFlow(ACCEPT.to_string()))
    {
        if !vec.is_empty() {
            accept_flow = true;
        }
    }

    if flow
        .flow_instructions
        .get(&InstructionType::NormalStep("start".to_string()))
        .is_some()
    {
        start = true;
    }

    start && accept_flow
}
