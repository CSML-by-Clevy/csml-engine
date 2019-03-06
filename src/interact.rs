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

// json example
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

pub struct Interpreter {
    pub ast: Flow,
    pub start: bool,
    pub end: bool,
    pub accept_flow: bool,
}

impl Interpreter {
    pub fn new(ast: Flow) -> Interpreter {
        Interpreter {
            ast,
            start: false,
            end: false,
            accept_flow: false
        }
    }

    fn check_valid_flow(&mut self) -> bool {
        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { .. }   =>  { self.accept_flow = true },
                Step::Block { label, ..  }     => {
                    match label {
                        Ident(t) if t == "start" => self.start = true,
                        Ident(t) if t == "end"   => self.end = true,
                        _                        => {},
                    }
                },
            }
        }
        self.start && self.end && self.accept_flow
    }

    fn check_valid_step(&self, step: &[Expr]) -> bool {
        let mut nbr = 0;

        for expr in step {
            if let Expr::Reserved { fun, .. } = expr {
                match fun {
                    Ident(ident) if ident == "ask" => nbr += 1,
                    _                              => {}
                }
            }
        }
        nbr < 2
    }

    // return Result<struct, error>
    fn parse_block(&self, _label: &Ident, actions: &[Expr]) {
        
        // need to check if block for ask expr
        self.check_valid_step(actions);

        for action in actions {
            match action {
                Expr::Reserved { fun, arg }         => { match_reserved(fun, arg) },
                Expr::Goto(ident)                   => println!("goto -> {:?}", ident),
                Expr::IfExpr { cond, consequence }  => match_ifexpr(cond, consequence),
                _                                   => println!("error block must start with a reserved keyword"),
                // Expr::InfixExpr(infix, expr)        => println!("{:?} {:?}", infix, expr),
                // Expr::LitExpr(literal)              => println!("{:?}", literal),
                // Expr::IdentExpr(ident)              => println!("{:?}", ident),
                // Expr::VecExpr(vec)                  => println!("{:?}", vec),
            }
        }
        println!("==========================");
    }

    pub fn interpret(&mut self) {
        if !self.check_valid_flow() {
            println!("the Flow is not valid it need a start and end Label and a Accept Flow");
            return;
        }

        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { ident, list }   => match_flowstrarter(ident, list),
                Step::Block { label, actions }      => self.parse_block(label, actions),
            }
        }
    }
}

// ################### match ast structure

// return Result<struct, error>
fn match_flowstrarter(ident: &Ident, list: &[Expr])
{
    println!("{:?} - {:?}", ident, list);
}

// return Result<struct, error>
fn match_action(action: &Expr) {
    match action {
        Expr::Action { builtin, args }      => match_builtin(builtin, args),
        Expr::LitExpr(literal)              => println!("--> literal {:?}", literal),
        _                                   => println!("error block must start with a reserved keyword"),
    }
}

// return Result<struct, error>
fn match_reserved(reserved: &Ident, arg: &Expr)
{
    match reserved {
        Ident(ident) if ident == "say"      => {print!("say  "); match_action(arg)},
        Ident(ident) if ident == "ask"      => {print!("ask  "); match_action(arg)},
        Ident(ident) if ident == "retry"    => {print!("retry  "); match_action(arg)},
        _                                   => {print!("error"); }
    }
}

// return Result<struct, error>
fn match_builtin(builtin: &Ident, _args: &[Expr])
{
    match builtin {
        Ident(arg) if arg == "Typing"   => println!("Typing"),
        Ident(arg) if arg == "Wait"     => println!("Wait"),
        Ident(arg) if arg == "Text"     => println!("Text"),
        Ident(arg) if arg == "Button"   => println!("Button"),
        Ident(arg) if arg == "Url"      => println!("Url"),
        Ident(arg) if arg == "OneOf"    => println!("Oneof"),
        Ident(arg)                      => println!("Error no buitin with name {}", arg)
    }
}

// ################ structure rules for CSML
fn check_infixexpr(exprs: &[Expr]) -> bool
{
    for expr in exprs.iter() {
        let res = match expr {
            Expr::InfixExpr(_, _)   => true, 
            _                       => false 
        };
        if !res { return false;}
    };
    true
}

fn is_variable(expr: &Expr) -> bool
{
    match expr { 
        Expr::LitExpr(_e)   => true,
        Expr::IdentExpr(_e) => true,
        _                   => false
    }
}

fn eval_condition(cond: &[Expr]) -> bool
{
    match cond.split_last() {
        Some((last, elements)) 
            if is_variable(last) && check_infixexpr(elements)
                            => true,
            _               => false
    }
}
// #####################

// return Result<struct, error>
fn match_ifexpr(cond: &[Expr], consequence: &[Expr])
{
    println!("If");
    if eval_condition(cond) {
        for expr in consequence {
            match expr {
                Expr::Reserved { fun, arg }         => match_reserved(fun, arg),
                Expr::Goto(ident)                   => println!("goto -> {:?}", ident),
                Expr::IfExpr { cond, consequence }  => match_ifexpr(cond, consequence),
                _                                   => println!("Error in If block "),
            }
        }
    } else {
        //replace with return error
        println!("error in if condition it does not reduce to a boolean expression -> {:?}", cond);
    }
    // eval condition
    // matche actions
}

fn reserved_keywords(ident: &Ident)
{
    match ident {
        Ident(arg) if arg == "input"   => println!("input is the input get from client response"),
        _                              => println!("error at finding keyword")
    }
}
