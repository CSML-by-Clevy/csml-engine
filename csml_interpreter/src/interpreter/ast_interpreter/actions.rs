use crate::data::{
    ast::*, literal::ContentType, message::*, primitive::null::PrimitiveNull, Data, Literal,
    Memory, MemoryType, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::variable_handler::{
    exec_path_actions, expr_to_literal, get_var_from_mem, interval::*, memory::*, resolve_fn_args,
    search_goto_var_memory
};
use crate::parser::ExitCondition;
use crate::data::position::Position;
use std::sync::mpsc;

fn get_var_info<'a>(
    expr: &'a Expr,
    path: Option<&[(Interval, PathState)]>,
    data: &'a mut Data,
    msg_data: &mut MessageData,
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
        Expr::PathExpr { literal, path } => {
            get_var_info(literal, Some(path), data, msg_data, sender)
        }
        Expr::IdentExpr(var) => match search_in_memory_type(var, data) {
            Ok(_) => get_var_from_mem(var.to_owned(), false, path, data, msg_data, sender),
            Err(_) => {
                let lit = PrimitiveNull::get_literal(var.interval.to_owned());
                data.step_vars.insert(var.ident.to_owned(), lit);
                get_var_from_mem(var.to_owned(), false, path, data, msg_data, sender)
            }
        },
        e => Err(gen_error_info(
            Position::new(interval_from_expr(e)),
            ERROR_GET_VAR_INFO.to_owned(),
        )),
    }
}

pub fn match_actions(
    function: &ObjectType,
    mut msg_data: MessageData,
    data: &mut Data,
    sender: &Option<mpsc::Sender<MSG>>,
) -> Result<MessageData, ErrorInfo> {
    match function {
        ObjectType::Say(arg) => {
            let msg = Message::new(expr_to_literal(
                arg,
                false,
                None,
                data,
                &mut msg_data,
                sender,
            )?)?;
            MSG::send(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(msg_data, MessageType::Msg(msg)))
        }
        ObjectType::Debug(args, interval) => {
            let args = resolve_fn_args(args, data, &mut msg_data, sender)?;

            let msg = Message::new(args.args_to_debug(interval.to_owned()))?;
            MSG::send(&sender, MSG::Message(msg.clone()));
            Ok(Message::add_to_message(msg_data, MessageType::Msg(msg)))
        }
        ObjectType::Use(arg) => {
            expr_to_literal(arg, false, None, data, &mut msg_data, sender)?;
            Ok(msg_data)
        }
        ObjectType::Do(DoType::Update(old, new)) => {
            let new_value = expr_to_literal(new, false, None, data, &mut msg_data, sender)?;
            let (lit, name, mem_type, path) = get_var_info(old, None, data, &mut msg_data, sender)?;
            exec_path_actions(
                lit,
                false,
                Some(new_value),
                &path,
                &ContentType::get(&lit),
                &mut msg_data,
                sender,
            )?;
            save_literal_in_mem(
                lit.to_owned(),
                name,
                &mem_type,
                true,
                data,
                &mut msg_data,
                sender,
            );

            Ok(msg_data)
        }
        ObjectType::Do(DoType::Exec(expr)) => {
            expr_to_literal(expr, false, None, data, &mut msg_data, sender)?;
            Ok(msg_data)
        }
        ObjectType::Goto(GotoType::Step(step), ..) => {
            let step = search_goto_var_memory(step, &mut msg_data, data)?;

            MSG::send(
                &sender,
                MSG::Next {
                    flow: None,
                    step: Some(step.to_string()),
                },
            );

            data.context.step = step.to_string();
            msg_data.exit_condition = Some(ExitCondition::Goto);

            if step == "end" {
                msg_data.exit_condition = Some(ExitCondition::End);
            }

            Ok(msg_data)
        }
        ObjectType::Goto(GotoType::Flow(flow), ..) => {
            let flow = search_goto_var_memory(&flow, &mut msg_data, data)?;

            MSG::send(
                &sender,
                MSG::Next {
                    flow: Some(flow.to_string()),
                    step: None,
                },
            );

            data.context.step = "start".to_string();
            data.context.flow = flow.to_string();
            msg_data.exit_condition = Some(ExitCondition::Goto);

            Ok(msg_data)
        }
        ObjectType::Goto(GotoType::StepFlow { step, flow }, ..) => {
            let step = search_goto_var_memory(&step, &mut msg_data, data)?;
            let mut flow_opt = Some(search_goto_var_memory(&flow, &mut msg_data, data)?);

            msg_data.exit_condition = Some(ExitCondition::Goto);

            if step == "end" {
                msg_data.exit_condition = Some(ExitCondition::End);
                flow_opt = None;
            }

            MSG::send(
                &sender,
                MSG::Next {
                    flow: flow_opt,
                    step: Some(step.to_string()),
                },
            );

            data.context.step = step.to_string();
            data.context.flow = flow.to_string();

            Ok(msg_data)
        }
        ObjectType::Remember(name, variable) => {
            let lit = expr_to_literal(variable, false, None, data, &mut msg_data, sender)?;
            msg_data.add_to_memory(&name.ident, lit.clone());

            MSG::send(
                &sender,
                MSG::Memory(Memory::new(name.ident.to_owned(), lit.clone())),
            );

            data.context.current.insert(name.ident.to_owned(), lit);
            Ok(msg_data)
        }
        reserved => Err(gen_error_info(
            Position::new(interval_from_reserved_fn(reserved)),
            ERROR_START_INSTRUCTIONS.to_owned(),
        )),
    }
}
