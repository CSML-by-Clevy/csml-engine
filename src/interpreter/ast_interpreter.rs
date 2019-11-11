pub mod for_loop;
pub mod if_statment;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::{
        // ask_response::match_ask_response,
        for_loop::for_loop,
        if_statment::{evaluate_condition, solve_if_statments},
    },
    builtins::{api_functions::*, reserved_functions::*},
    data::Data,
    message::*,
    variable_handler::{
        expr_to_literal::expr_to_literal,
        get_string_from_complexstring, get_var,
        interval::{interval_from_expr, interval_from_reserved_fn},
    },
};
use crate::parser::{ast::*, literal::Literal, tokens::*};
use std::collections::HashMap;

fn check_if_ident(expr: &Expr) -> bool {
    match expr {
        Expr::LitExpr { .. }
        | Expr::IdentExpr(..)
        | Expr::BuilderExpr(..)
        | Expr::ComplexLiteral(..)
        | Expr::ObjectExpr(..) => true,
        _ => false,
    }
}

pub fn match_builtin(
    name: &str,
    args: HashMap<String, Literal>,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    match name {
        TYPING => Ok(typing(args, name.to_owned(), interval)?),
        WAIT => Ok(wait(args, name.to_owned(), interval)?),
        URL => Ok(url(args, name.to_owned(), interval)?),
        IMAGE => Ok(img(args, name.to_owned(), interval)?),
        ONE_OF => Ok(one_of(args, interval)?),
        SHUFFLE => Ok(shuffle(args, interval)?),
        QUESTION => Ok(question(args, name.to_owned(), interval)?),

        LENGTH => Ok(length(args, interval)?),
        FIND => Ok(find(args, interval)?),
        RANDOM => Ok(random(&interval)?),
        FLOOR => Ok(floor(args, interval)?),
        VIDEO => Ok(video(args, name.to_owned(), interval)?),
        AUDIO => Ok(audio(args, name.to_owned(), interval)?),

        BUTTON => Ok(button(args, name.to_owned(), &interval)?),
        FN => Ok(api(args, interval, data)?),
        OBJECT => Ok(object(args, interval)?),
        _ => Ok(text(args, name.to_owned(), interval)?),
    }
}

pub fn match_functions(action: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match action {
        Expr::ObjectExpr(ObjectType::As(name, expr)) => {
            let lit = match_functions(expr, data)?;
            data.step_vars.insert(name.ident.to_owned(), lit.clone());
            Ok(lit)
        }
        Expr::ComplexLiteral(vec, ..) => Ok(get_string_from_complexstring(vec, data)),
        Expr::InfixExpr(infix, exp_1, exp_2) => Ok(evaluate_condition(infix, exp_1, exp_2, data)?),
        Expr::IdentExpr(ident) => match get_var(ident.to_owned(), data) {
            Ok(val) => Ok(val),
            Err(_e) => Ok(Literal::null(ident.interval.to_owned())),
        },
        Expr::ObjectExpr(ObjectType::Normal(..))
        | Expr::BuilderExpr(..)
        | Expr::LitExpr { .. }
        | Expr::VecExpr(..) => Ok(expr_to_literal(action, data)?),
        e => Err(ErrorInfo {
            message: format!("Error must be a valid function {:?}", e),
            interval: interval_from_expr(e),
        }),
    }
}

fn match_actions(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => Ok(Message::add_to_message(
            root,
            MessageType::Msg(Message::new(match_functions(arg, data)?)),
        )),
        ObjectType::Use(arg) => {
            match_functions(arg, data)?;
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => Ok(root.add_next_step(&step_name.ident)),
        ObjectType::Goto(GotoType::Flow, flow_name) => Ok(root.add_next_flow(&flow_name.ident)),
        ObjectType::Remember(name, variable) => {
            let lit = match_functions(variable, data)?;
            root = root.add_to_memory(&name.ident, lit.clone());
            data.step_vars.insert(name.ident.to_owned(), lit); // can be remove if we check if tmp var are saved in memory
            Ok(root)
        }
        ObjectType::Import {
            step_name: name, ..
        } => {
            if let Some(Expr::Scope { scope: actions, block_type, .. }) = data
                .ast
                .flow_instructions
                .get(&InstructionType::NormalStep(name.ident.to_owned()))
            {
                match interpret_scope(block_type, actions, data) {
                    //, &range.start
                    Ok(root2) => Ok(root + root2),
                    Err(err) => Err(ErrorInfo {
                        message: format!("Error in import function {:?}", err),
                        interval: interval_from_reserved_fn(function),
                    }),
                }
            } else {
                Err(ErrorInfo {
                    message: format!("Error step {} not found in flow", name.ident),
                    interval: interval_from_reserved_fn(function),
                })
            }
        }
        reserved => Err(ErrorInfo {
            message: "Error must be a valid action".to_owned(),
            interval: interval_from_reserved_fn(reserved),
        }),
    }
}

pub fn interpret_scope(block_type: &BlockType, actions: &Block, data: &mut Data) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();

    for (i, action) in actions.commands.iter().enumerate() {
        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Hold(interval)) => {
                match block_type {
                    BlockType::Step => {},
                    _ => return Err(ErrorInfo {
                        message: "no hold allowed in if/foreach blocks".to_owned(),
                        interval: interval.to_owned(),
                    })
                };
                root.index = (i + 1) as i64;
                return Ok(root)
            },
            Expr::ObjectExpr(fun) => root = match_actions(fun, root, data)?,
            Expr::IfExpr(ref ifstatement) => root = solve_if_statments(ifstatement, root, data)?,
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop(ident, i, expr, block, range, root, data)?
            }
            e => {
                return Err(ErrorInfo {
                    message: "Block must start with a reserved keyword".to_owned(),
                    interval: interval_from_expr(e),
                })
            }
        };
    }
    Ok(root)
}

pub fn interpret_scope_at_index(actions: &Block, data: &mut Data, index: i64) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();
    let vec = actions.commands.clone().split_off(index as usize);

    for (i, action) in vec.iter().enumerate() {
        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Hold(_)) => {
                root.index = (i as i64) + index + 1;
                return Ok(root)
            },
            Expr::ObjectExpr(fun) => root = match_actions(fun, root, data)?,
            Expr::IfExpr(ref ifstatement) => root = solve_if_statments(ifstatement, root, data)?,
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop(ident, i, expr, block, range, root, data)?
            }
            e => {
                return Err(ErrorInfo {
                    message: "Block must start with a reserved keyword".to_owned(),
                    interval: interval_from_expr(e),
                })
            }
        };
    }
    Ok(root)
}