use crate::data::position::Position;
use crate::data::{
    data::{Data, init_child_context},
    ast::*, literal::ContentType, message::*, primitive::{PrimitiveNull, closure::capture_variables}, Literal,
    Memory, MemoryType, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::variable_handler::{
    exec_path_actions, expr_to_literal, get_var_from_mem, interval::*, memory::*, resolve_fn_args,
    search_goto_var_memory,
};
use crate::parser::ExitCondition;
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
            // ######################
            let mut context_tmp = init_child_context(&data);
            let flows_tmp = data.flows.clone();
            let flow_tmp = data.flow.clone();
            let event_tmp = data.event.clone();
            let env_tmp = data.env.clone();
            let loop_indexs_tmp = data.loop_indexs.clone();
            let loop_index_tmp = data.loop_index.clone();
            let step_vars_tmp = data.step_vars.clone();
            let custom_component_tmp = data.custom_component.clone();
            let native_component_tmp = data.native_component.clone();
            let mut new_scope_data = Data {
                flows: &flows_tmp,
                flow: &flow_tmp,
                context: &mut context_tmp,
                event: &event_tmp,
                env: &env_tmp,
                loop_indexs: loop_indexs_tmp,
                loop_index: loop_index_tmp,
                step_vars: step_vars_tmp,
                custom_component: &custom_component_tmp,
                native_component: &native_component_tmp,
            };
            // #####################
            let mut new_value = expr_to_literal(new, false, None, data, &mut msg_data, sender)?;

            // only for closure capture the step variables
            capture_variables(&mut &mut new_value, data.step_vars.clone());

            let (lit, name, mem_type, path) = get_var_info(old, None, data, &mut msg_data, sender)?;
            exec_path_actions(
                lit,
                false,
                Some(new_value),
                &path,
                &ContentType::get(&lit),
                &mut new_scope_data,
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
            let flow = search_goto_var_memory(&flow, &mut msg_data, data)?;
            let mut flow_opt = Some(flow.clone());

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
            let mut new_value = expr_to_literal(variable, false, None, data, &mut msg_data, sender)?;

            // only for closure capture the step variables
            capture_variables(&mut &mut new_value, data.step_vars.clone());

            msg_data.add_to_memory(&name.ident, new_value.clone());

            MSG::send(
                &sender,
                MSG::Memory(Memory::new(name.ident.to_owned(), new_value.clone())),
            );

            data.context.current.insert(name.ident.to_owned(), new_value);
            Ok(msg_data)
        }
        reserved => Err(gen_error_info(
            Position::new(interval_from_reserved_fn(reserved)),
            ERROR_START_INSTRUCTIONS.to_owned(),
        )),
    }
}
