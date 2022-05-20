use crate::db_connectors::{conversations::*, memories::*, messages::*, state::*};
use crate::utils::*;
use crate::{data::*, delete_client_memories};

use csml_interpreter::data::context::ContextStepInfo;
use csml_interpreter::{
    data::{
        ast::ForgetMemory, csml_bot::CsmlBot, csml_flow::CsmlFlow, csml_logs::*, Event, Hold, MSG,
    },
    interpret,
};
use serde_json::{map::Map, Value};
use std::collections::HashMap;
use std::{sync::mpsc, thread};

/**
 * This is the CSML Engine action.
 * A request came in and should be handled. Once the ConversationInfo is correctly setup,
 * this step is called in a loop until a `hold` or `goto end` is reached.
 */
pub fn interpret_step(
    data: &mut ConversationInfo,
    event: Event,
    bot: &CsmlBot,
) -> Result<Map<String, Value>, EngineError> {
    let mut current_flow: &CsmlFlow = get_flow_by_id(&data.context.flow, &bot.flows)?;
    let mut interaction_order = 0;
    let mut conversation_end = false;
    let (sender, receiver) = mpsc::channel::<MSG>();
    let context = data.context.clone();

    csml_logger(
        CsmlLog::new(
            None,
            Some(data.context.flow.to_string()),
            None,
            format!("interpreter: start interpretations of bot {:?}", bot.id),
        ),
        LogLvl::Info,
    );
    csml_logger(
        CsmlLog::new(
            Some(&data.client),
            Some(data.context.flow.to_string()),
            None,
            format!(
                "interpreter: start interpretations of bot {:?}, with ",
                bot.id
            ),
        ),
        LogLvl::Debug,
    );
    let new_bot = bot.clone();
    thread::spawn(move || {
        interpret(new_bot, context, event, Some(sender));
    });

    let mut memories = HashMap::new();

    for received in receiver {
        match received {
            MSG::Remember(mem) => {
                memories.insert(mem.key.clone(), mem);
            }
            MSG::Forget(mem) => match mem {
                ForgetMemory::ALL => {
                    memories.clear();
                    delete_client_memories(&data.client)?;
                }
                ForgetMemory::SINGLE(memory) => {
                    memories.remove(&memory.ident);
                    crate::delete_client_memory(&data.client, &memory.ident)?;
                }
                ForgetMemory::LIST(mem_list) => {
                    for mem in mem_list.iter() {
                        memories.remove(&mem.ident);
                        crate::delete_client_memory(&data.client, &mem.ident)?;
                    }
                }
            },
            MSG::Message(msg) => {
                csml_logger(
                    CsmlLog::new(
                        None,
                        Some(data.context.flow.to_string()),
                        None,
                        format!("sending message"),
                    ),
                    LogLvl::Info,
                );
                csml_logger(
                    CsmlLog::new(
                        Some(&data.client),
                        Some(data.context.flow.to_string()),
                        None,
                        format!("sending message {:?}", msg),
                    ),
                    LogLvl::Debug,
                );

                println!("{:?}", msg);

                send_msg_to_callback_url(data, vec![msg.clone()], interaction_order, false);
                data.messages.push(msg);
            }
            MSG::Log {
                flow,
                line,
                message,
                log_lvl,
            } => {
                csml_logger(
                    CsmlLog::new(Some(&data.client), Some(flow), Some(line), message),
                    log_lvl,
                );
            }
            MSG::Hold(Hold {
                index,
                step_vars,
                step_name,
                flow_name,
                previous,
                secure,
            }) => {
                let hash = get_current_step_hash(&data.context, bot)?;
                let state_hold: Value = serde_json::json!({
                    "index": index,
                    "step_vars": step_vars,
                    "hash": hash,
                    "previous": previous,
                    "secure": secure
                });

                csml_logger(
                    CsmlLog::new(
                        None,
                        Some(data.context.flow.to_string()),
                        None,
                        format!("hold bot"),
                    ),
                    LogLvl::Info,
                );
                csml_logger(
                    CsmlLog::new(
                        Some(&data.client),
                        Some(data.context.flow.to_string()),
                        None,
                        format!("hold bot, state_hold {:?}", state_hold),
                    ),
                    LogLvl::Debug,
                );

                set_state_items(
                    &data.client,
                    "hold",
                    vec![("position", &state_hold)],
                    data.ttl,
                    &mut data.db,
                )?;
                data.context.hold = Some(Hold {
                    index,
                    step_vars,
                    step_name,
                    flow_name,
                    previous,
                    secure,
                });
            }
            MSG::Next { flow, step } => match (flow, step) {
                (Some(flow), Some(step)) => {
                    csml_logger(
                        CsmlLog::new(
                            Some(&data.client),
                            None,
                            None,
                            format!(
                                "goto flow: {}, step: {} from: flow: {} step: {}",
                                flow,
                                step.get_step(),
                                data.context.flow,
                                data.context.step.get_step()
                            ),
                        ),
                        LogLvl::Debug,
                    );
                    update_current_context(data, &memories);
                    goto_flow(
                        data,
                        &mut interaction_order,
                        &mut current_flow,
                        &bot,
                        flow,
                        step,
                    )?
                }
                (Some(flow), None) => {
                    csml_logger(
                        CsmlLog::new(
                            Some(&data.client),
                            None,
                            None,
                            format!(
                                "goto flow: {}, step: start from: flow: {} step: {}",
                                flow,
                                data.context.flow,
                                data.context.step.get_step()
                            ),
                        ),
                        LogLvl::Debug,
                    );
                    update_current_context(data, &memories);
                    let step = ContextStepInfo::Normal("start".to_owned());
                    goto_flow(
                        data,
                        &mut interaction_order,
                        &mut current_flow,
                        &bot,
                        flow,
                        step,
                    )?
                }
                (None, Some(step)) => {
                    csml_logger(
                        CsmlLog::new(
                            Some(&data.client),
                            None,
                            None,
                            format!(
                                "goto flow: {}, step: {} from: flow: {} step: {}",
                                data.context.flow,
                                step.get_step(),
                                data.context.flow,
                                data.context.step.get_step()
                            ),
                        ),
                        LogLvl::Debug,
                    );
                    if goto_step(data, &mut conversation_end, &mut interaction_order, step)? {
                        break;
                    }
                }
                (None, None) => {
                    csml_logger(
                        CsmlLog::new(
                            Some(&data.client),
                            Some(data.context.flow.to_string()),
                            None,
                            format!(
                                "goto end from: flow: {} step: {}",
                                data.context.flow,
                                data.context.step.get_step()
                            ),
                        ),
                        LogLvl::Debug,
                    );

                    let step = ContextStepInfo::Normal("end".to_owned());
                    if goto_step(data, &mut conversation_end, &mut interaction_order, step)? {
                        break;
                    }
                }
            },
            MSG::Error(err_msg) => {
                conversation_end = true;
                csml_logger(
                    CsmlLog::new(
                        Some(&data.client),
                        Some(data.context.flow.to_string()),
                        None,
                        format!("interpreter error: {:?}", err_msg),
                    ),
                    LogLvl::Error,
                );

                send_msg_to_callback_url(data, vec![err_msg.clone()], interaction_order, true);
                data.messages.push(err_msg);
                close_conversation(&data.conversation_id, &data.client, &mut data.db)?;
            }
        }
    }

    // save in db
    let msgs: Vec<serde_json::Value> = data
        .messages
        .iter()
        .map(|var| var.clone().message_to_json())
        .collect();

    if !data.low_data {
        add_messages_bulk(data, msgs, interaction_order, "SEND")?;
    }
    add_memories(data, &memories)?;

    Ok(messages_formatter(
        data,
        data.messages.clone(),
        interaction_order,
        conversation_end,
    ))
}

