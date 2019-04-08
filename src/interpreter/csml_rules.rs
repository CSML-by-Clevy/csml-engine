use crate::parser::ast::*;
// use std::io::*;

pub fn check_ident(expr: &Ident, name: &str) -> bool {
    match expr {
        Ident(string) if string == name => true,
        _                               => false,
    }
}

fn contains_label(step: &Step, name: &str) -> bool {
    match step {
        Step{label: Ident(label), ..} if label == name   => true,
        _                                                       => false
    }
}

pub fn double_label(ast: &Flow) -> bool
{
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
