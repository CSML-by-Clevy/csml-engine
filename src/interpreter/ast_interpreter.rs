pub mod for_loop;
pub mod if_statment;

use crate::error_format::data::ErrorInfo;
use crate::interpreter::{
    ast_interpreter::{
        for_loop::{for_loop, for_loop_mpsc},
        if_statment::{evaluate_condition, solve_if_statments, solve_if_statments_mpsc},
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
            message: format!("Error must be a valid function {:?}", e),
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
) -> MessageData {
    if mem_type == "remember" {
        // add mesage to rememeber new value
        root = root.add_to_memory(&name, lit.clone());
        // add value in current mem
        // TODO: update existing value
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
        ObjectType::Do(DoType::Update(old, new)) => {
            //TODO: make error if try to change _metadata
            println!("=> {:?}", old);
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
                ));
            }
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => {
            root.exit_condition = Some(ExitCondition::Goto);
            return Ok(root.add_next_step(&step_name.ident))
        }
        ObjectType::Goto(GotoType::Flow, flow_name) => {
            root.exit_condition = Some(ExitCondition::Goto);
            return Ok(root.add_next_flow(&flow_name.ident))
        }
        ObjectType::Remember(name, variable) => {
            let lit = match_functions(variable, data)?;
            root = root.add_to_memory(&name.ident, lit.clone());
            data.memory.current.insert(name.ident.to_owned(), lit);
            Ok(root)
        }
        ObjectType::Import {
            step_name: name, ..
        } => {
            if let Some(Expr::Scope {
                scope: actions,
                block_type,
                ..
            }) = data
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

pub fn interpret_scope(
    block_type: &BlockType,
    actions: &Block,
    data: &mut Data,
) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();

    for (i, action) in actions.commands.iter().enumerate() {
        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }
        
        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(interval)) => {
                match block_type {
                    BlockType::Step => {}
                    _ => {
                        return Err(ErrorInfo {
                            message: "no hold allowed in if blocks".to_owned(),
                            interval: interval.to_owned(),
                        })
                    }
                };
                root.index = (i + 1) as i64;
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions(fun, root, data)?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statments(ifstatement, root, data)?
            }
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

pub fn interpret_scope_at_index(
    actions: &Block,
    data: &mut Data,
    index: i64,
) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();
    let vec = actions.commands.clone().split_off(index as usize);

    for (i, action) in vec.iter().enumerate() {
        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(_)) => {
                root.index = (i as i64) + index + 1;
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions(fun, root, data)?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statments(ifstatement, root, data)?
            }
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

///////////////////////////// TODO: tmp mpsc
/////////////////////////////
/////////////////////////////
/////////////////////////////

fn send_msg(sender: &mpsc::Sender<MSG>, msg: MSG) {
    sender.send(msg).unwrap();
}

fn match_actions_mpsc(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
    sender: mpsc::Sender<MSG>,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            let msg = Message::new(match_functions(arg, data)?);
            send_msg(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(root, MessageType::Msg(msg)))
        }
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
                ));
            }
            Ok(root)
        }
        ObjectType::Goto(GotoType::Step, step_name) => {
            send_msg(&sender, MSG::NextStep(step_name.ident.clone()));
            root.exit_condition = Some(ExitCondition::Goto);
            return Ok(root.add_next_step(&step_name.ident))
        }
        ObjectType::Goto(GotoType::Flow, flow_name) => {
            send_msg(&sender, MSG::NextFlow(flow_name.ident.clone()));
            root.exit_condition = Some(ExitCondition::Goto);
            return Ok(root.add_next_flow(&flow_name.ident))
        }
        ObjectType::Remember(name, variable) => {
            let lit = match_functions(variable, data)?;
            root = root.add_to_memory(&name.ident, lit.clone());

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
            if let Some(Expr::Scope {
                scope: actions,
                block_type,
                ..
            }) = data
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

pub fn interpret_scope_mpsc(
    block_type: &BlockType,
    actions: &Block,
    data: &mut Data,
    sender: mpsc::Sender<MSG>,
) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();

    for (i, action) in actions.commands.iter().enumerate() {
        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(interval)) => {
                match block_type {
                    BlockType::Step => {}
                    _ => {
                        return Err(ErrorInfo {
                            message: "no hold allowed in if blocks".to_owned(),
                            interval: interval.to_owned(),
                        })
                    }
                };

                root.index = (i + 1) as i64;
                send_msg(
                    &sender,
                    MSG::Hold {
                        index: root.index,
                        step_vars: data.step_vars.clone(),
                    },
                );
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions_mpsc(fun, root, data, mpsc::Sender::clone(&sender))?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statments_mpsc(ifstatement, root, data, sender.clone())?
            }
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop_mpsc(ident, i, expr, block, range, root, data, sender.clone())?
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

pub fn interpret_scope_at_index_mpsc(
    actions: &Block,
    data: &mut Data,
    index: i64,
    sender: mpsc::Sender<MSG>,
) -> Result<MessageData, ErrorInfo> {
    let mut root = MessageData::default();
    let vec = actions.commands.clone().split_off(index as usize);

    for (i, action) in vec.iter().enumerate() {
        if root.next_step.is_some() || root.next_flow.is_some() {
            return Ok(root);
        }

        match action {
            Expr::ObjectExpr(ObjectType::Break(..)) => {
                root.exit_condition = Some(ExitCondition::Break);
                return Ok(root);
            }
            Expr::ObjectExpr(ObjectType::Hold(..)) => {
                root.index = (i as i64) + index + 1;
                send_msg(
                    &sender,
                    MSG::Hold {
                        index: root.index,
                        step_vars: data.step_vars.clone(),
                    },
                );
                return Ok(root);
            }
            Expr::ObjectExpr(fun) => {
                root = match_actions_mpsc(fun, root, data, mpsc::Sender::clone(&sender))?
            }
            Expr::IfExpr(ref ifstatement) => {
                root = solve_if_statments_mpsc(ifstatement, root, data, sender.clone())?
            }
            Expr::ForEachExpr(ident, i, expr, block, range) => {
                root = for_loop_mpsc(ident, i, expr, block, range, root, data, sender.clone())?
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
