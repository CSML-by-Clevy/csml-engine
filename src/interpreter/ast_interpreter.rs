pub mod for_loop;
pub mod if_statment;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::{
        for_loop::for_loop,
        if_statment::{evaluate_condition, solve_if_statments},
    },
    builtins::{api_functions::*, reserved_functions::*},
    data::{Data, MemoryType},
    message::*,
    variable_handler::{
        exec_path_actions,
        expr_to_literal::expr_to_literal,
        get_string_from_complexstring, get_var, get_var_from_mem,
        interval::{interval_from_expr, interval_from_reserved_fn},
        memory::{save_literal_in_mem, search_in_memory_type},
    },
};
use crate::parser::{ast::*, literal::Literal, tokens::*};
use crate::primitive::null::PrimitiveNull;
use std::collections::HashMap;
use std::sync::mpsc;

pub fn send_msg(sender: &Option<mpsc::Sender<MSG>>, msg: MSG) {
    if let Some(sender) = sender {
        sender.send(msg).unwrap();
    }
}

fn check_if_ident(expr: &Expr) -> bool {
    match expr {
        Expr::LitExpr { .. }
        | Expr::IdentExpr(..)
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
        // CUSTOM
        TYPING => typing(args, name.to_owned(), interval),
        WAIT => wait(args, name.to_owned(), interval),
        URL => url(args, name.to_owned(), interval),
        IMAGE => img(args, name.to_owned(), interval),
        QUESTION => question(args, name.to_owned(), interval),
        VIDEO => video(args, name.to_owned(), interval),
        AUDIO => audio(args, name.to_owned(), interval),
        BUTTON => button(args, name.to_owned(), interval),
        OBJECT => object(args, interval),

        // DEFAULT
        FN => api(args, interval, data),
        ONE_OF => one_of(args, interval),
        SHUFFLE => shuffle(args, interval),
        LENGTH => length(args, interval),
        FIND => find(args, interval),
        RANDOM => random(interval),
        FLOOR => floor(args, interval),
        _ => text(args, name.to_owned(), interval),
    }
}

