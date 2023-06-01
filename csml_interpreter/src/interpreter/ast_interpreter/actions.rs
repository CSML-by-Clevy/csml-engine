use crate::data::data::PreviousInfo;
use crate::data::position::Position;
use crate::data::warnings::DisplayWarnings;
use crate::data::{
    ast::*,
    context::ContextStepInfo,
    data::Data,
    literal::ContentType,
    message::*,
    primitive::{closure::capture_variables, PrimitiveNull, PrimitiveString},
    Literal, Memory, MemoryType, MessageData, MSG,
};
use crate::error_format::*;
use crate::interpreter::variable_handler::{
    exec_path_actions, expr_to_literal,
    forget_memories::{forget_scope_memories, remove_message_data_memories},
    get_var_from_mem,
    interval::*,
    memory::*,
    resolve_fn_args, search_goto_var_memory,
};
use crate::parser::ExitCondition;
use std::collections::HashMap;
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
            Ok(_) => get_var_from_mem(
                var.to_owned(),
                &DisplayWarnings::On,
                path,
                data,
                msg_data,
                sender,
            ),
            // If variable doesn't exist, create the variable in the flow scope 'use' with NULL as value
            // this is done in order to prevent stopping errors
            Err(_) => {
                let lit = PrimitiveNull::get_literal(var.interval.to_owned());

                data.step_vars.insert(var.ident.to_owned(), lit);
                get_var_from_mem(
                    var.to_owned(),
                    &DisplayWarnings::On,
                    path,
                    data,
                    msg_data,
                    sender,
                )
            }
        },
        e => Err(gen_error_info(
            Position::new(interval_from_expr(e), &data.context.flow),
            ERROR_GET_VAR_INFO.to_owned(),
        )),
    }
}

