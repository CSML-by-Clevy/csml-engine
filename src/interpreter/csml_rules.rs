use crate::parser::{
    ast::{Expr, Flow, InstructionType},
    tokens::*,
};

//TODO: Check sub block ask/respond rules
pub fn check_ident(expr: &str, name: &str) -> bool {
    match expr {
        string if string == name => true,
        _ => false,
    }
}

// fn contains_label(step: &Step, name: &str) -> bool {
//     match step {
//         Step{label: label, ..} if label == name   => true,
//         _                                                => false
//     }
// }

// pub fn double_label(ast: &Flow) -> bool {
//     let mut steps: &[Step] = &ast.steps;
//     if ast.accept.is_empty() {
//         return false;
//     }
//     while let Some((hd, tl)) = steps.split_first() {
//         match (hd, tl) {
//             (Step{label: name, ..}, tl)  => {
//                 match tl.iter().find(|&x| contains_label(x, name)) {
//                     Some( .. )  => return false,
//                     None        => true
//                 }
//             },
//         };
//         steps = tl;
//     }
//     true
// }

// let info = flow.flow_instructions.get(&InstructionType::StartFlow(ACCEPT.to_string()));

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
