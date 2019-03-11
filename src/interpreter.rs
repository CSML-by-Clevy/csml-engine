use crate::builtins::*;
use crate::parser::ast::*;
use serde::{Deserialize, Serialize};
use std::io::*;
use std::ops::Add;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content {
    Text(String),
    Button(String, Vec<String>)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "type")]
    pub my_type: String,
    pub content: Content,
}

impl Message {
    pub fn new(expr: &Expr) -> Self {
        let mut msg = Message {
            my_type: "".to_string(),
            content: Content::Text("".to_string())
        };

        match expr {
            Expr::LitExpr(Literal::IntLiteral(val))     => {msg.my_type = "Int".to_string(); msg.content = Content::Text(val.to_string()); msg},
            Expr::LitExpr(Literal::StringLiteral(val))  => {msg.my_type = "Text".to_string(); msg.content = Content::Text(val.to_string()); msg},
            Expr::LitExpr(Literal::BoolLiteral(val))    => {msg.my_type = "Bool".to_string(); msg.content = Content::Text(val.to_string()); msg},
            _                                           => {msg},
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootInterface {
    pub remember: Option<Vec<String>>,
    pub message: Vec<Message>,
    pub next_step: Option<String>,
}

impl Add for RootInterface {
    type Output = RootInterface;

    // return Result<struct, error>
    fn add(self, other: RootInterface) -> RootInterface {
        RootInterface {
            remember: None,
            message: [&self.message[..], &other.message[..]].concat(),
            next_step: match (self.next_step, other.next_step) {
                (None, None)    => None,
                (None, t)       => t,
                (t, None)       => t,
                (_, _)          => panic!("ERROR bad paring can't have too goto at same time"),
            },
        }
    }
}

impl RootInterface {
    // fn add_remeber(){}
    fn add_message(&mut self, message: Message) {
        self.message.push(message);
    }

    fn add_next_step(&mut self, next_step: &str) {
        self.next_step = Some(next_step.to_string());
    }
}

// json example
pub fn test_json() {
    let point = RootInterface {
        remember: Option::None,
        message: vec![Message {
            my_type: "say".to_owned(),
            content: Content::Text("text".to_owned()),
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
            accept_flow: false,
        }
    }

    fn check_valid_flow(&mut self) -> bool {
        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { .. }        => self.accept_flow = true,
                Step::Block { label, .. }       => match label {
                    Ident(t) if t == "start"    => self.start = true,
                    Ident(t) if t == "end"      => self.end = true,
                    _                           => {}
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
                    Ident(ident) if ident == "ask"  => nbr += 1,
                    _                               => {}
                }
            }
        }
        nbr < 2
    }

    fn match_block(&self, _label: &Ident, actions: &[Expr]) -> Result<RootInterface> {
        self.check_valid_step(actions);
        let mut root = RootInterface {remember: None, message: vec![], next_step: None};

        for action in actions {
            //check goto and maybe ask
            if root.next_step.is_some() {
                return Ok(root)
            }

            match action {
                Expr::Reserved { fun, arg } => {
                    match match_reserved(fun, arg) {
                        Ok(action)  => root.add_message(action),
                        Err(err)    => return Err(err)
                    }
                },
                Expr::IfExpr { cond, consequence }  => {
                    match match_ifexpr(cond, consequence) {
                        Ok(action)  => root = root + action,
                        Err(err)    => return Err(err)
                    }
                },
                Expr::Goto(Ident(ident))    => root.add_next_step(ident),
                _                           => return Err(Error::new(ErrorKind::Other, "Block must start with a reserved keyword")),
            };
        }
        Ok(root)
    }

    // return Result<struct, error>
    pub fn search_for(&self, name: &str) -> Option<String> {
        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { ident, list } if check_ident(ident, name) => {
                    match_flowstarter(ident, list);
                    return None;
                }
                Step::Block { label, actions } if check_ident(label, name) => {
                    let json = self.match_block(label, actions).unwrap();
                    let ser = serde_json::to_string(&json).unwrap();
                    println!("--------> {}", ser);
                    return Some(ser);
                }
                _ => continue,
            }
        }
        None
    }

