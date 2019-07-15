use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    builtins::{api_functions::*, reserved_functions::*},
    data::Data,
    message::*,
    variable_handler::*,
};
use crate::parser::{ast::*, tokens::*};
use std::collections::HashMap;

fn match_obj(lit1: &SmartLiteral, lit2: &SmartLiteral) -> Result<Literal, ErrorInfo> {
    let _b = BUTTON.to_owned();
    if let Literal::ObjectLiteral { name: _b, .. } = lit2.literal.clone() {
        if let Literal::ObjectLiteral { value, .. } = match_builtin(lit2.clone())?.0 {
            match value.get("payload") {
                Some(Literal::ArrayLiteral(vec)) => {
                    return Ok(Literal::BoolLiteral(vec.contains(&lit1.literal)))
                }
                Some(val) => return Ok(Literal::BoolLiteral(val == &lit1.literal)),
                _ => return Ok(Literal::BoolLiteral(lit1.literal == lit2.literal)),
            }
        }
    }
    Ok(Literal::BoolLiteral(lit1 == lit2))
}

fn cmp_lit(
    infix: &Infix,
    lit1: Result<SmartLiteral, ErrorInfo>,
    lit2: Result<SmartLiteral, ErrorInfo>,
) -> Result<SmartLiteral, ErrorInfo> {
    match (infix, lit1, lit2) {
        (Infix::NotEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(l1 != l2), interval: l1.interval.to_owned()}),
        (Infix::Equal, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(l1 == l2), interval: l1.interval.to_owned()}),
        (Infix::GreaterThanEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(l1 >= l2), interval: l1.interval.to_owned()}),
        (Infix::LessThanEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(l1 <= l2), interval: l1.interval.to_owned()}),
        (Infix::GreaterThan, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(l1 > l2), interval: l1.interval.to_owned()}),
        (Infix::LessThan, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(l1 < l2), interval: l1.interval.to_owned()}),
        (Infix::Or, Ok(l1), Ok(..)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(true), interval: l1.interval.to_owned()}),
        (Infix::Or, Ok(l1), Err(..)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(true), interval: l1.interval.to_owned()}),
        (Infix::Or, Err(e), Ok(..)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(true), interval: e.interval.to_owned()}),
        (Infix::And, Ok(l1), Ok(..)) => Ok(SmartLiteral{literal: Literal::BoolLiteral(true), interval: l1.interval.to_owned()}),
        (Infix::Adition, Ok(l1), Ok(l2)) => l1 + l2,
        (Infix::Substraction, Ok(l1), Ok(l2)) => l1 - l2,
        (Infix::Divide, Ok(l1), Ok(l2)) => l1 / l2,
        (Infix::Multiply, Ok(l1), Ok(l2)) => l1 * l2,
        (Infix::Match, Ok(ref l1), Ok(ref l2)) => Ok(SmartLiteral{literal: match_obj(l1, l2)?, interval: l1.interval.to_owned()}),
        (_, Ok(l1), ..) => Ok(SmartLiteral{literal: Literal::BoolLiteral(false), interval: l1.interval.to_owned()}),
        (_, Err(e), ..) => Ok(SmartLiteral{literal: Literal::BoolLiteral(false), interval: e.interval.to_owned()}),
    }
}

fn check_if_ident(expr: &Expr) -> bool {
    match expr {
        Expr::LitExpr { .. } => true,
        Expr::IdentExpr(..) => true,
        Expr::BuilderExpr(..) => true,
        Expr::ComplexLiteral(..) => true,
        _ => false,
    }
}

fn check_if_not_operator(infix: &Infix) -> bool {
    if let Infix::Not = infix {
        true
    } else {
        false
    }
}

