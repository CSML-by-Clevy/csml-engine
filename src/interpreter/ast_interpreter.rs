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

fn cmp_lit(infix: &Infix, lit1: Result<Literal, String>, lit2: Result<Literal, String>) -> Result<Literal, String> {
    match (infix, lit1, lit2) {
        (Infix::Equal, Ok(l1), Ok(l2) )             => Ok(Literal::BoolLiteral(l1 == l2)),
        (Infix::GreaterThanEqual, Ok(l1), Ok(l2))   => Ok(Literal::BoolLiteral(l1 >= l2)),
        (Infix::LessThanEqual, Ok(l1), Ok(l2))      => Ok(Literal::BoolLiteral(l1 <= l2)),
        (Infix::GreaterThan, Ok(l1), Ok(l2))        => Ok(Literal::BoolLiteral(l1 > l2)),
        (Infix::LessThan, Ok(l1), Ok(l2))           => Ok(Literal::BoolLiteral(l1 < l2)),
        (Infix::Or, Ok(..), Ok(..))                 => Ok(Literal::BoolLiteral(true)),
        (Infix::Or, Ok(..), Err(..))                => Ok(Literal::BoolLiteral(true)),
        (Infix::Or, Err(..), Ok(..))                => Ok(Literal::BoolLiteral(true)),
        (Infix::And, Ok(..), Ok(..))                => Ok(Literal::BoolLiteral(true)),
        (Infix::Adition, Ok(l1), Ok(l2))            => l1 + l2,
        (Infix::Substraction, Ok(l1), Ok(l2))       => l1 - l2,
        (Infix::Divide, Ok(l1), Ok(l2))             => l1 / l2,
        (Infix::Multiply, Ok(l1), Ok(l2))           => l1 * l2,
        _                                           => Ok(Literal::BoolLiteral(false)),
    }
}

fn check_if_ident(expr: &Expr) -> bool {
    match expr {
        Expr::LitExpr{..}           => true,
        Expr::IdentExpr(..)         => true,
        Expr::BuilderExpr(..)       => true,
        Expr::ComplexLiteral(..)    => true,
        _                           => false
    }
}

pub fn evaluate_condition(infix: &Infix, expr1: &Expr, expr2: &Expr, memory: &Memory, event: &Option<Event>) -> Result<Literal, String> {
    match (expr1, expr2) {
        (exp1, exp2) if check_if_ident(exp1) && check_if_ident(exp2)   => {
            cmp_lit(infix, get_var_from_ident(memory, event, exp1), get_var_from_ident(memory, event, exp2))
        },
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2))      => {
            cmp_lit(infix, evaluate_condition(i1, ex1, ex2, memory, event), evaluate_condition(i2, exp1, exp2, memory, event))
        },
        (Expr::InfixExpr(i1, ex1, ex2), exp)                => {
            cmp_lit(infix, evaluate_condition(i1, ex1, ex2, memory, event), gen_literal_form_exp(memory, event, exp))
        },
        (exp, Expr::InfixExpr(i1, ex1, ex2))                => {
            cmp_lit(infix, gen_literal_form_exp(memory, event, exp), evaluate_condition(i1, ex1, ex2, memory, event))
        }
        (_, _)                                              => Err("error in evaluate_condition function".to_owned()),
    }
}

impl<'a> AstInterpreter<'a> {
    // return Result<Expr, String, error>
    fn valid_condition(&self, expr: &Expr) -> bool {
        match expr {
            Expr::InfixExpr(inf, exp1, exp2)    => {
                match evaluate_condition(inf, exp1, exp2, self.memory, self.event) {
                    Ok(Literal::BoolLiteral(false)) => false,
                    Ok(_)                           => true,
                    Err(_e)                         => false
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

    fn match_builtin(&self, builtin: &str, args: &Expr) -> Result<MessageType, String> {
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
    fn match_functions(&self, action: &Expr) -> Result<MessageType, String> {
        match action {
            Expr::FunctionExpr(ReservedFunction::Normal(name, variable)) => self.match_builtin(&name, variable),
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
            Expr::InfixExpr(infix, exp1, exp2) => {
                match evaluate_condition(infix, exp1, exp2, self.memory, self.event) {
                    Ok(val) => Ok(MessageType::Msg(Message::new(&Expr::LitExpr{lit: val}, INT.to_string()))),
                    Err(e)  => Err(e)
                }
            },
            Expr::IdentExpr(ident)          => {
                match get_var(self.memory, self.event, ident) {
                    Ok(val) => Ok(MessageType::Msg(Message::new(&Expr::LitExpr{lit: val}, INT.to_string()))),
                    Err(_e)  => Ok(MessageType::Msg(Message::new(&Expr::LitExpr{lit: Literal::StringLiteral("NULL".to_owned())}, TEXT.to_string()))),//Err(e)
                }
            },
            _                               => Err("Error must be a valid action".to_owned()),
        }
    }

    fn match_actions(&self, function: &ReservedFunction, root: &mut RootInterface) -> Result<bool, String> {
        match function {
        ReservedFunction::Say(arg)                      => {
                let msgtype = self.match_functions(arg)?;

                self.add_to_message(root, msgtype);
            },
            ReservedFunction::Goto(.., step_name)       => { root.add_next_step(&step_name) },
            ReservedFunction::Remember(name, variable)  => { // if self.check_if_ident(variable)
                if let Ok(Literal::StringLiteral(variable)) = get_var_from_ident(self.memory, self.event, variable) {
                    self.add_to_message(root, remember(name.to_string(), variable.to_string()));
                }
                // } else {
                //     return Err("Error Assign value must be valid"));
                // }
            },
            // (ReservedFunction::Retry, arg)      => {
            _                                           => {return Err("Error Assign value must be valid".to_owned())}
        };
        Ok(true)
    }

    fn match_ask_response(&self, vec: &[Expr], root: RootInterface) -> Result<RootInterface, String> {
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
        Err("error sub block arg must be of type Expr::VecExpr".to_owned())
    }

    pub fn interpret_block(&self, actions: &[Expr]) -> Result<RootInterface, String> {
        let mut root = RootInterface {memories: None, messages: vec![], next_flow: None, next_step: None};

        for action in actions {
            if root.next_step.is_some() {
                return Ok(root)
            }

            match action {
                Expr::FunctionExpr(fun)                                         => { self.match_actions(fun, &mut root)?; },
                Expr::IfExpr { cond, consequence }                              => {
                    if self.valid_condition(cond) {
                        root = root + self.interpret_block(consequence)?;
                    }
                    // else {
                    //     return Err("error in if condition, it does not reduce to a boolean expression "));
                    // }
                },
                Expr::Block { block_type: BlockType::AskResponse, arg: vec }    => {
                    root = self.match_ask_response(vec, root)?;
                }
                _                                                               => return Err("Block must start with a reserved keyword".to_owned()),
            };
        }
        Ok(root)
    }
}