pub fn match_functions(
    action: &Expr,
    data: &mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<Literal, ErrorInfo> {
    match action {
        Expr::ObjectExpr(ObjectType::As(name, expr)) => {
            let lit = match_functions(expr, data, root, sender)?;
            data.step_vars.insert(name.ident.to_owned(), lit.clone());
            Ok(lit)
        }
        Expr::ComplexLiteral(vec, RangeInterval { start, .. }) => Ok(
            get_string_from_complexstring(vec, start.to_owned(), data, root, sender),
        ),
        Expr::InfixExpr(infix, exp_1, exp_2) => {
            Ok(evaluate_condition(infix, exp_1, exp_2, data, root, sender)?)
        }
        Expr::IdentExpr(ident) => match get_var(ident.to_owned(), data, root, sender) {
            Ok(val) => Ok(val),
            Err(e) => Err(e),
        },
        Expr::ObjectExpr(ObjectType::Normal(..))
        | Expr::MapExpr(..)
        | Expr::LitExpr { .. }
        | Expr::VecExpr(..) => Ok(expr_to_literal(action, data, root, sender)?),
        e => Err(ErrorInfo {
            message: format!("invalid function {:?}", e),
            interval: interval_from_expr(e),
        }),
    }
}

fn get_var_info<'a>(
    expr: &'a Expr,
    data: &'a mut Data,
    root: &mut MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<
    (
        &'a mut Literal,
        String,
        MemoryType,
        Option<Vec<(Interval, PathLiteral)>>,
    ),
    ErrorInfo,
> {
    match expr {
        Expr::IdentExpr(var) => match search_in_memory_type(var, data) {
            Ok(_) => get_var_from_mem(var.to_owned(), data, root, sender),
            Err(_) => {
                let lit = PrimitiveNull::get_literal("null", var.interval.to_owned());
                data.step_vars.insert(var.ident.to_owned(), lit);
                get_var_from_mem(var.to_owned(), data, root, sender)
            }
        },
        e => Err(ErrorInfo {
            message: "expression need to be of type variable".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

fn match_actions(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            let msg = Message::new(match_functions(arg, data, &mut root, sender)?);
            send_msg(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(root, MessageType::Msg(msg)))
        }
        ObjectType::Use(arg) => {
            match_functions(arg, data, &mut root, sender)?;
            Ok(root)
        }
        ObjectType::Do(DoType::Update(old, new)) => {
            let new_value = match_functions(new, data, &mut root, sender)?;
            let (lit, name, mem_type, path) = get_var_info(old, data, &mut root, sender)?;
            exec_path_actions(lit, Some(new_value), &path, &mem_type)?;
            save_literal_in_mem(
                lit.to_owned(),
                name,
                &mem_type,
                true,
                data,
                &mut root,
                sender,
            );

            Ok(root)
        }
        ObjectType::Do(DoType::Exec(expr)) => {
            match_functions(expr, data, &mut root, sender)?;
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => {
            send_msg(&sender, MSG::NextStep(step_name.ident.clone()));
            root.exit_condition = Some(ExitCondition::Goto);
            Ok(root.add_next_step(&step_name.ident))
        }
        ObjectType::Goto(GotoType::Flow, flow_name) => {
            send_msg(&sender, MSG::NextFlow(flow_name.ident.clone()));
            root.exit_condition = Some(ExitCondition::Goto);
            Ok(root.add_next_flow(&flow_name.ident))
        }
        ObjectType::Remember(name, variable) => {
            let lit = match_functions(variable, data, &mut root, sender)?;
            root.add_to_memory(&name.ident, lit.clone());

            send_msg(
                &sender,
                MSG::Memorie(Memories::new(name.ident.to_owned(), lit.clone())),
            );

            data.memory.current.insert(name.ident.to_owned(), lit);
            Ok(root)
        }
        ObjectType::Import {
            step_name: name, ..
        } => {
            if let Some(Expr::Scope { scope: actions, .. }) = data
                .ast
                .flow_instructions
                .get(&InstructionType::NormalStep(name.ident.to_owned()))
            {
                match interpret_scope(actions, data, instruction_index, sender) {
                    //, &range.start
                    Ok(root2) => Ok(root + root2),
                    Err(err) => Err(ErrorInfo {
                        message: format!("invalid import function {:?}", err),
                        interval: interval_from_reserved_fn(function),
                    }),
                }
            } else {
                Err(ErrorInfo {
                    message: format!("invalid step {} not found in flow", name.ident),
                    interval: interval_from_reserved_fn(function),
                })
            }
        }
        reserved => Err(ErrorInfo {
            message: "invalid action".to_owned(),
            interval: interval_from_reserved_fn(reserved),
        }),
    }
}

pub fn interpret_scope(
    actions: &Block,
    data: &mut Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();

    for (action, instruction_info) in actions.commands.iter() {
        let instruction_total = instruction_info.index + instruction_info.total;
        if let Some(instruction_index) = instruction_index {
            if instruction_index >= instruction_total {
                continue;
            }
        }

        if root.exit_condition.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(..)) => {
                root.exit_condition = Some(ExitCondition::Hold);
                send_msg(
                    &sender,
                    MSG::Hold {
                        instruction_index: instruction_info.index,
                        step_vars: step_vars_to_json(data.step_vars.clone()),
                    },
                );
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions(fun, root, data, instruction_index, &sender)?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statments(
                    ifstatement,
                    root,
                    data,
                    instruction_index,
                    instruction_info,
                    &sender,
                )?;
            }
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop(
                    ident,
                    i,
                    expr,
                    block,
                    range,
                    root,
                    data,
                    instruction_index,
                    &sender,
                )?
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
