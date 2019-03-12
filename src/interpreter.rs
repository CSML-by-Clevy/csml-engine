pub mod message;
pub mod csml_rules;
pub mod ast_interpreter;
pub mod builtins;

use crate::parser::ast::*;
use message::*;
use csml_rules::*;
use ast_interpreter::*;

use std::io::*;
use std::io::{Error, ErrorKind};

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
            //check ask
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
        let mut json: Option<String> = None;

        if !self.check_valid_flow() {
            println!("The Flow is not valid it need a start , end Labels and a Accept Flow");
            return;
        }
        loop {
            let read = read_standar_in();

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
                (Err(e), _)                                         => {
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
