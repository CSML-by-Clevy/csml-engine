use std::collections::HashMap;
use crate::parser::{ast::*, tokens::*};
use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    builtins::{api_functions::*, reserved_functions::*},
    data::Data,
    message::*,
    variable_handler::*,
};

fn match_obj(lit1: &SmartLiteral, lit2: &SmartLiteral) -> Result<Literal, ErrorInfo> {
    let _b = BUTTON.to_owned();
    if let Literal::ObjectLiteral{properties, .. } = lit2.literal.clone() {
        match Literal::search_in_obj(&properties, "accept") {
            Some(Literal::ArrayLiteral{items, ..}) => {
                return Ok(Literal::boolean(items.contains(&lit1.literal)))
            }
            Some(val) => {
                return Ok(Literal::boolean(val == &lit1.literal))
            },
            _ => return Ok(Literal::boolean(lit1.literal == lit2.literal)),
        }
    }
    Ok(Literal::boolean(lit1 == lit2))
}

fn cmp_lit(
    infix: &Infix,
    lit1: Result<SmartLiteral, ErrorInfo>,
    lit2: Result<SmartLiteral, ErrorInfo>,
) -> Result<SmartLiteral, ErrorInfo> {
    match (infix, lit1, lit2) {
        (Infix::NotEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 != l2), interval: l1.interval.to_owned()}),
        (Infix::Equal, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 == l2), interval: l1.interval.to_owned()}),
        (Infix::GreaterThanEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 >= l2), interval: l1.interval.to_owned()}),
        (Infix::LessThanEqual, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 <= l2), interval: l1.interval.to_owned()}),
        (Infix::GreaterThan, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 > l2), interval: l1.interval.to_owned()}),
        (Infix::LessThan, Ok(l1), Ok(l2)) => Ok(SmartLiteral{literal: Literal::boolean(l1 < l2), interval: l1.interval.to_owned()}),
        (Infix::Or, Ok(l1), Ok(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: l1.interval.to_owned()}),
        (Infix::Or, Ok(l1), Err(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: l1.interval.to_owned()}),
        (Infix::Or, Err(e), Ok(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: e.interval.to_owned()}),
        (Infix::And, Ok(l1), Ok(..)) => Ok(SmartLiteral{literal: Literal::boolean(true), interval: l1.interval.to_owned()}),
        (Infix::Adition, Ok(l1), Ok(l2)) => l1 + l2,
        (Infix::Substraction, Ok(l1), Ok(l2)) => l1 - l2,
        (Infix::Divide, Ok(l1), Ok(l2)) => l1 / l2,
        (Infix::Multiply, Ok(l1), Ok(l2)) => l1 * l2,
        (Infix::Match, Ok(ref l1), Ok(ref l2)) => Ok(SmartLiteral{literal: match_obj(l1, l2)?, interval: l1.interval.to_owned()}),
        (_, Ok(l1), ..) => Ok(SmartLiteral{literal: Literal::boolean(false), interval: l1.interval.to_owned()}),
        (_, Err(e), ..) => Ok(SmartLiteral{literal: Literal::boolean(false), interval: e.interval.to_owned()}),
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

