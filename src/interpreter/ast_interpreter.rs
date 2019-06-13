use std::io::{Error, ErrorKind, Result};
use crate::parser::{ast::*, tokens::*};
use crate::interpreter:: {
    builtins::{api_functions::*, reserved_functions::*},
    message::*,
    json_to_rust::*,
    variable_handler::*,
};

pub struct AstInterpreter<'a> {
    pub memory: &'a Memory,
    pub event: &'a Option<Event>,
}

impl<'a> AstInterpreter<'a> {
    fn cmp_lit(&self, infix: &Infix, lit1: &Literal, lit2: &Literal) -> bool {
        match infix {
            Infix::Equal                => lit1 == lit2,
            Infix::GreaterThanEqual     => lit1 >= lit2,
            Infix::LessThanEqual        => lit1 <= lit2,
            Infix::GreaterThan          => lit1 > lit2,
            Infix::LessThan             => lit1 < lit2,
            Infix::And                  => true,
            Infix::Or                   => true,
            _                           => true, // <= Arithmetic Operators
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
            _                           => true, // <= Arithmetic Operators
        }
    }

    // NOTE: see if IDENT CAN HAVE BUILDER_IDENT inside and LitExpr can have ComplexLiteral
    fn check_if_ident(&self, expr: &Expr) -> bool {
        match expr {
            Expr::LitExpr{..}           => true,
            Expr::IdentExpr(..)         => true,
            Expr::BuilderExpr(..)       => true,
            Expr::ComplexLiteral(..)    => true,
            _                           => false
        }
    }

