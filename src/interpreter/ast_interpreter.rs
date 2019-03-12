use crate::parser::ast::*;
use crate::interpreter::{
    builtins::*,
    message::*,
    csml_rules::*
};

use std::io::*;
use std::io::{Error, ErrorKind};

// return Result<struct, error>
pub fn match_flowstarter(ident: &Ident, list: &[Expr]) {
    println!("{:?} - {:?}", ident, list);
}

pub fn match_action(action: &Expr) -> Result<Message> {
    match action {
        Expr::Action { builtin, args }  => match_builtin(builtin, args),
        Expr::LitExpr(_literal)         => Ok(Message::new(action)),
        _                               => Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword")),
    }
}

pub fn match_reserved(reserved: &Ident, arg: &Expr) -> Result<Message> {
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

pub fn match_reserved_if(reserved: &Ident, arg: &Expr) -> Result<Message>{
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

pub fn match_builtin(builtin: &Ident, args: &[Expr]) -> Result<Message> {
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

pub fn match_ifexpr(cond: &[Expr], consequence: &[Expr]) -> Result<RootInterface>{
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