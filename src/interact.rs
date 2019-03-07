use crate::builtins::*;
use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::io::*;
use std::ops::Add;
use std::io::{Error, ErrorKind};
// use std::result::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Action {
    m_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Content {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    action: Action,
    content: Content,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RootInterface {
    remember: Option<Vec<String>>,
    message: Vec<Message>,
    next_step: Option<String>,
}

impl Add for RootInterface {
    type Output = RootInterface;

    fn add(self, other: RootInterface) -> RootInterface {
        RootInterface {
            remember: None,
            message: [&self.message[..], &other.message[..]].concat(),
            next_step: match (self.next_step, other.next_step) {
                (None, None)    => None,
                (None, t)       => t,
                (t, None)       => t,
                (_, _)          => panic!("ERROR bad paring can not have too goto at same time"),
            },
        }
    }
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
    msg: RootInterface,
}

impl Interpreter {
    pub fn new(ast: Flow) -> Interpreter {
        Interpreter {
            ast,
            start: false,
            end: false,
            accept_flow: false,
            msg: RootInterface {
                remember: None,
                message: vec![],
                next_step: None,
            },
        }
    }

    fn jump(&mut self, l: &Ident, to_ask: bool) {
        for step in self.ast.iter() {
            // let _res =
            match step {
                Step::Block { label, actions } if label == l => {self.match_block(l, actions);},
                _                                            => {}
            }
        }
    }

    fn check_valid_flow(&mut self) -> bool {
        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { .. } => self.accept_flow = true,
                Step::Block { label, .. } => match label {
                    Ident(t) if t == "start" => self.start = true,
                    Ident(t) if t == "end" => self.end = true,
                    _ => {}
                },
            }
        }
        // Check if no double label with same name
        self.start && self.end && self.accept_flow
    }

    fn check_valid_step(&self, step: &[Expr]) -> bool {
        let mut nbr = 0;

        for expr in step {
            if let Expr::Reserved { fun, .. } = expr {
                match fun {
                    Ident(ident) if ident == "ask" => nbr += 1,
                    _ => {}
                }
            }
        }
        nbr < 2
    }

    fn match_block(&self, _label: &Ident, actions: &[Expr]) -> Result<RootInterface> {
        self.check_valid_step(actions);
        let mut res = RootInterface {remember: None, message: vec![], next_step: None};

        for action in actions {
            //check goto and mabe ask
            res = match action {
                Expr::Reserved { fun, arg }         => {
                    match_reserved(fun, arg);
                    res + RootInterface {remember: None, message: vec![], next_step: None}
                },
                Expr::Goto(Ident(t))                => {
                    res + RootInterface {remember: None, message: vec![], next_step: Some(t.to_string())}
                },
                Expr::IfExpr { cond, consequence }  => {
                    match_ifexpr(cond, consequence);
                    res + RootInterface {remember: None, message: vec![], next_step: None}
                },
                _                                   => return Err(Error::new(ErrorKind::Other, "Block must start with a reserved keyword")),
            };
        }

        Ok(RootInterface {remember: None, message: vec![], next_step: None})
    }

    // return Result<struct, error>
    pub fn serch_for(&self, name: &str) {
        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { ident, list } if check_ident(ident, name) => {
                    match_flowstrarter(ident, list)
                }
                Step::Block { label, actions } if check_ident(label, name) => {
                    self.match_block(label, actions);
                }
                _ => continue,
            }
        }
    }

    pub fn interpret(&mut self) {
        if !self.check_valid_flow() {
            println!("The Flow is not valid it need a start , end Labels and a Accept Flow");
            return;
        }

        loop {
            let read = read_standar_in();
            println!("{:?}", read);

            match read {
                Ok(ref string) if string.trim() == "exit" => break,
                Ok(ref string) if string.trim() == "flow" => {
                    // check if flow can start
                    self.serch_for("flow");
                    self.serch_for("start");
                }
                Ok(ref string) if string.trim() == "hello" => {
                    self.serch_for("hello");
                }
                Ok(_string) => continue,
                Err(e) => {
                    println!("Error => {:?}", e);
                    break;
                }
            }
        }
    }
}

fn read_standar_in() -> Result<String> {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

// ################### match ast structure

// return Result<struct, error>
fn match_flowstrarter(ident: &Ident, list: &[Expr]) {
    println!("{:?} - {:?}", ident, list);
}

// return Result<struct, error>
fn match_action(action: &Expr) {
    match action {
        Expr::Action { builtin, args }  => match_builtin(builtin, args),
        Expr::LitExpr(literal)          => println!("--> literal {:?}", literal),
        _                               => println!("error block must start with a reserved keyword"),
    }
}

// return Result<struct, error>
fn match_reserved(reserved: &Ident, arg: &Expr) {
    match reserved {
        Ident(ident) if ident == "say"  => {
            print!("say ");
            match_action(arg)
        }
        Ident(ident) if ident == "ask"  => {
            print!("ask ");
            match_action(arg)
        }
        Ident(ident) if ident == "retry" => {
            print!("retry ");
            match_action(arg)
        }
        _                                => {
            print!("error");
        }
    }
}

// return Result<struct, error>
fn match_reserved_if(reserved: &Ident, arg: &Expr) {
    match reserved {
        Ident(ident) if ident == "say" => {
            print!("say ");
            match_action(arg)
        }
        Ident(ident) if ident == "retry" => {
            print!("retry ");
            match_action(arg)
        }
        _ => {
            print!("error");
        }
    }
}

// return Result<struct, error>
fn match_builtin(builtin: &Ident, _args: &[Expr]) {
    match builtin {
        Ident(arg) if arg == "Typing"   => println!("Typing"),
        Ident(arg) if arg == "Wait"     => println!("Wait"),
        Ident(arg) if arg == "Text"     => println!("Text"),
        Ident(arg) if arg == "Button"   => println!("Button"),
        Ident(arg) if arg == "Url"      => println!("Url"),
        Ident(arg) if arg == "OneOf"    => println!("Oneof"),
        Ident(arg)                      => println!("Error no buitin with name {}", arg),
    }
}

// return Result<struct, error>
fn match_ifexpr(cond: &[Expr], consequence: &[Expr]) {
    println!("If");
    if eval_condition(cond) {
        for expr in consequence {
            match expr {
                Expr::Reserved { fun, arg } => match_reserved_if(fun, arg),
                Expr::Goto(ident) => println!("goto -> {:?}", ident),
                Expr::IfExpr { cond, consequence } => match_ifexpr(cond, consequence),
                _ => println!("Error in If block "),
            }
        }
    } else {
        //replace with return error
        println!(
            "error in if condition does not reduce to a boolean expression -> {:?}",
            cond
        );
    }
    // eval condition
    // matche actions
}

// ################ structure rules for CSML

fn check_infixexpr(exprs: &[Expr]) -> bool {
    for expr in exprs.iter() {
        let res = match expr {
            Expr::InfixExpr(_, _) => true,
            _ => false,
        };
        if !res {
            return false;
        }
    }
    true
}

fn check_ident(expr: &Ident, name: &str) -> bool {
    match expr {
        Ident(string) if string == name => true,
        _ => false,
    }
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

fn is_variable(expr: &Expr) -> bool {
    match expr {
        Expr::LitExpr(_e) => true,
        Expr::IdentExpr(_e) => true,
        _ => false,
    }
}

fn eval_condition(cond: &[Expr]) -> bool {
    match cond.split_last() {
        Some((last, elements)) if is_variable(last) && check_infixexpr(elements) => true,
        _ => false,
    }
}
