use std::collections::HashMap;
use crate::parser::{ast::*, tokens::*};
use crate::interpreter:: {
    builtins::{api_functions::*, reserved_functions::*},
    message::*,
    variable_handler::*,
    data::Data
};

fn cmp_lit(infix: &Infix, lit1: Result<Literal, String>, lit2: Result<Literal, String>) -> Result<Literal, String> {
    match (infix, lit1, lit2) {
        (Infix::NotEqual, Ok(l1), Ok(l2) )          => Ok(Literal::BoolLiteral(l1 != l2)),
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

pub fn evaluate_condition(infix: &Infix, expr1: &Expr, expr2: &Expr, data: &mut Data) -> Result<Literal, String> {
    match (expr1, expr2) {
        (exp1, exp2) if check_if_ident(exp1) && check_if_ident(exp2)        => {
            cmp_lit(infix, get_var_from_ident(exp1, data), get_var_from_ident(exp2, data))
        },
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2))    => {
            cmp_lit(infix, evaluate_condition(i1, ex1, ex2, data), evaluate_condition(i2, exp1, exp2, data))
        },
        (Expr::InfixExpr(i1, ex1, ex2), exp)                                => {
            cmp_lit(infix, evaluate_condition(i1, ex1, ex2, data), gen_literal_form_exp(exp, data))
        },
        (exp, Expr::InfixExpr(i1, ex1, ex2))                                => {
            cmp_lit(infix, gen_literal_form_exp(exp, data), evaluate_condition(i1, ex1, ex2, data))
        }
        (_, _)                                                              => Err("error in evaluate_condition function".to_owned()),
    }
}

// return Result<Expr, String, error>
fn valid_condition(expr: &Expr, data: &mut Data) -> bool {
    match expr {
        Expr::InfixExpr(inf, exp1, exp2)    => {
            match evaluate_condition(inf, exp1, exp2, data) {
                Ok(Literal::BoolLiteral(false)) => false,
                Ok(_)                           => true,
                Err(_e)                         => false
            }
        },
        Expr::LitExpr{..}                   => true,
        Expr::BuilderExpr(..)               => get_var_from_ident(expr, data).is_ok(), // error
        Expr::IdentExpr(ident)              => get_var(ident, data).is_ok(), // error
        _                                   => false, // return error
    }
}

fn add_to_message(root: RootInterface, action: MessageType) -> RootInterface {
    match action {
        MessageType::Msg(msg)            => root.add_message(msg),
        MessageType::Assign{name, value} => root.add_to_memory(name, value),
        MessageType::Empty               => root,
    }
}

fn match_builtin(object: Literal, data: &mut Data) -> Result<MessageType, String> {
    match object {
        Literal::ObjectLiteral{ref name, ref value} if name == TYPING   => typing(value, name.to_owned()),
        Literal::ObjectLiteral{ref name, ref value} if name == WAIT     => wait(value, name.to_owned()),
        Literal::ObjectLiteral{ref name, ref value} if name == TEXT     => text(value, name.to_owned()),
        Literal::ObjectLiteral{ref name, ref value} if name == URL      => url(value, name.to_owned()),
        Literal::ObjectLiteral{ref name, ref value} if name == IMAGE    => img(value, name.to_owned()),
        Literal::ObjectLiteral{ref name, ref value} if name == ONE_OF   => one_of(value, TEXT.to_owned(), data),
        Literal::ObjectLiteral{ref name, ref value} if name == QUESTION => question(value, name.to_owned(), data),
        Literal::ObjectLiteral{name: _, ref value}                     => api(value, data),
    _                                                                   => Err("buitin format Error".to_owned()),
    }
}

fn expr_to_literal(expr: &Expr, data: &mut Data) -> Result<Literal, String> {
    match expr {
        Expr::FunctionExpr(ReservedFunction::As(name, var))        => {
            let value = expr_to_literal(var, data)?;
            data.step_vars.insert(name.to_owned(), value.clone());
            Ok(value)
        },
        Expr::FunctionExpr(ReservedFunction::Normal(name, var))    => {
            let mut obj = HashMap::new();
            let expr: &Expr = var;

            if let Expr::VecExpr(vec) = expr {
                for elem in vec.iter() {
                    match elem {
                        Expr::FunctionExpr(ReservedFunction::Assign(name, var))    => {
                            obj.insert(name.to_owned(), expr_to_literal(var, data)?);
                        },
                        _   => {
                            let value = expr_to_literal(elem, data)?;
                            obj.insert(value.type_to_string(), expr_to_literal(var, data)?);
                        },
                    }
                }
            }
            Ok(Literal::ObjectLiteral{name: name.to_owned(), value: obj})
        },
        Expr::ComplexLiteral(vec)                                       => Ok(get_string_from_complexstring(vec, data)),
        Expr::VecExpr(vec)                                              => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data)?)
            }
            Ok(Literal::ArrayLiteral(array))
        },
        Expr::IdentExpr(var)                                            => get_var(var, data),
        // Expr::BuilderExpr(expr1, expr2)                              => Ok(),
        Expr::LitExpr{lit: literal}                                     => Ok(literal.clone()),
        _                                                               => Err(format!("ERROR: Expr {:?} can't be converted to Literal", expr).to_owned())
    }
}

