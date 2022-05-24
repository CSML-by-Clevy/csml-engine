use crate::db_connectors::{conversations::*, memories::*, messages::*, state::*};
use crate::utils::*;
use crate::{data::*, delete_client_memories};

use csml_interpreter::{
    data::{
        ast::ForgetMemory, csml_bot::CsmlBot, csml_flow::CsmlFlow, csml_logs::*, Client, Event,
        Hold, Memory, Message, MultiBot, MSG,
    },
    interpret,
};
use serde_json::{map::Map, Value};
use std::collections::HashMap;
use std::{sync::mpsc, thread};

#[derive(Debug, Clone)]
enum InterpreterReturn {
    Continue,
    End,
    SwitchBot(SwitchBot),
}

#[derive(Debug, Clone)]
pub struct SwitchBot {
    pub bot_id: String,
    pub version_id: Option<String>,
    pub flow: Option<String>,
    pub step: String,
}

/**
 * This is the CSML Engine action.
 * A request came in and should be handled. Once the ConversationInfo is correctly setup,
 * this step is called in a loop until a `hold` or `goto end` is reached.
 */
pub fn interpret_step(
    data: &mut ConversationInfo,
    event: Event,
    bot: &CsmlBot,
) -> Result<(Map<String, Value>, Option<SwitchBot>), EngineError> {
    let mut current_flow: &CsmlFlow = get_flow_by_id(&data.context.flow, &bot.flows)?;
    let mut interaction_order = 0;
    let mut conversation_end = false;
    let (sender, receiver) = mpsc::channel::<MSG>();
    let context = data.context.clone();
    let mut switch_bot = None;

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
            MSG::Next {
                flow,
                step,
                bot: None,
            } => {
                if let Ok(InterpreterReturn::End) = manage_internal_goto(
                    data,
                    &mut conversation_end,
                    &mut interaction_order,
                    &mut current_flow,
                    &bot,
                    &mut memories,
                    flow,
                    step,
                ) {
                    break;
                }
            }

            MSG::Next {
                flow,
                step,
                bot: Some(target_bot),
            } => {
                if let Ok(InterpreterReturn::SwitchBot(s_bot)) =
                    manage_switch_bot(data, &mut interaction_order, &bot, flow, step, target_bot)
                {
                    switch_bot = Some(s_bot);
                    break;
                }
            }

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

    Ok((
        messages_formatter(
            data,
            data.messages.clone(),
            interaction_order,
            conversation_end,
        ),
        switch_bot,
    ))
}

fn manage_switch_bot<'a>(
    data: &mut ConversationInfo,
    interaction_order: &mut i32,
    bot: &'a CsmlBot,
    flow: Option<String>,
    step: Option<String>,
    target_bot: String,
) -> Result<InterpreterReturn, EngineError> {
    // check if we are allow to switch to 'target_bot'

    let next_bot = if let Some(multi_bot) = &bot.multi_bot {
        multi_bot.iter().find(
            |&MultiBot {
                 id,
                 name,
                 bot_version: _,
             }| target_bot == *id || target_bot == *name,
        )
    } else {
        None
    };

    let next_bot = match next_bot {
        Some(next_bot) => next_bot,
        None => {
            csml_logger(
                CsmlLog::new(
                    None,
                    Some(data.context.flow.to_string()),
                    None,
                    format!("Switching to Bot: ({}) is not allowed", target_bot),
                ),
                LogLvl::Error,
            );
            return Ok(InterpreterReturn::End);
        }
    };

    let (flow, step) = match (flow, step) {
        (Some(flow), Some(step)) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    None,
                    None,
                    format!(
                        "goto flow: {flow}, step: {step} in bot: {target_bot} from: flow: {} step: {} in bot: {}",
                        data.context.flow, data.context.step, bot.id
                    ),
                ),
                LogLvl::Debug,
            );

            (Some(flow), step)
        }
        (Some(flow), None) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    None,
                    None,
                    format!(
                        "goto flow: {flow}, step: start in bot: {target_bot} from: flow: {} step: {} in bot: {}",
                        data.context.flow, data.context.step, bot.id
                    ),
                ),
                LogLvl::Debug,
            );

            (Some(flow), "start".to_owned())
        }
        (None, Some(step)) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    None,
                    None,
                    format!(
                        "goto flow: default_flow, step: {step} in bot: {target_bot} from: flow: {} step: {} in bot: {}",
                        data.context.flow, data.context.step, bot.id
                    ),
                ),
                LogLvl::Debug,
            );

            (None, step)
        }
        (None, None) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    Some(data.context.flow.to_string()),
                    None,
                    format!(
                        "goto flow: default_flow step: start in bot: {target_bot} from: flow: {} step: {} in bot: {}",
                        data.context.flow, data.context.step, bot.id
                     ),
                ),
                LogLvl::Debug,
            );

            (None, "start".to_owned())
        }
    };

    // send message switch bot
    send_msg_to_callback_url(
        data,
        vec![Message::switch_bot_message(&next_bot.id, &data.client)],
        *interaction_order,
        true,
    );

    csml_logger(
        CsmlLog::new(
            None,
            Some(data.context.flow.to_string()),
            None,
            format!("switch bot"),
        ),
        LogLvl::Info,
    );

    close_conversation(&data.conversation_id, &data.client, &mut data.db)?;

    let previous_bot: Value = serde_json::json!({
        "bot": data.client.bot_id,
        "flow": data.context.step,
        "step": data.context.step,
    });

    set_state_items(
        &Client::new(
            next_bot.id.to_owned(),
            data.client.channel_id.clone(),
            data.client.user_id.clone(),
        ),
        "bot",
        vec![("previous", &previous_bot)],
        data.ttl,
        &mut data.db,
    )?;

    Ok(InterpreterReturn::SwitchBot(SwitchBot {
        bot_id: next_bot.id.to_owned(),
        version_id: next_bot.bot_version.to_owned(),
        flow,
        step,
    }))
}

