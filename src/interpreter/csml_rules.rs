use crate::parser::ast::{Expr, Flow, Ident, Literal, Step};
use crate::interpreter::ast_interpreter::AstInterpreter;

//TODO: Check sub block ask/respond rules

pub fn check_expr_is_ident(expr: &Expr, name: &str) -> bool {
    if let Expr::IdentExpr(Ident(var)) = expr {
        if var == name {
            return true;
        } else {
            return false;
        }
    }
    false
}

pub fn is_trigger(flow: &Flow, string: &str) -> bool {
    for elem in flow.accept.iter() {
        match elem {
            Expr::LitExpr(Literal::StringLiteral(tag)) if tag == string => return true,
            _                                                           => continue,
        }
    }
    false
}

// return Result<struct, error>
pub fn search_for(flow: &Flow, name: &str, preter: AstInterpreter) -> Option<String> {
    for step in flow.steps.iter() {
        match step {
            Step{label, actions} if check_ident(label, name) => {
                let result = preter.match_block(actions).unwrap();
                let ser = serde_json::to_string(&result).unwrap();
                return Some(ser);
            }
            _ => continue,
        }
    }
    None
}

pub fn check_ident(expr: &Ident, name: &str) -> bool {
    match expr {
        Ident(string) if string == name => true,
        _                               => false,
    }
}

fn contains_label(step: &Step, name: &str) -> bool {
    match step {
        Step{label: Ident(label), ..} if label == name   => true,
        _                                                => false
    }
}

pub fn double_label(ast: &Flow) -> bool {
    let mut steps: &[Step] = &ast.steps;
    if ast.accept.is_empty() {
        return false;
    }
    while let Some((hd, tl)) = steps.split_first() {
        match (hd, tl) {
            (Step{label: Ident(name), ..}, tl)  => {
                match tl.iter().find(|&x| contains_label(x, name)) {
                    Some( .. )  => return false,
                    None        => true
                }
            },
        };
        steps = tl;
    }
    true
}

pub fn check_valid_flow(flow: &Flow) -> bool {
    let mut accept_flow = false;
    let mut start = false;

    if !flow.accept.is_empty() { accept_flow = true; }
    for step in flow.steps.iter() {
        match step {
            Step{label: Ident(label), .. } if label == "start"  => start = true,
            _                                                   => continue
        }
    }
    start && accept_flow && double_label(&flow)
}