use std::io::{Error, ErrorKind, Result};
use crate::parser::ast::*;
use crate::lexer::token::*;
use crate::interpreter:: {
    builtins::*,
    message::*,
    json_to_rust::*,
    variable_handler::*,
};

pub struct AstInterpreter<'a> {
    pub memory: &'a Memory,
    pub event: &'a Option<Event>,
}

impl<'a> AstInterpreter<'a> {
    fn match_builtin(&self, builtin: &Ident, args: &Expr) -> Result<MessageType> {
        match builtin {
            Ident(arg) if arg == TYPING         => typing(args, arg.to_string()),
            Ident(arg) if arg == WAIT           => wait(args, arg.to_string()),
            Ident(arg) if arg == TEXT           => text(args, arg.to_string()),
            Ident(arg) if arg == URL            => url(args, arg.to_string()),
            Ident(arg) if arg == IMAGE          => img(args, arg.to_string()),

            Ident(arg) if arg == ONE_OF         => one_of(args, "text".to_owned(), self.memory, self.event),
            Ident(arg) if arg == QUESTION       => question(args, arg.to_string(), self.memory, self.event),
            
            Ident(arg) if arg == METEO          => meteo(args, self.memory, self.event),
            Ident(arg) if arg == WTTJ           => wttj(args, self.memory, self.event),

            Ident(arg) if arg == GET_GSHEET     => get_gsheet(args, self.memory, self.event),
            Ident(arg) if arg == APPEND_GSHEET  => append_gsheet(args, self.memory, self.event),
            Ident(arg) if arg == HUB_SPOT       => hub_spot(args, self.memory, self.event),
            Ident(arg) if arg == AWS            => aws(args, self.memory, self.event),

            Ident(_arg)                         => Err(Error::new(ErrorKind::Other, "Error no builtin found")),
        }
    }

    // Match reserved Actions ###############################################################
    fn match_action(&self, action: &Expr) -> Result<MessageType> {
        match action {
            Expr::Action { builtin, args }  => self.match_builtin(builtin, args),
            Expr::LitExpr(_)                => Ok(MessageType::Msg(Message::new(action, "text".to_string()))),
            //NOTE: ONLY Work for LITERAR::STRINGS for now
            Expr::BuilderExpr(..)           =>
                match get_var_from_ident(self.memory, self.event, action) {
                    Ok(val) => Ok(MessageType::Msg(Message::new(&Expr::LitExpr(val), "text".to_string()))),
                    Err(e)  => Err(e)
                }
            ,
            Expr::ComplexLiteral(vec)       => {
                Ok(MessageType::Msg(
                    Message::new(
                        &Expr::LitExpr(get_string_from_complexstring(self.memory, self.event, vec)),
                        "text".to_string()
                    )
                ))
            },
            Expr::Empty                     => Ok(MessageType::Empty),
            _                               => Err(Error::new(ErrorKind::Other, "Error must be a valid action")),
        }
    }

    fn match_reserved(&self, reserved: &Ident, arg: &Expr) -> Result<MessageType> {
        match reserved {
            Ident(ident) if ident == SAY      => {
                self.match_action(arg)
            }
            Ident(ident) if ident == RETRY    => {
                self.match_action(arg)
            }
            _                                   => {
                Err(Error::new(ErrorKind::Other, "Error block must start with a reserved keyword"))
            }
        }
    }

    // Match reserved Actions ###############################################################
    fn cmp_lit(&self, infix: &Infix, lit1: &Literal, lit2: &Literal) -> bool {
        match infix {
            Infix::Equal                => lit1 == lit2,
            Infix::GreaterThanEqual     => lit1 >= lit2,
            Infix::LessThanEqual        => lit1 <= lit2,
            Infix::GreaterThan          => lit1 > lit2,
            Infix::LessThan             => lit1 < lit2,
            Infix::And                  => true,
            Infix::Or                   => true,
        }
    }

    fn cmp_bool(&self, infix: &Infix, b1: bool, b2: bool) -> bool {
        match infix {
            Infix::Equal                => b1 == b2,
            Infix::GreaterThanEqual     => b1 >= b2,
            Infix::LessThanEqual        => b1 <= b2,
            Infix::GreaterThan          => b1 & !b2,
            Infix::LessThan             => !b1 & b2,
            Infix::And                  => b1 && b2,
            Infix::Or                   => b1 || b2,
        }
    }

    fn gen_literal_form_exp(&self, expr: &Expr) -> Result<Literal> {
        match expr {
            Expr::LitExpr(lit)      => Ok(lit.clone()),
            Expr::IdentExpr(ident)  => get_var(self.memory, self.event, ident),
            _                       => Err(Error::new(ErrorKind::Other, "Expression must be a literal or an identifier"))
        }
    }

    // NOTE: see if IDENT CAN HAVE BUILDER_IDENT inside and LitExpr can have ComplexLiteral
    fn check_if_ident(&self, expr: &Expr) -> bool {
        match expr {
            Expr::LitExpr(..)         => true,
            Expr::IdentExpr(..)       => true,
            Expr::BuilderExpr(..)     => true,
            Expr::ComplexLiteral(..)  => true,
            _                         => false
        }
    }