//NOTE: Add detection variable type
fn match_functions(action: &Expr, data: &mut Data) -> Result<MessageType, String> {
    match action {
        Expr::FunctionExpr(ReservedFunction::As(name, expr)) => {
            let msg = match_functions(expr, data)?;

            match msg {
                MessageType::Msg(Message{ref content, ..})        => {data.step_vars.insert(name.to_owned(), content.clone());},
                MessageType::Assign{..}                           => {},
                MessageType::Empty                                => {}
            };
            Ok(msg)
        },
        Expr::FunctionExpr(ReservedFunction::Normal(..)) => match_builtin(expr_to_literal(action, data)?, data),
        Expr::BuilderExpr(..)           => {
            match get_var_from_ident(action, data) {
                Ok(val) => Ok(MessageType::Msg(Message::new(&val, TEXT.to_string()))),
                Err(e)  => Err(e)
            }
        },
        Expr::ComplexLiteral(vec)       => {
            Ok(MessageType::Msg(
                Message::new(
                    &get_string_from_complexstring(vec, data),
                    TEXT.to_string()
                )
            ))
        },
        Expr::InfixExpr(infix, exp1, exp2) => {
            match evaluate_condition(infix, exp1, exp2, data) {
                Ok(val) => Ok(MessageType::Msg(Message::new(&val, INT.to_string()))),
                Err(e)  => Err(e)
            }
        },
        Expr::IdentExpr(ident)          => {
            match get_var(ident, data) {
                Ok(val)     => Ok(MessageType::Msg(Message::new(&val, INT.to_string()))),
                Err(_e)     => Ok(MessageType::Msg(Message::new(&Literal::StringLiteral("NULL".to_owned()), TEXT.to_string()))),//Err(e)
            }
        },
        Expr::LitExpr{..}                  => Ok(MessageType::Msg(Message::new(&expr_to_literal(action, data)?, TEXT.to_string()))),
        Expr::VecExpr(..)                 => Ok(MessageType::Msg(Message::new(&expr_to_literal(action, data)?, "Array".to_string()))),
        err                                => Err(format!("Error must be a valid function {:?}", err)),
    }
}

fn match_actions(function: &ReservedFunction, root: RootInterface, data: &mut Data) -> Result<RootInterface, String> {
    match function {
        ReservedFunction::Say(arg)                      => {
            let msgtype = match_functions(arg, data)?;
            Ok(add_to_message(root, msgtype))
        },
        ReservedFunction::Use(arg)                      => {
            match_functions(arg, data)?;
            Ok(root)
        },
        ReservedFunction::Goto(.., step_name)       => Ok(root.add_next_step(&step_name)),
        ReservedFunction::Remember(name, variable)  => { // if self.check_if_ident(variable)
            if let Ok(Literal::StringLiteral(variable)) = get_var_from_ident(variable, data) {
                Ok(add_to_message(root, remember(name.to_string(), variable.to_string())))
            } else {
                Ok(root)
            //     return Err("Error Assign value must be valid"));
            }
        },
        ReservedFunction::Import{step_name: name, ..}          => {
            if let Some(Expr::Block{arg: actions, ..}) = data.ast.flow_instructions.get(&InstructionType::NormalStep(name.to_string())) {
                match interpret_block(&actions, data) {
                    Ok(root2)  => Ok(root + root2),
                    Err(..)     => Err("Error in import function".to_owned())
                }
            } else {
                Err(format!("Error step {} not found in flow", name))
            }
        }
        
        // (ReservedFunction::Retry, arg)      => {
        _                                           => {Err("Error must be a valid action".to_owned())}
    }
}

fn match_ask_response(vec: &[Expr], root: RootInterface, data: &mut Data) -> Result<RootInterface, String> {
    for block in vec.iter() {
        match (block, data.event) {
            (Expr::Block{block_type: BlockType::Ask, arg: args}, None)              => {
                return Ok(root + interpret_block(args, data)?);
            },
            (Expr::Block{block_type: BlockType::Response, arg: args}, Some(..))     => {
                return Ok(root + interpret_block(args, data)?);
            },
            (_, _)                                                                  => continue,
        }
    }
    Err("error sub block arg must be of type Expr::VecExpr".to_owned())
}

pub fn interpret_block(actions: &[Expr], data: &mut Data) -> Result<RootInterface, String> {
    let mut root = RootInterface {memories: None, messages: vec![], next_flow: None, next_step: None};

    for action in actions {
        if root.next_step.is_some() {
            return Ok(root)
        }

        match action {
            Expr::FunctionExpr(fun)                                         => { root = match_actions(fun, root, data)?; },
            Expr::IfExpr { cond, consequence }                              => {
                if valid_condition(cond, data) {
                    root = root + interpret_block(consequence, data)?;
                }
                // else {
                //     return Err("error in if condition, it does not reduce to a boolean expression "));
                // }
            },
            Expr::Block { block_type: BlockType::AskResponse, arg: vec }    => {
                root = match_ask_response(vec, root, data)?;
            }
            _                                                               => return Err("Block must start with a reserved keyword".to_owned()),
        };
    }
    Ok(root)
}