/**
 * CSML `goto flow` action
 */
fn goto_flow<'a>(
    data: &mut ConversationInfo,
    interaction_order: &mut i32,
    current_flow: &mut &'a CsmlFlow,
    bot: &'a CsmlBot,
    nextflow: String,
    nextstep: ContextStepInfo,
) -> Result<(), EngineError> {
    *current_flow = get_flow_by_id(&nextflow, &bot.flows)?;
    data.context.flow = nextflow;
    data.context.step = nextstep;

    update_conversation(
        data,
        Some(current_flow.id.clone()),
        Some(data.context.step.get_step()),
    )?;

    *interaction_order += 1;

    Ok(())
}

/**
 * CSML `goto step` action
 */
fn goto_step<'a>(
    data: &mut ConversationInfo,
    conversation_end: &mut bool,
    interaction_order: &mut i32,
    nextstep: ContextStepInfo,
) -> Result<bool, EngineError> {
    if nextstep.is_step("end") {
        *conversation_end = true;

        // send end of conversation
        send_msg_to_callback_url(data, vec![], *interaction_order, *conversation_end);
        close_conversation(&data.conversation_id, &data.client, &mut data.db)?;

        // break interpret_step loop
        return Ok(*conversation_end);
    } else {
        data.context.step = nextstep;
        update_conversation(data, None, Some(data.context.step.get_step()))?;
    }

    *interaction_order += 1;
    Ok(false)
}
