use crate::parser::ast::*;
use crate::interpreter::*;
use rand::Rng;

fn exprvec_to_vec(vec: &[Expr]) -> Vec<String> {
    vec.iter().filter_map(|elem|
        match elem {
           Expr::LitExpr(Literal::StringLiteral(string))    => Some(string.clone()),
           Expr::LitExpr(Literal::IntLiteral(int))          => Some(int.to_string()),
           _                                                => None
        }
    ).collect::<Vec<String>>()
}

// return Result<Expr, error>
pub fn typing(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(Literal::IntLiteral(_)) = &args[0] {
            return &args[0];
        }
    }
    &args[0]
}

// return Result<Expr, error>
pub fn wait(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(Literal::IntLiteral(_)) = &args[0] {
            return &args[0];
        }
    }
    &args[0]
}

// return Result<Expr, error>
pub fn text(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(_) = &args[0] {
            return &args[0];
        }
    }
    &args[0]
}

// return Result<Expr, error>
pub fn button(args: &[Expr]) -> Message {
    if args.len() == 2 {
        if let (Expr::LitExpr(Literal::StringLiteral(arg1)), Expr::VecExpr(arg2)) = (&args[0], &args[1]) {
            return Message {
                my_type: "Button".to_string(),
                content: Content::Button(arg1.to_string(), exprvec_to_vec(arg2))
            }
        }
    }
    Message {my_type: "say".to_owned(), content: Content::Button("Button".to_owned(), vec![])}
}

// return Result<Expr, error>
pub fn url(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(_) = &args[0] {
            return &args[0];
        }
    }
    &args[0]
}

// return Result<Expr, error>
pub fn one_of(args: &[Expr]) -> &Expr {
    &args[rand::thread_rng().gen_range(0, args.len())]
}
