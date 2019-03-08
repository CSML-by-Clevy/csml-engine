use crate::parser::ast::*;
use rand::Rng;

// return Result<Expr, error>
pub fn typing(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(Literal::IntLiteral(_)) = &args[0] {
            return &args[0];
        }
    }
    return &args[0];
}

// return Result<Expr, error>
pub fn wait(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(Literal::IntLiteral(_)) = &args[0] {
            return &args[0];
        }
    }
    return &args[0];
}

// return Result<Expr, error>
pub fn text(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(_) = &args[0] {
            return &args[0];
        }
    }
    return &args[0];
}

// return Result<Expr, error>
pub fn button(args: &[Expr]) {
    println!("button -> {:?}", args);
}

// return Result<Expr, error>
pub fn url(args: &[Expr]) -> &Expr {
    if args.len() == 1 {
        if let Expr::LitExpr(_) = &args[0] {
            return &args[0];
        }
    }
    return &args[0];
}

// return Result<Expr, error>
pub fn one_of(args: &[Expr]) -> &Expr {
    &args[rand::thread_rng().gen_range(0, args.len())]
}
