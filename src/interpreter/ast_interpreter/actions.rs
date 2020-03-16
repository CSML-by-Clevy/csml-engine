use crate::data::primitive::null::PrimitiveNull;
use crate::data::{
    ast::*, message::*, msg::send_msg, Data, Literal, Memories, MemoryType, MessageData, MSG,
};
use crate::error_format::ErrorInfo;
use crate::interpreter::{
    interpret_scope,
    variable_handler::{
        exec_path_actions, expr_to_literal, get_var_from_mem, interval::*, memory::*,
    },
};
use crate::parser::ExitCondition;
use crate::data::literal::ContentType;
use std::sync::mpsc;

fn get_var_info<'a>(
    expr: &'a Expr,
    path: Option<&[(Interval, PathState)]>,
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
        Expr::PathExpr { literal, path } => get_var_info(literal, Some(path), data, root, sender),
        Expr::IdentExpr(var) => match search_in_memory_type(var, data) {
            Ok(_) => get_var_from_mem(var.to_owned(), path, data, root, sender),
            Err(_) => {
                let lit = PrimitiveNull::get_literal(var.interval.to_owned());
                data.step_vars.insert(var.ident.to_owned(), lit);
                get_var_from_mem(var.to_owned(), path, data, root, sender)
            }
        },
        e => Err(ErrorInfo {
            message: "expression need to be of type variable".to_owned(),
            interval: interval_from_expr(e),
        }),
    }
}

pub fn match_actions(
    function: &ObjectType,
    mut root: MessageData,
    data: &mut Data,
    instruction_index: Option<usize>,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            let msg = Message::new(expr_to_literal(arg, None, data, &mut root, sender)?);
            send_msg(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(root, MessageType::Msg(msg)))
        }
        ObjectType::Use(arg) => {
            expr_to_literal(arg, None, data, &mut root, sender)?;
            Ok(root)
        }
        ObjectType::Do(DoType::Update(old, new)) => {
            let new_value = expr_to_literal(new, None, data, &mut root, sender)?;
            let (lit, name, mem_type, path) = get_var_info(old, None, data, &mut root, sender)?;
            exec_path_actions(lit, Some(new_value), &path, &ContentType::get(&lit))?;
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
            expr_to_literal(expr, None, data, &mut root, sender)?;
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
            let lit = expr_to_literal(variable, None, data, &mut root, sender)?;
            root.add_to_memory(&name.ident, lit.clone());

            send_msg(
                &sender,
                MSG::Memorie(Memories::new(name.ident.to_owned(), lit.clone())),
            );

            data.context.current.insert(name.ident.to_owned(), lit);
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