fn check_if_inserted_step<'a>(name: &str, interval: &Interval, data: &'a Data) -> Option<String> {
    match data
        .flow
        .flow_instructions
        .get_key_value(&InstructionScope::InsertStep(InsertStep {
            name: name.to_owned(),
            original_name: None,
            from_flow: "".to_owned(),
            interval: interval.clone(),
        })) {
        Some((InstructionScope::InsertStep(insert_step), _expr)) => {
            Some(insert_step.from_flow.to_owned())
        }
        _ => None,
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
            let lit =
                expr_to_literal(arg, &DisplayWarnings::On, None, data, &mut msg_data, sender)?;

            // check if it is secure variable
            if lit.secure_variable {
                let err = gen_error_info(
                    Position::new(lit.interval, &data.context.flow),
                    "Secure variable can not be displayed".to_owned(),
                );

                MSG::send_error_msg(&sender, &mut msg_data, Err(err));
                Ok(msg_data)
            } else {
                let msg = Message::new(lit, &data.context.flow)?;
                MSG::send(&sender, MSG::Message(msg.clone()));
                Ok(Message::add_to_message(msg_data, MessageType::Msg(msg)))
            }
        }
        ObjectType::Debug(args, interval) => {
            let args = resolve_fn_args(args, data, &mut msg_data, &DisplayWarnings::On, sender)?;

            let lit = args.args_to_debug(interval.to_owned());

            // check if it is secure variable
            if lit.secure_variable {
                let err = gen_error_info(
                    Position::new(lit.interval, &data.context.flow),
                    "Secure variable can not be displayed".to_owned(),
                );

                MSG::send_error_msg(&sender, &mut msg_data, Err(err));
                Ok(msg_data)
            } else {
                let msg = Message::new(lit, &data.context.flow)?;
                MSG::send(&sender, MSG::Message(msg.clone()));
                Ok(Message::add_to_message(msg_data, MessageType::Msg(msg)))
            }
        }
        ObjectType::Log {
            expr,
            interval,
            log_lvl,
        } => {
            let args = resolve_fn_args(expr, data, &mut msg_data, &DisplayWarnings::On, sender)?;
            let log_msg = args.args_to_log();

            MSG::send(
                &sender,
                MSG::Log {
                    flow: data.context.flow.to_owned(),
                    line: interval.start_line,
                    message: log_msg,
                    log_lvl: log_lvl.to_owned(),
                },
            );

            Ok(msg_data)
        }
        ObjectType::Use(arg) => {
            expr_to_literal(arg, &DisplayWarnings::On, None, data, &mut msg_data, sender)?;
            Ok(msg_data)
        }
        ObjectType::Do(DoType::Update(assign_type, old, new)) => {
            // ######################
            // create a temporary scope
            let (
                tmp_default_flow,
                mut tmp_context,
                tmp_event,
                tmp_env,
                tmp_loop_indexes,
                tmp_loop_index,
                mut tmp_step_count,
                tmp_step_limit,
                tmp_step_vars,
            ) = data.copy_scope();

            let mut new_scope_data = Data::new(
                data.flows,
                data.extern_flows,
                data.flow,
                tmp_default_flow,
                &mut tmp_context,
                &tmp_event,
                &tmp_env,
                tmp_loop_indexes,
                tmp_loop_index,
                &mut tmp_step_count,
                tmp_step_limit,
                tmp_step_vars,
                data.previous_info.clone(),
                data.custom_component,
                data.native_component,
            );
            // #####################

            let mut new_value =
                expr_to_literal(new, &DisplayWarnings::On, None, data, &mut msg_data, sender)?;

            // check if it is secure variable
            if new_value.secure_variable {
                let err = gen_error_info(
                    Position::new(new_value.interval, &data.context.flow),
                    "Assignation of secure variable is not allowed".to_owned(),
                );

                MSG::send_error_msg(&sender, &mut msg_data, Err(err));
                return Ok(msg_data);
            }

            // only for closure capture the step variables
            let memory: HashMap<String, Literal> = data.get_all_memories();
            capture_variables(&mut &mut new_value, memory, &data.context.flow);

            let (lit, name, mem_type, path) = get_var_info(old, None, data, &mut msg_data, sender)?;

            let primitive = match assign_type {
                AssignType::AdditionAssignment => {
                    Some(lit.primitive.clone() + new_value.primitive.clone())
                }
                AssignType::SubtractionAssignment => {
                    Some(lit.primitive.clone() - new_value.primitive.clone())
                }
                AssignType::DivisionAssignment => {
                    Some(lit.primitive.clone() / new_value.primitive.clone())
                }
                AssignType::MultiplicationAssignment => {
                    Some(lit.primitive.clone() * new_value.primitive.clone())
                }
                AssignType::RemainderAssignment => {
                    Some(lit.primitive.clone() % new_value.primitive.clone())
                }
                AssignType::Assignment => None,
            };

            match primitive {
                Some(Ok(primitive)) => {
                    new_value = Literal {
                        content_type: new_value.content_type,
                        interval: new_value.interval,
                        additional_info: None,
                        secure_variable: false,
                        primitive,
                    };
                }
                Some(Err(err)) => {
                    new_value = PrimitiveString::get_literal(&err, lit.interval);
                    MSG::send_error_msg(
                        &sender,
                        &mut msg_data,
                        Err(gen_error_info(
                            Position::new(new_value.interval, &new_scope_data.context.flow),
                            err,
                        )),
                    );
                }
                None => {}
            }

            //TODO: refacto memory update system

            let (new_value, update) = if let MemoryType::Constant = mem_type {
                MSG::send_error_msg(
                    &sender,
                    &mut msg_data,
                    Err(gen_error_info(
                        Position::new(new_value.interval, &new_scope_data.context.flow),
                        format!("const variables are immutable"),
                    )),
                );

                (None, false)
            } else {
                (Some(new_value), true)
            };

            exec_path_actions(
                lit,
                &DisplayWarnings::On,
                &mem_type,
                new_value,
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
                update,
                data,
                &mut msg_data,
                sender,
            );

            Ok(msg_data)
        }
        ObjectType::Do(DoType::Exec(expr)) => {
            expr_to_literal(
                expr,
                &DisplayWarnings::On,
                None,
                data,
                &mut msg_data,
                sender,
            )?;
            Ok(msg_data)
        }
        ObjectType::Goto(GotoType::Step(step), interval) => {
            let step = search_goto_var_memory(step, &mut msg_data, data, sender)?;

            // previous flow/step
            match data.previous_info {
                Some(ref mut previous_info) => {
                    previous_info.goto(data.context.flow.clone(), data.context.step.clone());
                }
                None => {
                    data.previous_info = Some(PreviousInfo::new(
                        data.context.flow.clone(),
                        data.context.step.clone(),
                    ))
                }
            }

            let insert_from_flow = check_if_inserted_step(&step, interval, &data);

            // current flow/step
            data.context.step = match insert_from_flow {
                Some(from_flow) => ContextStepInfo::InsertedStep {
                    step: step.to_string(),
                    flow: from_flow,
                },
                None => ContextStepInfo::Normal(step.to_string()),
            };

            MSG::send(
                &sender,
                MSG::Next {
                    flow: None,
                    step: Some(data.context.step.clone()),
                    bot: None,
                },
            );

            msg_data.exit_condition = Some(ExitCondition::Goto);

            if step == "end" {
                msg_data.exit_condition = Some(ExitCondition::End);
            }

            Ok(msg_data)
        }
        ObjectType::Goto(GotoType::Flow(flow), ..) => {
            let flow = search_goto_var_memory(&flow, &mut msg_data, data, sender)?;

            MSG::send(
                &sender,
                MSG::Next {
                    flow: Some(flow.to_string()),
                    step: None,
                    bot: None,
                },
            );

            // previous flow/step
            match data.previous_info {
                Some(ref mut previous_info) => {
                    previous_info.goto(data.context.flow.clone(), data.context.step.clone());
                }
                None => {
                    data.previous_info = Some(PreviousInfo::new(
                        data.context.flow.clone(),
                        data.context.step.clone(),
                    ))
                }
            }
            // current flow/step
            data.context.step = ContextStepInfo::Normal("start".to_string());
            data.context.flow = flow.to_string();

            msg_data.exit_condition = Some(ExitCondition::Goto);

            Ok(msg_data)
        }
        ObjectType::Goto(
            GotoType::StepFlow {
                step,
                flow,
                bot: None,
            },
            interval,
        ) => {
            let step = match step {
                Some(step) => search_goto_var_memory(&step, &mut msg_data, data, sender)?,
                None => "start".to_owned(), // default value start step
            };
            let flow = match flow {
                Some(flow) => search_goto_var_memory(&flow, &mut msg_data, data, sender)?,
                None => data.context.flow.to_owned(), // default value current flow
            };

            let mut flow_opt = Some(flow.clone());

            msg_data.exit_condition = Some(ExitCondition::Goto);

            if step == "end" {
                msg_data.exit_condition = Some(ExitCondition::End);
                flow_opt = None;
            }

            // previous flow/step
            match data.previous_info {
                Some(ref mut previous_info) => {
                    previous_info.goto(data.context.flow.clone(), data.context.step.clone());
                }
                None => {
                    data.previous_info = Some(PreviousInfo::new(
                        data.context.flow.clone(),
                        data.context.step.clone(),
                    ))
                }
            }

            // current flow/step
            data.context.flow = flow.to_string();

            let insert_from_flow = check_if_inserted_step(&step, interval, &data);

            // current flow/step
            data.context.step = match insert_from_flow {
                Some(from_flow) => ContextStepInfo::InsertedStep {
                    step: step.to_string(),
                    flow: from_flow,
                },
                None => ContextStepInfo::Normal(step.to_string()),
            };

            MSG::send(
                &sender,
                MSG::Next {
                    flow: flow_opt,
                    step: Some(data.context.step.clone()),
                    bot: None,
                },
            );

            Ok(msg_data)
        }

        ObjectType::Goto(
            GotoType::StepFlow {
                step,
                flow,
                bot: Some(next_bot),
            },
            ..,
        ) => {
            let step = match step {
                Some(step) => ContextStepInfo::UnknownFlow(search_goto_var_memory(
                    &step,
                    &mut msg_data,
                    data,
                    sender,
                )?),
                None => ContextStepInfo::Normal("start".to_owned()), // default value start step
            };
            let flow = match flow {
                Some(flow) => search_goto_var_memory(&flow, &mut msg_data, data, sender).ok(),
                None => None,
            };

            let bot = search_goto_var_memory(&next_bot, &mut msg_data, data, sender)?;

            msg_data.exit_condition = Some(ExitCondition::End);

            MSG::send(
                &sender,
                MSG::Next {
                    step: Some(step),
                    flow: flow,
                    bot: Some(bot), // need to send previous flow / step / bot info
                },
            );

            Ok(msg_data)
        }

        ObjectType::Previous(previous_type, _) => {
            let flow_opt;
            let mut step_opt = None;

            match (previous_type, &mut data.previous_info) {
                (PreviousType::Flow(_interval), Some(ref mut previous_info)) => {
                    let tmp_f = previous_info.flow.clone();
                    flow_opt = Some(tmp_f.clone());

                    previous_info.flow = data.context.flow.clone();
                    previous_info.step_at_flow =
                        (data.context.step.clone(), data.context.flow.clone());

                    data.context.flow = tmp_f;
                    data.context.step = ContextStepInfo::Normal("start".to_string());
                }
                (PreviousType::Step(_interval), Some(ref mut previous_info)) => {
                    let (tmp_s, tmp_f) = previous_info.step_at_flow.clone();
                    flow_opt = Some(tmp_f.clone());
                    step_opt = Some(tmp_s.clone());

                    if data.context.flow != tmp_f {
                        previous_info.flow = tmp_f.clone();
                    }
                    previous_info.step_at_flow =
                        (data.context.step.clone(), data.context.flow.clone());

                    data.context.flow = tmp_f;
                    data.context.step = tmp_s;
                }
                (_, None) => {
                    flow_opt = None;
                    step_opt = Some(ContextStepInfo::Normal("end".to_owned()));

                    data.context.step = ContextStepInfo::Normal("end".to_string());
                }
            }

            msg_data.exit_condition = Some(ExitCondition::Goto);

            MSG::send(
                &sender,
                MSG::Next {
                    flow: flow_opt,
                    step: step_opt,
                    bot: None,
                },
            );

            Ok(msg_data)
        }
        ObjectType::Remember(name, variable) => {
            let mut new_value = expr_to_literal(
                variable,
                &DisplayWarnings::On,
                None,
                data,
                &mut msg_data,
                sender,
            )?;

            // check if it is secure variable
            if new_value.secure_variable {
                let err = gen_error_info(
                    Position::new(new_value.interval, &data.context.flow),
                    "Assignation of secure variable is not allowed".to_owned(),
                );

                MSG::send_error_msg(&sender, &mut msg_data, Err(err));
                return Ok(msg_data);
            }

            // only for closure capture the step variables
            let memory: HashMap<String, Literal> = data.get_all_memories();
            capture_variables(&mut &mut new_value, memory, &data.context.flow);

            msg_data.add_to_memory(&name.ident, new_value.clone());

            MSG::send(
                &sender,
                MSG::Remember(Memory::new(name.ident.to_owned(), new_value.clone())),
            );

            data.context
                .current
                .insert(name.ident.to_owned(), new_value);
            Ok(msg_data)
        }
        ObjectType::Forget(memory, _interval) => {
            // delete memories form message data
            remove_message_data_memories(&memory, &mut msg_data);
            // delete memory from current scope
            forget_scope_memories(&memory, data);

            MSG::send(&sender, MSG::Forget(memory.to_owned()));

            Ok(msg_data)
        }

        reserved => Err(gen_error_info(
            Position::new(interval_from_reserved_fn(reserved), &data.context.flow),
            ERROR_START_INSTRUCTIONS.to_owned(),
        )),
    }
}