pub fn evaluate_condition(
    infix: &Infix,
    expr1: &Expr,
    expr2: &Expr,
    data: &mut Data,
) -> Result<SmartLiteral, ErrorInfo> {
    match (expr1, expr2) {
        (exp1, ..) if Infix::Not == *infix && check_if_ident(exp1) => {
            match get_var_from_ident(exp1, data) {
                Ok(SmartLiteral {
                    literal: Literal::BoolLiteral{value: false, ..},
                    interval,
                }) => Ok(SmartLiteral {
                    literal: Literal::boolean(true),
                    interval,
                }),
                Ok(SmartLiteral {
                    literal: Literal::IntLiteral{value: 0, ..},
                    interval,
                }) => Ok(SmartLiteral {
                    literal: Literal::boolean(true),
                    interval,
                }),
                Ok(SmartLiteral { interval, .. }) => Ok(SmartLiteral {
                    literal: Literal::boolean(false),
                    interval,
                }),
                Err(err) => Ok(SmartLiteral {
                    literal: Literal::boolean(true),
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
            Ok(SmartLiteral{literal: Literal::BoolLiteral{value: false, ..}, ..}) => false,
            Ok(_) => true,
            Err(_e) => false,
        },
        Expr::LitExpr { .. } => true,
        Expr::BuilderExpr(..) => get_var_from_ident(expr, data).is_ok(), // error
        Expr::IdentExpr(ident, ..) => get_var(ident.to_owned(), data).is_ok(),      // error
        _ => false, // return error
    }
}

fn add_to_message(root: MessageData, action: MessageType) -> MessageData {
    match action {
        MessageType::Msg(msg) => root.add_message(msg),
        MessageType::Empty => root,
    }
}

fn match_builtin(name: &str, args: HashMap<String, Literal>, span: Interval, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match name {
        TYPING => Ok(typing(args, name.to_owned(), span)?),
        WAIT => Ok(wait(args, name.to_owned(), span)?),
        URL => Ok(url(args, name.to_owned(), span)?),
        IMAGE => Ok(img(args, name.to_owned(), span)?),
        ONE_OF => Ok(one_of(args, span)?),
        QUESTION => Ok(question(args, name.to_owned(), span)?),
        BUTTON => Ok(button(args, name.to_owned(), &span)?),
        FN => Ok(api(args, span, data)?),
        _ => Ok(text(args, name.to_owned(), span)?)
    }
}

fn expr_to_literal(expr: &Expr, data: &mut Data) -> Result<SmartLiteral, ErrorInfo> {
    match expr {
        Expr::ObjectExpr(ObjectType::As(name, var)) => {
            let value = expr_to_literal(var, data)?;
            data.step_vars.insert(name.ident.to_owned(), value.literal.clone());
            Ok(value)
        }
        Expr::ObjectExpr(ObjectType::Normal(name, var)) => {
            let mut obj: HashMap<String, Literal> = HashMap::new();
            let expr: &Expr = var;

            if let Expr::VecExpr(vec, _) = expr {
                for elem in vec.iter() {
                    match elem {
                        Expr::ObjectExpr(ObjectType::Assign(var_name, var)) => {
                            let value = expr_to_literal(var, data)?.literal;
                            obj.insert(var_name.ident.to_owned(), value);
                        }
                        _ => {
                            let value = expr_to_literal(elem, data)?.literal;
                            obj.insert("default".to_owned(), value);
                        }
                    }
                }
            };
            if BUILT_IN.contains(&&*name.ident) {
                Ok(
                    SmartLiteral {
                        literal: match_builtin(&name.ident, obj, interval_from_expr(expr), data)?,
                        interval: interval_from_expr(expr)
                    }
                )
            } else {
                let mut test: HashMap<String, Literal> = HashMap::new();
                test.insert(name.ident.to_owned(), Literal::object(obj));
                Ok(
                    SmartLiteral {
                        literal: Literal::object(test),
                        interval: interval_from_expr(expr)
                    }
                )
            }
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::VecExpr(vec, range) => {
            let mut array = vec![];
            for value in vec.iter() {
                array.push(expr_to_literal(value, data)?.literal)
            }
            Ok(SmartLiteral{
                    literal: Literal::array(array),
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
        Expr::ObjectExpr(ObjectType::As(name, expr)) => {
            let msg = match_functions(expr, data)?;

            match msg {
                MessageType::Msg(Message { ref content, .. }) => {
                    data.step_vars.insert(name.ident.to_owned(), content.clone());
                }
                MessageType::Empty => {}
            };
            Ok(msg)
        }
        Expr::ObjectExpr(ObjectType::Normal(..)) => {
            let literal = expr_to_literal(action, data)?.literal;
            Ok(MessageType::Msg(Message::new(literal)))
        },
        Expr::BuilderExpr(..) => match get_var_from_ident(action, data) {
            Ok(val) => {
                let literal = val.literal;
                Ok(MessageType::Msg(Message::new(literal)))
            },
            Err(e) => Err(e),
        },
        Expr::ComplexLiteral(vec, ..) => {
            let literal = get_string_from_complexstring(vec, data).literal;
            Ok(MessageType::Msg(Message::new(literal)))
        },
        Expr::InfixExpr(infix, exp1, exp2) => match evaluate_condition(infix, exp1, exp2, data) {
            Ok(val) => {
                let literal = val.literal;
                Ok(MessageType::Msg(Message::new(literal)))
            },
            Err(e) => Err(e),
        },
        Expr::IdentExpr(ident, ..) => match get_var(ident.to_owned(), data) {
            Ok(val) => {
                let literal = val.literal;
                Ok(MessageType::Msg(Message::new(literal)))
            },
            Err(_e) => {
                Ok(MessageType::Msg(Message::new(Literal::null())))
            }
        },
        Expr::LitExpr { .. } => {
            let literal = expr_to_literal(action, data)?.literal;
            Ok(MessageType::Msg(Message::new(literal)))
        },
        Expr::VecExpr(..) => {
            let literal = expr_to_literal(action, data)?.literal;
            Ok(MessageType::Msg(Message::new(literal)))
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
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            Ok(add_to_message(root, match_functions(arg, data)?))
        },
        ObjectType::Use(arg) => {
            match_functions(arg, data)?;
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => Ok(root.add_next_step(&step_name.ident)),
        ObjectType::Goto(GotoType::Flow, flow_name) => Ok(root.add_next_flow(&flow_name.ident)),
        ObjectType::Remember(name, variable) => {
            root = root.add_to_memory(name.ident.to_owned(), expr_to_literal(variable, data)?.literal);
            Ok(root)
        }
        ObjectType::Import {
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
        match (block, data.event, data.memory.is_initial_step) {
            (
                Expr::Block {
                    block_type: BlockType::Response,
                    arg: args,
                    ..
                },
                Some(..),
                false
            ) => {
                if let Some(SmartIdent { ident, interval }) = opt {
                    root = root.add_to_memory(
                        ident.to_owned(),
                        gen_literal_form_event(data.event, interval.to_owned())?.literal,
                    );
                }
                return Ok(root + interpret_block(args, data)?);
            },
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                None,
                false
            ) => return Ok(root + interpret_block(args, data)?),
            (
                Expr::Block {
                    block_type: BlockType::Ask,
                    arg: args,
                    ..
                },
                Some(..),
                true
            ) => return Ok(root + interpret_block(args, data)?),
            (..) => continue,
        }
    }
    Err(
        ErrorInfo{
            message: "Error fail to find the correct action block bettween Ask/Response".to_owned(),
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
            Expr::ObjectExpr(fun) => {
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
