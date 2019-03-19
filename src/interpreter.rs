pub mod message;
pub mod csml_rules;
pub mod ast_interpreter;
pub mod builtins;

use crate::parser::ast::*;
use message::*;
use csml_rules::*;
use ast_interpreter::*;

use std::io::*;
// use std::io::{Error, ErrorKind};

pub struct Interpreter {
    pub ast: Flow,
    pub start: bool,
    pub accept_flow: bool,
}

impl Interpreter {
    pub fn new(ast: Flow) -> Interpreter {
        Interpreter {
            ast,
            start: false,
            accept_flow: false,
        }
    }

    fn check_valid_flow(&mut self) -> bool {
        for step in self.ast.iter() {
            match step {
                Step::FlowStarter { .. }        => self.accept_flow = true,
                Step::Block { label, .. }       =>
                    match label {
                        Ident(t) if t == "start"    => self.start = true,
                        _                           => {}
                    },
            }
        }
        self.start && self.accept_flow && double_label(&self.ast)
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
                    let json = match_block(actions).unwrap();
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
            println!("The Flow is not valid it need a start/end Labels, a Accept Flow, and each label must be unique");
            return;
        }
        loop {
            let read = read_standar_in();

            match (read, &json) {
                (Ok(..), Some(string)) => {
                    let deserialized: RootInterface = serde_json::from_str(&string).unwrap();
                    json = self.search_for(&deserialized.next_step.unwrap());
                },
                (Ok(ref string), None) if string.trim() == "flow"   => {
                    // check if flow can start
                    self.search_for("flow");
                    json = self.search_for("start");
                },
                (Ok(ref string), None) if string.trim() == "exit"   => break,
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
