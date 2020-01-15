pub mod for_loop;
pub mod if_statment;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::{
        for_loop::for_loop,
        if_statment::{evaluate_condition, solve_if_statments},
    },
    builtins::{api_functions::*, reserved_functions::*},
    data::Data,
    message::*,
    variable_handler::{
        expr_to_literal::expr_to_literal,
        get_index, get_literal, get_string_from_complexstring, get_var, get_var_from_mem,
        interval::{interval_from_expr, interval_from_reserved_fn},
        memory::search_in_memory_type,
        object::update_value_in_object,
    },
};
use crate::parser::{ast::*, literal::Literal, tokens::*};
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
        TYPING => typing(args, name.to_owned(), interval),
        WAIT => wait(args, name.to_owned(), interval),
        URL => url(args, name.to_owned(), interval),
        IMAGE => img(args, name.to_owned(), interval),
        ONE_OF => one_of(args, interval),
        SHUFFLE => shuffle(args, interval),
        QUESTION => question(args, name.to_owned(), interval),

        LENGTH => length(args, interval),
        FIND => find(args, interval),
        RANDOM => random(&interval),
        FLOOR => floor(args, interval),
        VIDEO => video(args, name.to_owned(), interval),
        AUDIO => audio(args, name.to_owned(), interval),

        BUTTON => button(args, name.to_owned(), &interval),
        FN => api(args, interval, data),
        OBJECT => object(args, interval),
        _ => text(args, name.to_owned(), interval),
    }
}