pub fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    data: &mut Data,
) -> Result<SmartLiteral, ErrorInfo> {
    match (expr1, expr2) {
        (exp1, ..) if check_if_not_operator(infix) && check_if_ident(exp1) => {
            // TODO: add interval in error
            match get_var_from_ident(exp1, data) {
                Ok(SmartLiteral {
                    literal: Literal::BoolLiteral(false),
                    interval,
                }) => Ok(SmartLiteral {
                    literal: Literal::BoolLiteral(true),
                    interval,
                }),
                Ok(SmartLiteral {
                    literal: Literal::IntLiteral(0),
                    interval,
                }) => Ok(SmartLiteral {
                    literal: Literal::BoolLiteral(true),
                    interval,
                }),
                Ok(SmartLiteral { interval, .. }) => Ok(SmartLiteral {
                    literal: Literal::BoolLiteral(false),
                    interval,
                }),
                Err(err) => Ok(SmartLiteral {
                    literal: Literal::BoolLiteral(true),
                    interval: err.interval,
                }),
            }
        }
        (exp1, exp2) if check_if_ident(exp1) && check_if_ident(exp2) => {
            let lit = cmp_lit(infix, get_var_from_ident(exp1, data), get_var_from_ident(exp2, data))?;
            Ok(SmartLiteral {literal: lit.literal, interval: interval_from_expr(exp1)})
        },
        (Expr::InfixExpr(i1, ex1, ex2), Expr::InfixExpr(i2, exp1, exp2)) => cmp_lit(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            evaluate_condition(i2, exp1, exp2, data),
        ),
        (Expr::InfixExpr(i1, ex1, ex2), exp) => cmp_lit(
            infix,
            evaluate_condition(i1, ex1, ex2, data),
            gen_literal_form_exp(exp, data),
        ),
        (exp, Expr::InfixExpr(i1, ex1, ex2)) => cmp_lit(
            infix,
            gen_literal_form_exp(exp, data),
            evaluate_condition(i1, ex1, ex2, data),
        ),
        (e1, _e2) => Err(
            ErrorInfo{
                message: "error in evaluate_condition function".to_owned(),
                interval: interval_from_expr(e1)
            }
        )
    }
}

fn valid_condition(expr: &Expr, data: &mut Data) -> bool {
    match expr {
        Expr::InfixExpr(inf, exp1, exp2) => match evaluate_condition(inf, exp1, exp2, data) {
            Ok(SmartLiteral{literal: Literal::BoolLiteral(false), ..}) => false,
            Ok(_) => true,
            Err(_e) => false,
        },
        Expr::LitExpr { .. } => true,
        Expr::BuilderExpr(..) => get_var_from_ident(expr, data).is_ok(), // error
        Expr::IdentExpr(ident, ..) => get_var(ident.to_owned(), data).is_ok(),      // error
        _ => false,                                                      // return error
    }
}

fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
    match action {
        MessageType::Msg(msg) => root.add_message(msg),
        MessageType::Empty => root,
    }
}

