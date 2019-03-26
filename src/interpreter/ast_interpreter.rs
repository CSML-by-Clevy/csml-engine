use std::io::{Error, ErrorKind, Result};
use crate::parser::ast::*;
use crate::interpreter::{
    builtins::*,
    message::*,
    csml_rules::*,
    json_to_rust::*,
};
// use std::collections::HashMap;

// struct StepInfo<'a> {
//     name: &'a str,
//     var: HashMap<&'a str, &'a Expr>,
//     retry: i32
// }

// // return Result<struct, error>
// pub fn match_flowstarter(ident: &Ident, list: &[Expr]) {
//     println!("{:?} - {:?}", ident, list);
// }

pub struct AstInterpreter<'a> {
    pub context: &'a JsContext,
    pub event: &'a Option<Event>,
}

impl<'a> AstInterpreter<'a>
{
    fn match_var(&self, var: &Ident, action: &Expr) -> Result<MessageType> {
        match var {
            Ident(arg) if arg == "event"   => Ok(MessageType::Msg(Message::new(action, "Text".to_string()))),
            Ident(_arg)                    => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
        }
    }

    fn match_builtin(&self, builtin: &Ident, args: &Expr) -> Result<MessageType> {
        match builtin {
            Ident(arg) if arg == "Typing"=> Ok(MessageType::Msg(Message::new(typing(args), arg.to_string()))),
            Ident(arg) if arg == "Wait"  => Ok(MessageType::Msg(Message::new(wait(args), arg.to_string()))),
            Ident(arg) if arg == "Text"  => Ok(MessageType::Msg(Message::new(text(args), arg.to_string()))),
            Ident(arg) if arg == "Url"   => Ok(MessageType::Msg(Message::new(url(args), arg.to_string()))),
            Ident(arg) if arg == "OneOf" => Ok(MessageType::Msg(Message::new(one_of(args), "Text".to_string()))),
            Ident(arg) if arg == "Button"=> Ok(button(args)),
            // arg                          => self.match_var(arg, args),
            Ident(_arg)                  => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
        }
    }

    fn match_action(&self, action: &Expr) -> Result<MessageType> {
        match action {
            Expr::Action { builtin, args }  => self.match_builtin(builtin, args),
            Expr::LitExpr(_literal)         => Ok(MessageType::Msg(Message::new(action, "Text".to_string()))),
            Expr::Empty                     => Ok(MessageType::Empty),
            _                               => Err(Error::new(ErrorKind::Other, "Error must be a valid action")),
        }
    }

    fn match_reserved(&self, reserved: &Ident, arg: &Expr) -> Result<MessageType> {
        match reserved {
            Ident(ident) if ident == "say"      => {
                self.match_action(arg)
            }
            Ident(ident) if ident == "ask"      => {
                // TMP implementation of block for an action
                if let Expr::VecExpr(block) = arg {
                    match self.match_block(block) { 
                        Ok(root)  => Ok(MessageType::Msgs(root.message)),
                        Err(e)    => Err(e)
                    }
                } else {
                    //check for info
                    self.match_action(arg)
                    //check if retry > 1
                }
            }
            Ident(ident) if ident == "retry"    => {
                // check nbr
                // save new option if exist
                self.match_action(arg)
            }
            _                                   => {
                Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
            }
        }
    }

    // Can be rm if we want to have multiple ask in the same Step
    fn match_reserved_if(&self, reserved: &Ident, arg: &Expr) -> Result<MessageType> {
        match reserved {
            Ident(ident) if ident == "say"      => {
                self.match_action(arg)
            }
            Ident(ident) if ident == "retry"    => {
                // check nbr
                // save new option if exist 
                self.match_action(arg)
            }
            _                                   => {
                Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
            }
        }
    }

    // TMP implementation of block for an action
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

    fn match_ifexpr(&self, cond: &Expr, consequence: &[Expr]) -> Result<RootInterface> {
       println!("> condition > {:?}", cond);
        if check_infixexpr(cond) {
            let mut root = RootInterface {remember: None, message: vec![], next_flow: None , next_step: None};
            for expr in consequence {
                if root.next_step.is_some() {
                    return Ok(root)
                }

                match expr {
                    Expr::Reserved { fun, arg }         => {
                        match self.match_reserved_if(fun, arg) {
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
                        match self.match_ifexpr(cond, consequence) {
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

    // TMP implementation of block for an action
    pub fn match_block(&self, actions: &[Expr]) -> Result<RootInterface> {
        self.check_valid_step(actions);
        let mut root = RootInterface {remember: None, message: vec![], next_flow: None , next_step: None};

        for action in actions {
            //TODO: check ask
            if root.next_step.is_some() {
                return Ok(root)
            }

            match action {
                Expr::Reserved { fun, arg } => {
                    match self.match_reserved(fun, arg) {
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
                    match self.match_ifexpr(cond, consequence) {
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
}