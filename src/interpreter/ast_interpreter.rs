use crate::parser::ast::*;
use crate::interpreter::{
    builtins::*,
    message::*,
    csml_rules::*
};

use std::io::{Error, ErrorKind, Result};

use std::collections::HashMap;

struct StepInfo<'a> {
    name: &'a str,
    var: HashMap<&'a str, &'a Expr>,
    retry: i32
}

// return Result<struct, error>
pub fn match_flowstarter(ident: &Ident, list: &[Expr]) {
    println!("{:?} - {:?}", ident, list);
}

pub fn match_action(action: &Expr) -> Result<MessageType> {
    match action {
        Expr::Action { builtin, args }  => match_builtin(builtin, args),
        Expr::LitExpr(_literal)         => Ok(MessageType::Msg(Message::new(action, "Text".to_string()))),
        Expr::Empty                     => Ok(MessageType::Empty),
        _                               => Err(Error::new(ErrorKind::Other, "Error must be a valid action")),
    }
}

pub fn match_builtin(builtin: &Ident, args: &Expr) -> Result<MessageType> {
    match builtin {
        Ident(arg) if arg == "Typing"=> Ok(MessageType::Msg(Message::new(typing(args), arg.to_string()))),
        Ident(arg) if arg == "Wait"  => Ok(MessageType::Msg(Message::new(wait(args), arg.to_string()))),
        Ident(arg) if arg == "Text"  => Ok(MessageType::Msg(Message::new(text(args), arg.to_string()))),
        Ident(arg) if arg == "Url"   => Ok(MessageType::Msg(Message::new(url(args), arg.to_string()))),
        Ident(arg) if arg == "OneOf" => Ok(MessageType::Msg(Message::new(one_of(args), "Text".to_string()))),
        Ident(arg) if arg == "Button"=> Ok(button(args)),
        Ident(_arg)                  => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
    }
}

pub fn match_reserved(reserved: &Ident, arg: &Expr) -> Result<MessageType> {
    match reserved {
        Ident(ident) if ident == "say"      => {
            match_action(arg)
        }
        Ident(ident) if ident == "ask"      => {
            // TMP implementation of block for an action
            if let Expr::VecExpr(block) = arg {
                match match_block(block) { 
                    Ok(root)  => Ok(MessageType::Msgs(root.message)),
                    Err(e)    => Err(e)
                }
            } else {
                //check for info
                match_action(arg)
                //check if retry > 1
            }
        }
        Ident(ident) if ident == "retry"    => {
            // check nbr
            // save new option if exist
            match_action(arg)
        }
        _                                   => {
            Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
        }
    }
}

// Can be rm if we want to have multiple ask in the same Step
pub fn match_reserved_if(reserved: &Ident, arg: &Expr) -> Result<MessageType> {
    match reserved {
        Ident(ident) if ident == "say"      => {
            match_action(arg)
        }
        Ident(ident) if ident == "retry"    => {
            // check nbr
            // save new option if exist 
            match_action(arg)
        }
        _                                   => {
            Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
        }
    }
}

// TMP implementation of block for an action
fn check_valid_step(step: &[Expr]) -> bool {
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

// TMP implementation of block for an action
// _label: &Ident,
pub fn match_block( actions: &[Expr]) -> Result<RootInterface> {
    check_valid_step(actions);
    let mut root = RootInterface {remember: None, message: vec![], next_flow: None , next_step: None};

    for action in actions {
        //check ask
        if root.next_step.is_some() {
            return Ok(root)
        }

        match action {
            Expr::Reserved { fun, arg } => {
                match match_reserved(fun, arg) {
                    Ok(action)  => {
                        match action {
                            MessageType::Msg(msg)   => root.add_message(msg),
                            MessageType::Msgs(msgs) => root.message.extend(msgs),
                            MessageType::Empty      => {},
                        }
                    },
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

pub fn match_ifexpr(cond: &[Expr], consequence: &[Expr]) -> Result<RootInterface> {
    if eval_condition(cond) {
        let mut root = RootInterface {remember: None, message: vec![], next_flow: None , next_step: None};
        // println!("condition is Ok === {:?}", cond);
        for expr in consequence {
            if root.next_step.is_some() {
                return Ok(root)
            }

            match expr {
                Expr::Reserved { fun, arg }         => {
                    match match_reserved_if(fun, arg) {
                        Ok(msg)   => {
                            match msg {
                                MessageType::Msg(msg)   => root.add_message(msg),
                                MessageType::Msgs(msgs) => root.message.extend(msgs),
                                MessageType::Empty      => {},
                            }
                        },
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