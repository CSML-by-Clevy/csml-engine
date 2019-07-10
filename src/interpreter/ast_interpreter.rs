use std::collections::HashMap;
use crate::parser::{ast::*, tokens::*};
use crate::interpreter:: {
    builtins::{api_functions::*, reserved_functions::*},
    message::*,
    variable_handler::*,
    data::Data
};

fn match_obj(lit1: &Literal, lit2: &Literal) -> Result<Literal, String> {
    let _b = BUTTON.to_owned();
    if let Literal::ObjectLiteral{name: _b, ..} = lit2 {
        if let MessageType::Msg(Message{content: Literal::ObjectLiteral{value, ..} , ..}) = match_builtin(lit2.clone())? {
            if let Some(Literal::ArrayLiteral(vec)) = value.get("payload") {
                return Ok(Literal::BoolLiteral(vec.contains(lit1)));
            }
        }
    }
    println!("end of match not fouand");
    // TODO: TMP default return
    Ok(Literal::BoolLiteral(lit1 == lit2))
}

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
        (Infix::Match, Ok(ref l1), Ok(ref l2))      => match_obj(l1, l2),
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

fn check_if_not_operator(infix: &Infix) -> bool {
    if let Infix::Not = infix { true } else { false }
}

pub fn evaluate_condition(infix: &Infix, expr1: &Expr, expr2: &Expr, data: &mut Data) -> Result<Literal, String> {
    match (expr1, expr2) {
        (exp1, ..) if check_if_not_operator(infix) && check_if_ident(exp1)  => {
            match get_var_from_ident(exp1, data) {
                Ok(Literal::BoolLiteral(false)) => Ok(Literal::BoolLiteral(true)),
                Ok(Literal::IntLiteral(0))      => Ok(Literal::BoolLiteral(true)),
                Ok(..)                          => Ok(Literal::BoolLiteral(false)),
                Err(..)                         => Ok(Literal::BoolLiteral(true))
            }
        },
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
        Expr::IdentExpr(ident, ..)          => get_var(ident, data).is_ok(), // error
        _                                   => false, // return error
    }
}

fn add_to_message(root: RootInterface, action: MessageType) -> RootInterface {
    match action {
        MessageType::Msg(msg)            => root.add_message(msg),
        MessageType::Empty               => root,
    }
}

fn match_builtin(object: Literal) -> Result<MessageType, String> {
    match object {
        Literal::ObjectLiteral{ref name, ref value} if name == TYPING   => typing(value),
        Literal::ObjectLiteral{ref name, ref value} if name == WAIT     => wait(value),
        Literal::ObjectLiteral{ref name, ref value} if name == TEXT     => text(value),
        Literal::ObjectLiteral{ref name, ref value} if name == URL      => url(value),
        Literal::ObjectLiteral{ref name, ref value} if name == IMAGE    => img(value),
        Literal::ObjectLiteral{ref name, ref value} if name == ONE_OF   => one_of(value),
        Literal::ObjectLiteral{ref name, ref value} if name == QUESTION => question(value, name.to_owned()),
        Literal::ObjectLiteral{ref name, ref value} if name == BUTTON   => button(value),
        Literal::ObjectLiteral{ref name, ref value} if name == API      => api(value),
        Literal::ObjectLiteral{..}                                      => Ok(MessageType::Msg(Message::new(&object))),
        _                                                               => unreachable!(),
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
        Expr::ComplexLiteral(vec)                           => Ok(get_string_from_complexstring(vec, data)),
        Expr::VecExpr(vec)                                  => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data)?)
            }
            Ok(Literal::ArrayLiteral(array))
        },
        Expr::IdentExpr(var, ..)                            => get_var(var, data),
        Expr::LitExpr(literal, ..)                          => Ok(literal.clone()),
        // Expr::BuilderExpr(expr1, expr2)                  => Ok(),
        _                                                   => Err(format!("ERROR: Expr {:?} can't be converted to Literal", expr).to_owned())
    }
}