fn manage_internal_goto<'a>(
    data: &mut ConversationInfo,
    conversation_end: &mut bool,
    interaction_order: &mut i32,
    current_flow: &mut &'a CsmlFlow,
    bot: &'a CsmlBot,
    memories: &mut HashMap<String, Memory>,
    flow: Option<String>,
    step: Option<String>,
) -> Result<InterpreterReturn, EngineError> {
    match (flow, step) {
        (Some(flow), Some(step)) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    None,
                    None,
                    format!(
                        "goto flow: {}, step: {} from: flow: {} step: {}",
                        flow, step, data.context.flow, data.context.step
                    ),
                ),
                LogLvl::Debug,
            );
            update_current_context(data, &memories);
            goto_flow(data, interaction_order, current_flow, &bot, flow, step)?
        }
        (Some(flow), None) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    None,
                    None,
                    format!(
                        "goto flow: {}, step: start from: flow: {} step: {}",
                        flow, data.context.flow, data.context.step
                    ),
                ),
                LogLvl::Debug,
            );
            update_current_context(data, &memories);
            let step = "start".to_owned();
            goto_flow(data, interaction_order, current_flow, &bot, flow, step)?
        }
        (None, Some(step)) => {
            csml_logger(
                CsmlLog::new(
                    Some(&data.client),
                    None,
                    None,
                    format!(
                        "goto flow: {}, step: {} from: flow: {} step: {}",
                        data.context.flow, step, data.context.flow, data.context.step
                    ),
                ),
                LogLvl::Debug,
            );
            if goto_step(data, conversation_end, interaction_order, step)? {
                return Ok(InterpreterReturn::End);
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
                        data.context.flow, data.context.step
                    ),
                ),
                LogLvl::Debug,
            );

            let step = "end".to_owned();
            if goto_step(data, conversation_end, interaction_order, step)? {
                return Ok(InterpreterReturn::End);
            }
        }
    }

    Ok(InterpreterReturn::Continue)
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
    nextstep: String,
) -> Result<(), EngineError> {
    *current_flow = get_flow_by_id(&nextflow, &bot.flows)?;
    data.context.flow = nextflow;
    data.context.step = nextstep;

    update_conversation(
        data,
        Some(current_flow.id.clone()),
        Some(data.context.step.clone()),
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
    nextstep: String,
) -> Result<bool, EngineError> {
    if nextstep == "end" {
        *conversation_end = true;

        // send end of conversation
        send_msg_to_callback_url(data, vec![], *interaction_order, *conversation_end);
        close_conversation(&data.conversation_id, &data.client, &mut data.db)?;

        // break interpret_step loop
        return Ok(*conversation_end);
    } else {
        data.context.step = nextstep;
        update_conversation(data, None, Some(data.context.step.to_owned()))?;
    }

    *interaction_order += 1;
    Ok(false)
}
