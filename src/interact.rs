use crate::parser::ast::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Action {
    m_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    action: Action,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
struct RootInterface {
    remember: Option<Vec<String>>,
    message: Vec<Message>,
    next_step: Option<String>,
}

pub fn test_json() {
    let point = RootInterface {
        remember: Option::None,
        message: vec![Message {
            action: Action {
                m_type: "say".to_owned(),
            },
            content: Content {
                text: "text".to_owned(),
            },
        }],
        next_step: Option::None,
    };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: RootInterface = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

//check start flow

// return Result<struct, error>
fn parse_block(_label: &Ident, actions: &Vec<Expr>) {
    for action in actions {
        match action {
            Expr::Reserved { fun, arg }         => match_reserved(fun, arg),
            Expr::Goto(ident)                   => println!("goto -> {:?}", ident),
            Expr::IfExpr { cond, consequence }  => match_ifexpr(cond, consequence),
            // Expr::Action { builtin, args }      => match_builtin(builtin, args),
            _                                   => println!("error block must start with a reserved keyword"),
            // Expr::InfixExpr(infix, expr)        => println!("{:?} {:?}", infix, expr),
            // Expr::LitExpr(literal)              => println!("{:?}", literal),
            // Expr::IdentExpr(ident)              => println!("{:?}", ident),
            // Expr::VecExpr(vec)                  => println!("{:?}", vec),
        }
    }
    println!("==========================");
}

// return Result<struct, error>
fn match_reserved(reserved: &Ident, _arg: &Box<Expr>)
{
    match reserved {
        Ident(arg) if arg == "say"      => println!("say"),
        Ident(arg) if arg == "ask"      => println!("ask"),
        Ident(arg) if arg == "retry"    => println!("retry"),
        _                               => println!("error")
    }
}

fn check_infixexpr(exprs: &[Expr]) -> bool
{
    for expr in exprs.iter() {
        let res = match expr {
            Expr::InfixExpr(_, _)   => true, 
            _                       => false 
        };
        if !res { return false; }
    };
    true
}

fn eval_condition(cond: &Vec<Expr>) -> bool
{
    match cond.split_last() {
        Some((last, elements)) 
            if match last { 
                Expr::LitExpr(_e) => true, _ => false 
                } && check_infixexpr(elements)
                            => true,
            _               => false
    }
}

// return Result<struct, error>
fn match_ifexpr(cond: &Vec<Expr>, consequence: &Vec<Expr>)
{
    if eval_condition(cond) {
        for expr in consequence {
            match expr {
                Expr::Reserved { fun, arg }         => match_reserved(fun, arg),
                Expr::Goto(ident)                   => println!("goto -> {:?}", ident),
                Expr::IfExpr { cond, consequence }  => match_ifexpr(cond, consequence),
                _                                   => println!(" Error in If block "),
            }
        }
    } else {
        //replace with return error
        println!("error in if condition it does not reduce to a boolean expression");
    }
    // eval condition
    // matche actions
}

// return Result<struct, error>
fn match_builtin(builtin: &Ident, _args: &Vec<Expr>)
{
    match builtin {
        Ident(arg) if arg == "Typing"   => println!("Typing"),
        Ident(arg) if arg == "Text"     => println!("Text"),
        Ident(arg) if arg == "Wait"     => println!("Wait"),
        Ident(arg) if arg == "Url"      => println!("Botton"),
        Ident(arg) if arg == "Button"   => println!("Button"),
        Ident(arg) if arg == "OneOf"    => println!("Oneof"),
        _                               => println!("error")
    }
}

pub fn parse_flow(flow: Flow) {
    for step in flow.iter() {
        match step {
            Step::FlowStarter { ident, list } => println!(" +-------- {:?} {:?}", ident, list),
            Step::Block { label, actions } => parse_block(label, actions),
        }
    }
}