fn match_functions(action: &Expr, data: &mut Data) -> Result<MessageType, String> {
    match action {
        Expr::FunctionExpr(ReservedFunction::As(name, expr)) => {
            let msg = match_functions(expr, data)?;

            match msg {
                MessageType::Msg(Message{ref content, ..})        => {data.step_vars.insert(name.to_owned(), content.clone());},
                MessageType::Empty                                => {}
            };
            Ok(msg)
        },
        Expr::FunctionExpr(ReservedFunction::Normal(..)) => match_builtin(expr_to_literal(action, data)?),
        Expr::BuilderExpr(..)           => {
            match get_var_from_ident(action, data) {
                Ok(val) => Ok(MessageType::Msg(Message::new(&val))),
                Err(e)  => Err(e)
            }
        },
        Expr::ComplexLiteral(vec)       => {
            Ok(MessageType::Msg(
                Message::new(&get_string_from_complexstring(vec, data))
            ))
        },
        Expr::InfixExpr(infix, exp1, exp2) => {
            match evaluate_condition(infix, exp1, exp2, data) {
                Ok(val) => Ok(MessageType::Msg(Message::new(&val))),
                Err(e)  => Err(e)
            }
        },
        Expr::IdentExpr(ident, ..)         => {
            match get_var(ident, data) {
                Ok(val)     => Ok(MessageType::Msg(Message::new(&val))),
                Err(_e)     => Ok(MessageType::Msg(Message::new(&Literal::StringLiteral("NULL".to_owned())))),//Err(e)
            }
        },
        Expr::LitExpr{..}                  => Ok(MessageType::Msg(Message::new(&expr_to_literal(action, data)?))),
        Expr::VecExpr(..)                  => Ok(MessageType::Msg(Message::new(&expr_to_literal(action, data)?))),
        err                                => Err(format!("Error must be a valid function {:?}", err)),
    }
}

fn match_actions(function: &ReservedFunction, mut root: RootInterface, data: &mut Data) -> Result<RootInterface, String> {
    match function {
        ReservedFunction::Say(arg)                      => {
            Ok(add_to_message(root, match_functions(arg, data)?))
        },
        ReservedFunction::Use(arg)                      => {
            match_functions(arg, data)?;
            Ok(root)
        },
        ReservedFunction::Goto(GotoType::Step, step_name)=> Ok(root.add_next_step(&step_name)),
        ReservedFunction::Goto(GotoType::Flow, flow_name)=> Ok(root.add_next_flow(&flow_name)),
        ReservedFunction::Remember(name, variable)  => {
            root = root.add_to_memory(name.to_owned(), get_var_from_ident(variable, data)?);
            Ok(root)
        },
        ReservedFunction::Import{step_name: name, ..}    => {
            if let Some(Expr::Block{arg: actions, ..}) = data.ast.flow_instructions.get(&InstructionType::NormalStep(name.to_string())) {
                match interpret_block(&actions, data) {
                    Ok(root2)  => Ok(root + root2),
                    Err(err)     => Err(format!("Error in import function {:?}", err))
                }
            } else {
                Err(format!("Error step {} not found in flow", name))
            }
        }
        _                                               => {Err("Error must be a valid action".to_owned())}
    }
}

fn match_ask_response(vec: &[Expr], mut root: RootInterface, data: &mut Data, opt: &Option<String>) -> Result<RootInterface, String> {
    for block in vec.iter() {
        match (block, data.event) {
            (Expr::Block{block_type: BlockType::Ask, arg: args}, None)              => {
                return Ok(root + interpret_block(args, data)?);
            },
            (Expr::Block{block_type: BlockType::Response, arg: args}, Some(..))     => {
                if let Some(ident) = opt {
                   root = root.add_to_memory(ident.to_owned(), gen_literal_form_event(data.event)?);
                }
                return Ok(root + interpret_block(args, data)?);
            },
            (_, _)                                                                  => continue,
        }
    }
    Err("error sub block arg must be of type Expr::VecExpr".to_owned())
}

pub fn solve_if_statments(statment: &IfStatement, mut root: RootInterface, data: &mut Data) -> Result<RootInterface, String>{
    match statment {
        IfStatement::IfStmt{cond, consequence, then_branch}  => {
            if valid_condition(cond, data) {
                root = root + interpret_block(consequence, data)?;
                return Ok(root);
            } 
            if let Some(then) = then_branch {
                return solve_if_statments(then, root, data);
            }
            Ok(root)
        }
        IfStatement::ElseStmt(consequence)                   => {
            root = root + interpret_block(consequence, data)?;
            Ok(root)
        }
    }
}

pub fn interpret_block(actions: &[Expr], data: &mut Data) -> Result<RootInterface, String> {
    let mut root = RootInterface {memories: None, messages: vec![], next_flow: None, next_step: None};

    for action in actions {
        if root.next_step.is_some() {
            return Ok(root)
        }

        match action {
            Expr::FunctionExpr(fun)                                             => { root = match_actions(fun, root, data)?; },
            Expr::IfExpr(ref ifstatement)                                       => root = solve_if_statments(ifstatement, root, data)?,
            Expr::Block { block_type: BlockType::AskResponse(opt), arg: vec }   => {
                root = match_ask_response(vec, root, data, opt)?;
            }
            _                                                                   => return Err("Block must start with a reserved keyword".to_owned()),
        };
    }
    Ok(root)
}
