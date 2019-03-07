use crate::interact::*;
use rand;
use crate::parser::ast::*;

pub fn typing(args: &[Expr])
{
    println!("typing -> {:?}", args);
}

pub fn wait(args: &[Expr])
{
    println!("wait -> {:?}", args);
}

pub fn text(args: &[Expr])
{
    println!("text -> {:?}", args);
}

pub fn button(args: &[Expr])
{
    println!("button -> {:?}", args);
}

pub fn url(args: &[Expr])
{
    println!("url -> {:?}", args);
}

pub fn one_of(args: &[Expr])
{
    for arg in args.iter() {
        match arg {
            Expr::VecExpr(expr) => println!("one of => {:?}", expr),
            _                   => println!("Error")
        }
    }
}
