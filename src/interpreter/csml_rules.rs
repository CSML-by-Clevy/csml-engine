use crate::parser::ast::*;
// use std::io::*;

pub fn check_ident(expr: &Ident, name: &str) -> bool {
    match expr {
        Ident(string) if string == name => true,
        _                               => false,
    }
}

fn contains_label(step: &Step, name :&str) -> bool {
    match step {
        Step::Block{label: Ident(label), ..} if label == name   => true,
        _                                                       => false
    }
}

fn contains_starterflow(step: &Step) -> bool {
    if let Step::FlowStarter{ .. } = step { 
        true
    } else {
        false
    }
}

pub fn double_label(mut ast: &[Step]) -> bool
{
    while let Some((hd, tl)) = ast.split_first() {
        match (hd, tl) {
            (Step::Block{label: Ident(name), ..}, tl)  => {
                match tl.iter().find(|&x| contains_label(x, name)) {
                    Some( .. )  => return false,
                    None        => true
                }
            },
            (Step::FlowStarter{ .. }, tl)  => {
                match tl.iter().find(|&x| contains_starterflow(x)) {
                    Some( .. )  => return false,
                    None        => true
                }
            },
        };
        ast = tl;
    }
    true
}

pub fn is_variable(expr: &Expr) -> bool {
    match expr {
        Expr::LitExpr(_e)   => true,
        Expr::IdentExpr(_e) => true,
        _                   => false,
    }
}

pub fn eval_condition(cond: &[Expr]) -> bool {
    match cond.split_last() {
        Some((last, elements)) if is_variable(last) && check_infixexpr(elements)    => true,
        _                                                                           => false,
    }
}

pub fn check_infixexpr(exprs: &[Expr]) -> bool {
    for expr in exprs.iter() {
        match expr {
            Expr::InfixExpr(_, e)   => {
                if let Expr::VecExpr(expr) = e.as_ref() {
                   if !eval_condition(expr) {
                       return false;
                   }
                }
            },
            _                       => { return false; },
        };
    }
    true
}

// fn reserved_keywords(ident: &Ident) -> bool
// {
//     match ident {
//         Ident(arg) if arg == "input"   => true,
//         _                              => false
//     }
// }

// fn check_valid_literal(expr: &Expr) -> bool
// {
//     let mut res = false;

//     if let Expr::LitExpr(lit) = expr {
//         match lit {
//             Literal::StringLiteral(string)   => res = reserved_keywords(string),
//             _                                => res = true,
//         };
//     }
//     return res;
// }