    fn evaluate_condition(&self, infix: &Infix, expr1: &Expr, expr2: &Expr) -> Result<bool> {
        match (expr1, expr2) {
            (Expr::LitExpr(l1), Expr::LitExpr(l2))          => {
                Ok(self.cmp_lit(infix, l1, l2))
            },
            (exp1, exp2) if self.check_if_ident(exp1) && self.check_if_ident(exp2)    => {
                match (get_var_from_ident(self.memory, self.event, exp1), get_var_from_ident(self.memory, self.event, exp2)) {
                    (Ok(l1), Ok(l2))   => Ok(self.cmp_lit(infix, &l1, &l2)),
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between ident and ident"))
                }
            },
            (exp, Expr::LitExpr(l2)) if self.check_if_ident(exp)  => {
                match get_var_from_ident(self.memory, self.event, exp) {
                    Ok(l1) => Ok(self.cmp_lit(infix, &l1, l2)),
                    _      => Err(Error::new(ErrorKind::Other, "error in evaluation between ident and lit"))
                }
            },
            (Expr::LitExpr(l1), exp) if self.check_if_ident(exp)   => {
                match get_var_from_ident(self.memory, self.event, exp) {
                    Ok(l2) => Ok(self.cmp_lit(infix, l1, &l2)),
                    _      => Err(Error::new(ErrorKind::Other, "error in evaluation between lit and ident"))
                }
            },
            (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2))      => {
                match (self.evaluate_condition(i1, ex1, ex2), self.evaluate_condition(i2, exp1, exp2)) {
                    (Ok(l1), Ok(l2))   => Ok(self.cmp_bool(infix, l1, l2)),
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between InfixExpr and InfixExpr"))
                }
            },
            (Expr::InfixExpr(i1, ex1, ex2), exp)                => {
                match (self.evaluate_condition(i1, ex1, ex2), self.gen_literal_form_exp(exp)) {
                    (Ok(e1), Ok(_)) => {
                        match infix {
                            Infix::And                  => Ok(e1),
                            Infix::Or                   => Ok(true),
                            _                           => Err(Error::new(ErrorKind::Other, "error need && or ||"))
                        }
                    },
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between InfixExpr and expr"))
                }
            },
            (exp, Expr::InfixExpr(i1, ex1, ex2))                => {
                match (self.gen_literal_form_exp(exp), self.evaluate_condition(i1, ex1, ex2)) {
                    (Ok(_), Ok(e2)) => {
                        match infix {
                            Infix::And                  => Ok(e2),
                            Infix::Or                   => Ok(true),
                            _                           => Err(Error::new(ErrorKind::Other, "error need && or ||"))
                        }
                    },
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between expr and InfixExpr"))
                }
            }
            (_, _)                                              => Err(Error::new(ErrorKind::Other, "error in evaluate_condition function")),
        }
    }

    // return Result<Expr, error>
    fn valid_condition(&self, expr: &Expr) -> bool {
        match expr {
            Expr::InfixExpr(inf, exp1, exp2)    => {
                match self.evaluate_condition(inf, exp1, exp2) {
                    Ok(rep) => rep,
                    Err(_e)  => false
                }
            },
            Expr::LitExpr(_lit)                 => true,
            Expr::BuilderExpr(..)               => get_var_from_ident(self.memory, self.event, expr).is_ok(), // error
            Expr::IdentExpr(ident)              => get_var(self.memory, self.event, ident).is_ok(), // error
            _                                   => false, // return error
        }
    }

    fn add_to_message(&self, root: &mut RootInterface, action: MessageType) {
        match action {
            // MessageType::Msgs(msgs)          => root.messages.extend(msgs),
            MessageType::Msg(msg)            => root.add_message(msg),
            MessageType::Assign{name, value} => root.add_to_memory(name, value),
            MessageType::Empty               => {},
        }
    }

    fn match_sub_block(&self, arg: &Expr, root: RootInterface) -> Result<RootInterface> {
        if let Expr::VecExpr(vec) = arg {
            Ok(root + self.match_block(vec)?)
        } else {
            return Err(Error::new(ErrorKind::Other, "error sub block arg must be of type Expr::VecExpr"))
        }
    }

    pub fn match_block(&self, actions: &[Expr]) -> Result<RootInterface> {
        let mut root = RootInterface {memories: None, messages: vec![], next_flow: None, next_step: None};

        for action in actions {
            if root.next_step.is_some() {
                return Ok(root)
            }

            match action {
                Expr::Reserved { fun, arg } => {
                    match (fun, self.event) {
                        (Ident(ref name), None) if name == ASK                    => {
                            return self.match_sub_block(arg, root);
                        },
                        (Ident(ref name), Some(..)) if name == RESPOND            => {
                            return self.match_sub_block(arg, root);
                        },
                        (Ident(ref name), _) if name == RESPOND || name == ASK  => continue,
                        (_, _)                                                      => {
                            self.add_to_message(
                                &mut root,
                                self.match_reserved(fun, arg)?
                            )
                        }
                    };
                },
                Expr::IfExpr { cond, consequence } => {
                    if self.valid_condition(cond) {
                        root = root + self.match_block(consequence)?;
                    } 
                    // else {
                    //     return Err(Error::new(ErrorKind::Other, "error in if condition, it does not reduce to a boolean expression "));
                    // }
                },
                Expr::Goto(Ident(ident))    => {
                    root.add_next_step(ident)
                },
                Expr::Remember(Ident(name), expr) if self.check_if_ident(expr) => {
                        if let Ok(Literal::StringLiteral(var)) = get_var_from_ident(self.memory, self.event, expr) {
                            self.add_to_message(&mut root, remember(name.to_string(), var.to_string()));
                        } else {
                            return Err(Error::new(ErrorKind::Other, "Error Assign value must be valid"));
                        }
                    },
                _                           => return Err(Error::new(ErrorKind::Other, "Block must start with a reserved keyword")),
            };
        }
        Ok(root)
    }
}