    pub fn interpret(&mut self) {
        if !self.check_valid_flow() {
            println!("The Flow is not valid it need a start , end Labels and a Accept Flow");
            return;
        }

        let mut json: Option<String> = None;
        loop {
            let read = read_standar_in();
            // println!("{:?}", read);

            match (read, &json) {
                (Ok(_), Some(string)) => {
                    let deserialized: RootInterface = serde_json::from_str(&string).unwrap();
                    json = self.search_for(&deserialized.next_step.unwrap());
                },
                (Ok(ref string), None) if string.trim() == "exit"   => break,
                (Ok(ref string), None) if string.trim() == "flow"   => {
                    // check if flow can start
                    self.search_for("flow");
                    json = self.search_for("start");
                },
                (Ok(ref string), None) if string.trim() == "hello"  => {
                    self.search_for("hello");
                },
                (Ok(_string), None)                                 => continue,
                (Err(e), _)                                      => {
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
fn match_flowstarter(ident: &Ident, list: &[Expr]) {
    println!("{:?} - {:?}", ident, list);
}

fn match_action(action: &Expr) -> Result<Message> {
    match action {
        Expr::Action { builtin, args }  => match_builtin(builtin, args),
        Expr::LitExpr(_literal)         => Ok(Message::new(action)),
        _                               => Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword")),
    }
}

fn match_reserved(reserved: &Ident, arg: &Expr) -> Result<Message> {
    match reserved {
        Ident(ident) if ident == "say"      => {
            match_action(arg)
        }
        Ident(ident) if ident == "ask"      => {
            match_action(arg)
        }
        Ident(ident) if ident == "retry"    => {
            match_action(arg)
        }
        _                                   => {
            Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
        }
    }
}

fn match_reserved_if(reserved: &Ident, arg: &Expr) -> Result<Message>{
    match reserved {
        Ident(ident) if ident == "say"      => {
            match_action(arg)
        }
        Ident(ident) if ident == "retry"    => {
            match_action(arg)
        }
        _                                   => {
            Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
        }
    }
}

fn match_builtin(builtin: &Ident, args: &[Expr]) -> Result<Message> {
    match builtin {
        Ident(arg) if arg == "Typing"   => Ok(Message::new(typing(args))),
        Ident(arg) if arg == "Wait"     => Ok(Message::new(wait(args))),
        Ident(arg) if arg == "Text"     => Ok(Message::new(text(args))),
        Ident(arg) if arg == "Url"      => Ok(Message::new(url(args))),
        Ident(arg) if arg == "OneOf"    => Ok(Message::new(one_of(args))),
        Ident(arg) if arg == "Button"   => Ok(button(args)),
        Ident(_arg)                     => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
    }
}

fn match_ifexpr(cond: &[Expr], consequence: &[Expr]) -> Result<RootInterface>{
    if eval_condition(cond) {
        let mut root = RootInterface {remember: None, message: vec![], next_step: None};

        for expr in consequence {
            if root.next_step.is_some() {
                return Ok(root)
            }

            match expr {
                Expr::Reserved { fun, arg }         => {
                    match match_reserved_if(fun, arg) {
                        Ok(msg)   => root.add_message(msg),
                        Err(err)  => return Err(err)
                    }
                },
                Expr::IfExpr { cond, consequence }  => {
                    match match_ifexpr(cond, consequence) {
                        Ok(msg)   => root = root + msg,
                        Err(err)  => return Err(err)
                    }
                },
                Expr::Goto(Ident(ident))            => root.add_next_step(ident),
                _                                   => return Err(Error::new(ErrorKind::Other, "Error in If block")),
            }
        }
        Ok(root)
    } else {
        Err(Error::new(ErrorKind::Other, "error in if condition, it does not reduce to a boolean expression "))
    }
}

// ################ structure rules for CSML

fn check_infixexpr(exprs: &[Expr]) -> bool {
    for expr in exprs.iter() {
        let res = match expr {
            Expr::InfixExpr(_, _)   => true,
            _                       => false,
        };
        if !res { return false; }
    }
    true
}

fn check_ident(expr: &Ident, name: &str) -> bool {
    match expr {
        Ident(string) if string == name => true,
        _                               => false,
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
        Expr::LitExpr(_e)   => true,
        Expr::IdentExpr(_e) => true,
        _                   => false,
    }
}

fn eval_condition(cond: &[Expr]) -> bool {
    match cond.split_last() {
        Some((last, elements)) if is_variable(last) && check_infixexpr(elements) => true,
        _ => false,
    }
}