fn match_builtin(object: SmartLiteral) -> Result<(Literal, String), ErrorInfo> {
    match object.literal {
        Literal::ObjectLiteral {ref name, ref value} if name == TYPING => Ok((typing(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == WAIT => Ok((wait(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == TEXT => Ok((text(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == URL => Ok((url(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == IMAGE => Ok((img(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == ONE_OF => Ok((one_of(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == QUESTION => Ok((question(value, name.to_owned(), object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == BUTTON => Ok((button(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, ref value} if name == API => Ok((api(value, object.interval)?, name.to_owned())),
        Literal::ObjectLiteral {ref name, .. } => Ok((object.literal.to_owned(), name.to_owned())),
        _ => unreachable!(),
    }
}

fn expr_to_literal(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::FunctionExpr(ReservedFunction::As(name, var)) => {
            let value = expr_to_literal(var, data)?;
            data.step_vars.insert(name.ident.to_owned(), value.literal.clone());
            Ok(value)
        }
        Expr::FunctionExpr(ReservedFunction::Normal(name, var)) => {
            let mut obj: HashMap<String, Literal> = HashMap::new();
            let expr: &Expr = var;

            if let Expr::VecExpr(vec, _) = expr {
                for elem in vec.iter() {
                    match elem {
                        Expr::FunctionExpr(ReservedFunction::Assign(name, var)) => {
                            obj.insert(name.ident.to_owned(), expr_to_literal(var, data)?.literal);
                        }
                        _ => {
                            let value = expr_to_literal(elem, data)?.literal;
                            obj.insert(value.type_to_string(), value);
                        }
                    }
                }
            };
            Ok(SmartLiteral {
                    literal: Literal::ObjectLiteral {
                        name: name.ident.to_owned(),
                        value: obj,
                    },
                    interval: interval_from_expr(expr)
                }
            )
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data)?.literal)
            }
            Ok(SmartLiteral{
                    literal: Literal::ArrayLiteral(array),
                    interval: range.start.to_owned()
                }
            )
        }
        Expr::IdentExpr(var, ..) => Ok(get_var(var.to_owned(), data)?),
        Expr::LitExpr(literal) => Ok(literal.clone()),
        e => Err(
            ErrorInfo{
                message: format!("ERROR: Expr {:?} can't be converted to Literal", expr),
                interval: interval_from_expr(e)
            }
        )
    }
}

fn match_functions(action: &Expr, data: &mut Data) -> Result<MessageType, ErrorInfo> {
    match action {
        Expr::FunctionExpr(ReservedFunction::As(name, expr)) => {
            let msg = match_functions(expr, data)?;

            match msg {
                MessageType::Msg(Message { ref content, .. }) => {
                    data.step_vars.insert(name.ident.to_owned(), content.clone());
                }
                MessageType::Empty => {}
            };
            Ok(msg)
        }
        Expr::FunctionExpr(ReservedFunction::Normal(..)) => {
            let (literal, name) = match_builtin(expr_to_literal(action, data)?)?;
            Ok(MessageType::Msg(Message::new(literal, name)))
        },
        Expr::BuilderExpr(..) => match get_var_from_ident(action, data) {
            Ok(val) => {
                let literal = val.literal;
                let name = literal.type_to_string();
                Ok(MessageType::Msg(Message::new(literal, name)))
            },
            Err(e) => Err(e),
        },
        Expr::ComplexLiteral(vec, ..) => {
            let literal = get_string_from_complexstring(vec, data).literal;
            let name = literal.type_to_string();
            Ok(MessageType::Msg(Message::new(literal, name)))
        },
        Expr::InfixExpr(infix, exp1, exp2) => match evaluate_condition(infix, exp1, exp2, data) {
            Ok(val) => {
                let literal = val.literal;
                let name = literal.type_to_string();
                Ok(MessageType::Msg(Message::new(literal, name)))
            },
            Err(e) => Err(e),
        },
        Expr::IdentExpr(ident, ..) => match get_var(ident.to_owned(), data) {
            Ok(val) => {
                let literal = val.literal;
                let name = literal.type_to_string();
                Ok(MessageType::Msg(Message::new(literal, name)))
            },
            Err(_e) => {
                //TODO: change string null by literal NUll
                let literal = Literal::StringLiteral("NULL".to_owned());
                let name = literal.type_to_string();
                Ok(MessageType::Msg(Message::new(literal, name)))
            }
        },
        Expr::LitExpr { .. } => {
            let literal = expr_to_literal(action, data)?.literal;
            let name = literal.type_to_string();
            Ok(MessageType::Msg(Message::new(literal, name)))
        },
        Expr::VecExpr(..) => {
            let literal = expr_to_literal(action, data)?.literal;
            let name = literal.type_to_string();
            Ok(MessageType::Msg(Message::new(literal, name)))
        },
        e => Err(
            ErrorInfo{
                message: format!("Error must be a valid function {:?}", e),
                interval: interval_from_expr(e)
            }
        )
    }
}

fn match_actions(
    function: &ReservedFunction,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ReservedFunction::Say(arg) => Ok(add_to_message(root, match_functions(arg, data)?)),
        ReservedFunction::Use(arg) => {
            match_functions(arg, data)?;
            Ok(root)
        }
        ReservedFunction::Goto(GotoType::Step, step_name) => Ok(root.add_next_step(&step_name.ident)),
        ReservedFunction::Goto(GotoType::Flow, flow_name) => Ok(root.add_next_flow(&flow_name.ident)),
        ReservedFunction::Remember(name, variable) => {
            root = root.add_to_memory(name.ident.to_owned(), get_var_from_ident(variable, data)?.literal);
            Ok(root)
        }
        ReservedFunction::Import {
            step_name: name, ..
        } => {
            if let Some(Expr::Block { arg: actions, .. }) = data
                .ast
                .flow_instructions
                .get(&InstructionType::NormalStep(name.ident.to_owned()))
            {
                match interpret_block(&actions, data) {
                    Ok(root2) => Ok(root + root2),
                    Err(err) => Err(
                        ErrorInfo{
                            message: format!("Error in import function {:?}", err),
                            interval: interval_from_reserved_fn(function)
                        }
                    )
                }
            } else {
                Err(
                    ErrorInfo{
                        message: format!("Error step {} not found in flow", name.ident),
                        interval: interval_from_reserved_fn(function)
                    }
                )
            }
        }
        reserved => Err(
            ErrorInfo{
                message: "Error must be a valid action".to_owned(),
                interval: interval_from_reserved_fn(reserved)
            }
        )
    }
}

fn match_ask_response(
    vec: &[Expr],
    mut root: MessageData,
    data: &mut Data,
    opt: &Option<SmartIdent>,
    range: RangeInterval,
) -> Result<MessageData, ErrorInfo> {
    for block in vec.iter() {
        match (block, data.event) {
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                None,
            ) => return Ok(root + interpret_block(args, data)?),
            (
                Expr::Block {
                    block_type: BlockType::Response,
                    arg: args,
                    ..
                },
                Some(..),
            ) => {
                if let Some(SmartIdent { ident, interval }) = opt {
                    root = root.add_to_memory(
                        ident.to_owned(),
                        gen_literal_form_event(data.event, interval.to_owned())?.literal,
                    );
                }
                return Ok(root + interpret_block(args, data)?);
            }
            (_, _) => continue,
        }
    }
    Err(
        ErrorInfo{
            message: "Error sub block arg must be of type Expr::VecExpr".to_owned(),
            interval: range.start
        }
    )
}

pub fn solve_if_statments(
    statment: &IfStatement,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match statment {
        IfStatement::IfStmt {
            cond,
            consequence,
            then_branch,
        } => {
            if valid_condition(cond, data) {
                root = root + interpret_block(consequence, data)?;
                return Ok(root);
            }
            if let Some(then) = then_branch {
                return solve_if_statments(then, root, data);
            }
            Ok(root)
        }
        IfStatement::ElseStmt(consequence, ..) => {
            root = root + interpret_block(consequence, data)?;
            Ok(root)
        }
    }
}

pub fn interpret_block(actions: &[Expr], data: &mut Data) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData {
        memories: None,
        messages: vec![],
        next_flow: None,
        next_step: None,
    };

    for action in actions {
        if root.next_step.is_some() {
            return Ok(root);
        }

        match action {
            Expr::FunctionExpr(fun) => {
                root = match_actions(fun, root, data)?;
            }
            Expr::IfExpr(ref ifstatement) => root = solve_if_statments(ifstatement, root, data)?,
            Expr::Block {
                block_type: BlockType::AskResponse(opt),
                arg: vec,
                range
            } => {
                root = match_ask_response(vec, root, data, opt, range.clone())?;
            }
            e => return Err(
                ErrorInfo{
                    message: "Block must start with a reserved keyword".to_owned(),
                    interval: interval_from_expr(e)
                }
            )
        };
    }
    Ok(root)
}