    fn evaluate_condition(&self, infix: &Infix, expr1: &Expr, expr2: &Expr) -> Result<bool> {
        match (expr1, expr2) {
            (Expr::LitExpr{lit: l1}, Expr::LitExpr{lit: l2})          => {
                Ok(self.cmp_lit(infix, l1, l2))
            },
            (exp1, exp2) if self.check_if_ident(exp1) && self.check_if_ident(exp2)    => {
                match (get_var_from_ident(self.memory, self.event, exp1), get_var_from_ident(self.memory, self.event, exp2)) {
                    (Ok(l1), Ok(l2))   => Ok(self.cmp_lit(infix, &l1, &l2)),
                    _                  => Err(Error::new(ErrorKind::Other, "error in evaluation between ident and ident"))
                }
            },
            (exp, Expr::LitExpr{lit: l2}) if self.check_if_ident(exp)  => {
                match get_var_from_ident(self.memory, self.event, exp) {
                    Ok(l1) => Ok(self.cmp_lit(infix, &l1, l2)),
                    _      => Err(Error::new(ErrorKind::Other, "error in evaluation between ident and lit"))
                }
            },
            (Expr::LitExpr{lit: l1}, exp) if self.check_if_ident(exp)   => {
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
                match (self.evaluate_condition(i1, ex1, ex2), gen_literal_form_exp(self.memory, self.event, exp)) {
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
                match (gen_literal_form_exp(self.memory, self.event, exp), self.evaluate_condition(i1, ex1, ex2)) {
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
            Expr::LitExpr{..}                   => true,
            Expr::BuilderExpr(..)               => get_var_from_ident(self.memory, self.event, expr).is_ok(), // error
            Expr::IdentExpr(ident)              => get_var(self.memory, self.event, ident).is_ok(), // error
            _                                   => false, // return error
        }
    }

    fn add_to_message(&self, root: &mut RootInterface, action: MessageType) {
        match action {
            MessageType::Msg(msg)            => root.add_message(msg),
            MessageType::Assign{name, value} => root.add_to_memory(name, value),
            MessageType::Empty               => {},
        }
    }

    fn match_builtin(&self, builtin: &str, args: &Expr) -> Result<MessageType> {
        match builtin {
            arg if arg == TYPING         => typing(args, arg.to_string()),
            arg if arg == WAIT           => wait(args, arg.to_string()),
            arg if arg == TEXT           => text(args, arg.to_string()),
            arg if arg == URL            => url(args, arg.to_string()),
            arg if arg == IMAGE          => img(args, arg.to_string()),

            arg if arg == ONE_OF         => one_of(args, TEXT.to_owned(), self.memory, self.event),
            arg if arg == QUESTION       => question(args, arg.to_string(), self.memory, self.event),

            _                            => api(args, self.memory, self.event),
        }
    }

    //NOTE: ONLY Work for LITERAR::STRINGS for now
    fn match_functions(&self, action: &Expr) -> Result<MessageType> {
        match action {
            Expr::FunctionExpr(ReservedFunction::Normal(name), variable) => self.match_builtin(&name, variable),
            Expr::LitExpr{..}               => Ok(MessageType::Msg(Message::new(action, TEXT.to_string()))),
            Expr::BuilderExpr(..)           => {
                match get_var_from_ident(self.memory, self.event, action) {
                    Ok(val) => Ok(MessageType::Msg(Message::new(&Expr::LitExpr{lit: val}, TEXT.to_string()))),
                    Err(e)  => Err(e)
                }
            },
            Expr::ComplexLiteral(vec)       => {
                Ok(MessageType::Msg(
                    Message::new(
                        &Expr::LitExpr{lit: get_string_from_complexstring(self.memory, self.event, vec)},
                        TEXT.to_string()
                    )
                ))
            },
            _                               => Err(Error::new(ErrorKind::Other, "Error must be a valid action")),
        }
    }

    fn match_actions(&self, function: &ReservedFunction, expr: &Expr, root: &mut RootInterface) -> Result<bool> {
        match (function, expr) {
            (ReservedFunction::Say, arg)        => {
                let msgtype = self.match_functions(arg)?;

                self.add_to_message(root, msgtype);
            },
            (ReservedFunction::Goto(..), step_name) => { 
                match step_name {
                    Expr::IdentExpr(name) => root.add_next_step(&name),
                    _                     => return Err(Error::new(ErrorKind::Other, "Error Assign value must be valid"))
                };
            },
            (ReservedFunction::Remember(name), variable) => { // if self.check_if_ident(variable)
                if let Ok(Literal::StringLiteral(variable)) = get_var_from_ident(self.memory, self.event, variable) {
                    self.add_to_message(root, remember(name.to_string(), variable.to_string()));
                }
                // } else {
                //     return Err(Error::new(ErrorKind::Other, "Error Assign value must be valid"));
                // }
            },
            // (ReservedFunction::Retry, arg)      => {
            (_, _)                              => {return Err(Error::new(ErrorKind::Other, "Error Assign value must be valid"))}
        };
        Ok(true)
    }

    fn match_ask_response(&self, vec: &[Expr], root: RootInterface) -> Result<RootInterface> {
        for block in vec.iter() {
            match (block, self.event) {
                (Expr::Block{block_type: BlockType::Ask, arg: args}, None)              => {
                    return Ok(root + self.interpret_block(args)?);
                },
                (Expr::Block{block_type: BlockType::Response, arg: args}, Some(..))     => {
                    return Ok(root + self.interpret_block(args)?);
                },
                (_, _)                                                                  => continue,
            }
        }
        Err(Error::new(ErrorKind::Other, "error sub block arg must be of type Expr::VecExpr"))
    }

    pub fn interpret_block(&self, actions: &[Expr]) -> Result<RootInterface> {
        let mut root = RootInterface {memories: None, messages: vec![], next_flow: None, next_step: None};

        for action in actions {
            if root.next_step.is_some() {
                return Ok(root)
            }

            match action {
                Expr::FunctionExpr(fun, expr)                                   => { self.match_actions(fun, &expr, &mut root)?; },
                Expr::IfExpr { cond, consequence }                              => {
                    if self.valid_condition(cond) {
                        root = root + self.interpret_block(consequence)?;
                    }
                    // else {
                    //     return Err(Error::new(ErrorKind::Other, "error in if condition, it does not reduce to a boolean expression "));
                    // }
                },
                Expr::Block { block_type: BlockType::AskResponse, arg: vec }    => {
                    root = self.match_ask_response(vec, root)?;
                }
                _                                                               => return Err(Error::new(ErrorKind::Other, "Block must start with a reserved keyword")),
            };
        }
        Ok(root)
    }
}