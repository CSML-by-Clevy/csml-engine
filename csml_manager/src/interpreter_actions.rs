use crate::data::*;
use crate::db_interactions::{
    conversation::*, interactions::*, memories::*, messages::*, nodes::*, state::*,
};
use crate::tools::*;

use csmlinterpreter::{
    data::{csml_flow::CsmlFlow, Event, Hold, MSG},
    interpret,
};
use md5::{Digest, Md5};
use serde_json::{map::Map, Value};
use std::{env, sync::mpsc, thread, time::SystemTime};

pub fn interpret_step(
    data: &mut ConversationInfo,
    event: Event,
    csmldata: &CsmlData,
) -> Result<Map<String, Value>, ManagerError> {
    let mut current_flow: &CsmlFlow = get_flow_by_id(&data.context.flow, &csmldata.bot.flows)?;
    let mut interaction_order = 0;
    let mut conversation_end = false;
    let mut interaction_success = true;
    let bot = csmldata.bot.clone();
    let (sender, receiver) = mpsc::channel::<MSG>();
    let context = data.context.clone();
    let interpret_step = SystemTime::now();

    thread::spawn(move || {
        interpret(bot, context, event, Some(sender));
    });

    let mut memories = vec![];

    for received in receiver {
        match received {
            MSG::Memory(mem) => memories.push(mem),
            MSG::Message(msg) => {
                send_and_display_msg(data, vec![msg.clone()], interaction_order, false);
                data.messages.push(msg);
            }
            MSG::Hold(Hold {
                index: new_index,
                step_vars,
            }) => {
                let mut hash = Md5::new();

                // delete info of last_flow if any in order to prevent recursive flow
                data.last_flow = None;

                hash.input(current_flow.content.as_bytes());

                let state_hold: Value = serde_json::json!({
                    "index": new_index,
                    "step_vars": step_vars,
                    "hash": format!("{:x}", hash.result())
                });
                set_state_items(
                    data,
                    "hold",
                    interaction_order,
                    vec![("position", &state_hold)],
                )?;
                data.context.hold = Some(Hold {
                    index: new_index,
                    step_vars,
                });
            }
            MSG::Next { flow, step } => match (flow, step) {
                (Some(flow), Some(step)) => {
                    update_memories_in_data(data, &memories);
                    goto_flow(
                        data,
                        &mut interaction_order,
                        &mut current_flow,
                        csmldata,
                        flow,
                        step,
                    )?
                }
                (Some(flow), None) => {
                    update_memories_in_data(data, &memories);
                    let step = "start".to_owned();
                    goto_flow(
                        data,
                        &mut interaction_order,
                        &mut current_flow,
                        csmldata,
                        flow,
                        step,
                    )?
                }
                (None, Some(step)) => {
                    if goto_step(
                        data,
                        &mut conversation_end,
                        &mut interaction_order,
                        &mut current_flow,
                        csmldata,
                        step,
                    )? {
                        break;
                    }
                }
                (None, None) => {
                    let step = "end".to_owned();
                    if goto_step(
                        data,
                        &mut conversation_end,
                        &mut interaction_order,
                        &mut current_flow,
                        csmldata,
                        step,
                    )? {
                        break;
                    }
                }
            },
            MSG::Error(err_msg) => {
                conversation_end = true;
                interaction_success = false;
                send_and_display_msg(data, vec![err_msg.clone()], interaction_order, true);
                data.messages.push(err_msg);
                close_conversation(&data.conversation_id, &data.client, &data.db)?;
            }
        }
    }

    if let Ok(var) = env::var(DEBUG) {
        if var == "true" {
            let el = interpret_step.elapsed()?;
            println!(
                "Total Time interpret step {} - {}.{}",
                data.context.step,
                el.as_secs(),
                el.as_millis()
            );
        }
    }

    let now = SystemTime::now();
    // save in db
    let msgs: Vec<serde_json::Value> = data
        .messages
        .iter()
        .map(|var| var.clone().message_to_json())
        .collect();

    add_messages_bulk(data, msgs, interaction_order, "SEND")?;
    add_memories(data, &memories, interaction_order)?;

    if let Ok(var) = env::var(DEBUG) {
        if var == "true" {
            let el = now.elapsed()?;
            println!(
                "Save message & memories bulk at the end of step - {}.{}",
                el.as_secs(),
                el.as_millis()
            );
        }
    }

    update_interaction(data, interaction_success)?;

    Ok(messages_formater(
        data,
        data.messages.clone(),
        interaction_order,
        conversation_end,
    ))
}

fn goto_flow<'a>(
    data: &mut ConversationInfo,
    interaction_order: &mut i32,
    current_flow: &mut &'a CsmlFlow,
    csmldata: &'a CsmlData,
    nextflow: String,
    nextstep: String,
) -> Result<(), ManagerError> {
    data.last_flow = Some((data.context.flow.clone(), data.context.step.clone()));
    create_node(data, Some(nextflow.clone()), Some(nextstep.clone()))?;

    *current_flow = get_flow_by_id(&nextflow, &csmldata.bot.flows)?;
    data.context.flow = nextflow;
    data.context.step = nextstep;

    update_conversation(
        &data,
        Some(current_flow.id.clone()),
        Some(data.context.step.clone()),
    )?;

    *interaction_order += 1;
    update_interaction(data, false)?;

    Ok(())
}

fn goto_step<'a>(
    data: &mut ConversationInfo,
    conversation_end: &mut bool,
    interaction_order: &mut i32,
    current_flow: &mut &'a CsmlFlow,
    csmldata: &'a CsmlData,
    nextstep: String,
) -> Result<bool, ManagerError> {
    create_node(data, None, Some(nextstep.clone()))?;

    if nextstep == "end" {
        *conversation_end = true;

        if let Some((flow_name, step)) = &data.last_flow {
            *conversation_end = false;
            *current_flow = get_flow_by_id(&flow_name, &csmldata.bot.flows)?;
            data.context.flow = current_flow.name.clone();
            data.context.step = step.to_owned();
            update_conversation(
                &data,
                Some(current_flow.id.to_owned()),
                Some(data.context.step.to_owned()),
            )?;
        } else {
            // send end of conversation
            send_and_display_msg(data, vec![], *interaction_order, *conversation_end);
            update_conversation(&data, None, Some("end".to_owned()))?;
            close_conversation(&data.conversation_id, &data.client, &data.db)?;
        }
        data.last_flow = None;
        // break interpret_step loop
        return Ok(*conversation_end);
    } else {
        data.context.step = nextstep;
        update_conversation(&data, None, Some(data.context.step.to_owned()))?;
    }

    *interaction_order += 1;
    Ok(false)
}
