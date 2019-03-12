use crate::parser::ast::*;

pub fn check_ident(expr: &Ident, name: &str) -> bool {
    match expr {
        Ident(string) if string == name => true,
        _                               => false,
    }
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
                   if eval_condition(expr) == false {
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