pub fn match_functions(action: &Expr, data: &mut Data) -> Result<Literal, ErrorInfo> {
    match action {
        Expr::ObjectExpr(ObjectType::As(name, expr)) => {
            let lit = match_functions(expr, data)?;
            data.step_vars.insert(name.ident.to_owned(), lit.clone());
            Ok(lit)
        }
        Expr::ComplexLiteral(vec, RangeInterval{start, ..}) => Ok(get_string_from_complexstring(vec, start.to_owned(), data)),
        Expr::InfixExpr(infix, exp_1, exp_2) => Ok(evaluate_condition(infix, exp_1, exp_2, data)?),
        Expr::IdentExpr(ident) => match get_var(ident.to_owned(), data) {
            Ok(val) => Ok(val),
            Err(_e) => Ok(Literal::null(ident.interval.to_owned())),
        },
        Expr::ObjectExpr(ObjectType::Normal(..))
        | Expr::MapExpr(..)
        | Expr::BuilderExpr(..)
        | Expr::LitExpr { .. }
        | Expr::VecExpr(..) => Ok(expr_to_literal(action, data)?),
        e => Err(ErrorInfo {
            message: format!("invalid function {:?}", e),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn get_path(path: &[Expr], data: &mut Data) -> Result<Vec<Path>, ErrorInfo> {
    let mut vec = vec![];

    for node in path.iter() {
        match node {
            Expr::IdentExpr(Identifier{ident, index, ..}) => {
                match get_index(index.to_owned(), data)? {
                    Some(lit) => vec.push(Path::AtIndex(ident.to_owned(), lit.to_owned())),
                    None => vec.push(Path::Normal(ident.to_owned())),
                };
            },
            Expr::ObjectExpr(ObjectType::Normal(name, expr)) =>
                vec.push(Path::Exec(name.ident.to_owned(), expr_to_literal(expr, data)? ))
            ,
            err => return Err( ErrorInfo{
                message: format!("Bad Expression Type: 'in value.expression' expression need to be of type identifier"),
                interval: interval_from_expr(err),
            }),
        };
    }
    Ok(vec)
}

fn get_var_info<'a>(
    expr: &'a Expr,
    data: &'a mut Data,
) -> Result<
    (
        &'a mut Literal,
        String,
        String,
        Option<Vec<Path>>,
        Option<Literal>,
    ),
    ErrorInfo,
> {
    match expr {
        Expr::BuilderExpr(BuilderType::Normal(var), path) => {
            let index = get_index(var.index.clone(), data)?;
            let path = get_path(path, data)?;
            let (lit, name, mem_type) = get_var_from_mem(var.to_owned(), data)?;
            Ok((lit, name, mem_type, Some(path), index))
        }
        Expr::IdentExpr(var) => {
            let index = get_index(var.index.clone(), data)?;
            let (lit, name, mem_type) = match search_in_memory_type(var, data) {
                Ok(_) => get_var_from_mem(var.to_owned(), data)?,
                Err(_) => {
                    let lit = Literal::null(var.interval.to_owned());
                    data.step_vars.insert(var.ident.to_owned(), lit.clone());
                    get_var_from_mem(var.to_owned(), data)?
                }
            };
            Ok((lit, name, mem_type, None, index))
        }
        e => Err(ErrorInfo {
            message: format!("No methods available for this Literal type"),
            interval: interval_from_expr(e),
        }),
    }
}

fn update_literal(
    lit: &mut Literal,
    path: Option<Vec<Path>>,
    new_lit: Option<Literal>,
    interval: Interval,
) -> Result<(), ErrorInfo> {
    match path {
        None => {
            if let Some(new_lit) = new_lit {
                *lit = new_lit;
            }
            Ok(())
        }
        Some(path) => {
            update_value_in_object(lit, new_lit, &path, &interval)?;
            Ok(())
        }
    }
}

fn save_literal_in_mem(
    lit: Literal,
    name: String,
    mem_type: String,
    data: &mut Data,
    mut root: MessageData,
    sender: &Option<mpsc::Sender<MSG>>,
) -> MessageData {
    if mem_type == "remember" {
        // add mesage to rememeber new value
        root = root.add_to_memory(&name, lit.clone());
        // add value in current mem
        send_msg(sender, MSG::Memorie(Memories::new(name.clone(), lit.clone())));
        data.memory.current.insert(name, lit);
    } else {
        data.step_vars.insert(name, lit);
    }
    root
}

fn match_actions(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: usize,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match function {
<<<<<<< HEAD
        ObjectType::Say(arg) => {
            let msg = Message::new(match_functions(arg, data)?);
            send_msg(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(root, MessageType::Msg(msg)))
        },
=======
        ObjectType::Say(arg) => Ok(Message::add_to_message(
            root,
            MessageType::Msg(Message::new(match_functions(arg, data)?)),
        )),
>>>>>>> 6dc023c53f548a17fb18054162aa1767f5513b1c
        ObjectType::Use(arg) => {
            match_functions(arg, data)?;
            Ok(root)
        }
        ObjectType::Do(DoType::Update(old, new)) => {
            //TODO: make error if try to change _metadata
            let new_value = match_functions(new, data)?;
            let (lit, name, mem_type, path, index) = get_var_info(old, data)?;
            let inter = lit.get_interval();
            update_literal(get_literal(lit, index)?, path, Some(new_value), inter)?;
            Ok(save_literal_in_mem(
                lit.to_owned(),
                name,
                mem_type,
                data,
                root,
                sender
            ))
        }
        ObjectType::Do(DoType::Exec(expr)) => {
            if let Ok((lit, name, mem_type, path, index)) = get_var_info(expr, data) {
                let inter = lit.get_interval();
                update_literal(get_literal(lit, index)?, path, None, inter)?;
                return Ok(save_literal_in_mem(
                    lit.to_owned(),
                    name,
                    mem_type,
                    data,
                    root,
                    sender,
                ));
            }
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => {
<<<<<<< HEAD
            send_msg(&sender, MSG::NextStep(step_name.ident.clone()));
=======
>>>>>>> 6dc023c53f548a17fb18054162aa1767f5513b1c
            root.exit_condition = Some(ExitCondition::Goto);
            return Ok(root.add_next_step(&step_name.ident))
        }
        ObjectType::Goto(GotoType::Flow, flow_name) => {
<<<<<<< HEAD
            send_msg(&sender, MSG::NextFlow(flow_name.ident.clone()));
=======
>>>>>>> 6dc023c53f548a17fb18054162aa1767f5513b1c
            root.exit_condition = Some(ExitCondition::Goto);
            return Ok(root.add_next_flow(&flow_name.ident))
        }
        ObjectType::Remember(name, variable) => {
            let lit = match_functions(variable, data)?;
            root = root.add_to_memory(&name.ident, lit.clone());
<<<<<<< HEAD

            send_msg(
                &sender,
                MSG::Memorie(Memories::new(name.ident.to_owned(), lit.clone())),
            );

=======
>>>>>>> 6dc023c53f548a17fb18054162aa1767f5513b1c
            data.memory.current.insert(name.ident.to_owned(), lit);
            Ok(root)
        }
        ObjectType::Import {
            step_name: name, ..
        } => {
            if let Some(Expr::Scope {
                scope: actions,
                block_type: _,
                ..
            }) = data
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
    instruction_index: usize,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();

    for (action, instruction_info) in actions.commands.iter() {
        let instruction_total = instruction_info.index + instruction_info.total;
        if instruction_index > instruction_total {
            continue;
        }

        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }
        
        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(..)) => {
                send_msg(
                    &sender,
                    MSG::Hold {
                        instruction_index: instruction_info.index + 1,
                        step_vars: data.step_vars.clone(),
                    },
                );
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions(fun, root, data, instruction_index, &sender)?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statments(ifstatement, root, data, instruction_index, instruction_info, &sender)?;
            }
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop(ident, i, expr, block, range, root, data, instruction_index, &sender)?
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