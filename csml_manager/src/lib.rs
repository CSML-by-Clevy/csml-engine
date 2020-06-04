pub mod data;
mod db_interactions;
mod encrypt;
mod send;
mod tools;

use data::ManagerError;
use db_interactions::{
    conversation::*, interactions::*, memories::*, messages::*, nodes::*, state::*,
};

use csmlinterpreter::{
    data::{
        csml_bot::CsmlBot, csml_flow::CsmlFlow, Client, ContextJson, Event, Hold, Memories,
        Message, MSG, csml_result::CsmlResult, error_info::ErrorInfo
    },
    interpret,
};
use data::*;
use md5::{Digest, Md5};
use serde_json::{map::Map, Value};
use std::{env, sync::mpsc, thread, time::SystemTime};
use tools::*;

// pub fn step_exists(file: &str, step_name: &str) -> Result<bool, String> {
//     let flow: Flow = match parse_flow(file) {
//         Ok(flow) => flow,
//         Err(e) => return Err(format!("Error in parsing Flow : {:?}", e)),
//     };

//     match flow
//         .flow_instructions
//         .get(&InstructionType::NormalStep(step_name.to_owned()))
//     {
//         Some(_) => Ok(true),
//         None => Ok(false),
//     }
// }

pub fn validate_bot(bot: CsmlBot) -> Result<bool, Vec<ErrorInfo> > {
    match csmlinterpreter::validate_bot(bot) {
        CsmlResult{
            flows: _,
            warnings: _,
            errors: None,
        } => Ok(true),
        CsmlResult{
            flows: _,
            warnings: _,
            errors: Some(e),
        } => Err(e),
    }
}

pub fn user_close_all_conversations(
    client: Client,
) -> Result<(), ManagerError> {
    let mongo_client = mongodb::Client::with_uri_str("mongodb://localhost:2717/")?;
    let db = mongo_client.database("csml"); // tmp name
    close_all_conversations(&client, &db)
}

// reset memory if flow hash is different or see if there are some save tmp memories
fn check_for_hold(data: &mut ConversationInfo, flow: &CsmlFlow) -> Result<(), ManagerError> {
    match get_state_key(&data.client, "hold", "position", &data.db) {
        Ok(Some(string)) => {
            let hold = serde_json::to_value(string)?;
            let mut hash = Md5::new();

            hash.input(flow.content.as_bytes());
            let new_hash = format!("{:x}", hash.result());

            if new_hash != hold["hash"] {
                data.context.step = "start".to_owned();
                delete_state_key(&data.client, "hold", "position", &data.db)?;
                data.context.hold = None;
                return Ok(());
            }
            data.context.hold = Some(Hold {
                index: hold["index"].as_u64().ok_or(ManagerError::Interpreter(
                    "hold index bad format".to_owned(),
                ))? as usize,
                step_vars: hold["step_vars"].clone(),
            });
            delete_state_key(&data.client, "hold", "position", &data.db)?;
        }
        Ok(None) => (),
        Err(_) => (),
    };
    Ok(())
}

fn get_conversation<'a>(
    context: &mut ContextJson,
    bot: &'a CsmlBot,
    flow_found: Option<&'a CsmlFlow>, // FlowInfo<'a>
    metadata: Value,
    client: &Client,
    db: &mongodb::Database,
) -> Result<bson::Bson, ManagerError> {
    match get_latest_open(client, db)? {
        Some(doc) => {
            let conversation: Conversation = bson::from_bson(bson::Bson::Document(doc))?;

            //TODO: check for recursion
            match flow_found {
                Some(flow) => {
                    context.step = "start".to_owned();
                    context.flow = flow.name.to_owned();
                }
                //TODO: see if need to create a new conversation or create a last_flow
                None => {
                    let flow = match get_flow_by_id(&conversation.flow_id, &bot.flows) {
                        Ok(flow) => flow,
                        Err(e) => {
                            close_conversation(&bson::Bson::ObjectId(conversation.id), &client, &db)?;
                            return Err(e)
                        }
                    };

                    context.step = conversation.step_id.to_owned();
                    context.flow = flow.name.to_owned();
                }
            };

            Ok(bson::Bson::ObjectId(conversation.id))
        }
        None => {
            let flow = match flow_found {
                Some(flow) => flow,
                None => get_default_flow(bot)?,
            };
            context.step = "start".to_owned();
            context.flow = flow.name.to_owned();

            let conversation_id =
                create_conversation(&flow.id, &context.step, client, metadata.clone(), db)?;

            Ok(conversation_id)
        }
    }
}

fn init_conversation_info<'a>(
    default_flow: String,
    event: &Event,
    csmldata: &'a CsmlData,
) -> Result<ConversationInfo, ManagerError> {
    // TODO: mongo uri
    let client = mongodb::Client::with_uri_str("mongodb://localhost:2717/")?;
    let db = client.database("csml"); // tmp name

    let interaction_id = init_interaction(csmldata.payload.clone(), &csmldata.client, &db)?;
    let mut context = init_context(
        default_flow,
        csmldata.client.clone(),
        &csmldata.bot.fn_endpoint,
    );

    let curl = match csmldata.callback_url {
        Some(ref url) => {
            if let Ok(curl) = init_curl(url) {
                Some(curl)
            } else {
                None
            }
        }
        None => None,
    };

    let flow_found = search_flow(event, &csmldata.bot, &csmldata.client, &db).ok();
    let conversation_id = get_conversation(
        &mut context,
        &csmldata.bot,
        flow_found,
        csmldata.metadata.clone(),
        &csmldata.client,
        &db,
    )?;

    get_memories(
        &csmldata.client,
        // &conversation_id,
        &mut context,
        &csmldata.metadata,
        &db,
    )?;

    let data = ConversationInfo {
        conversation_id,
        interaction_id,
        context,
        metadata: csmldata.metadata.clone(), // ??
        request_id: csmldata.request_id.clone(),
        curl,
        last_flow: None,
        client: csmldata.client.clone(),
        messages: vec![],
        db,
    };

    update_conversation(
        &data,
        Some(data.context.flow.to_owned()),
        Some(data.context.step.to_owned()),
    )?;

    Ok(data)
}

pub fn start_conversation(
    json_event: Value,
    csmldata: CsmlData,
) -> Result<Map<String, Value>, ManagerError> {
    let now = SystemTime::now();

    let event = format_event(json_event.clone())?;
    let mut data = init_conversation_info(
        get_default_flow(&csmldata.bot)?.name.to_owned(),
        &event,
        &csmldata,
    )?;

    // save event in db as message RECEIVE
    let event_receive = format_event_message(&mut data, json_event)?;
    add_messages_bulk(&mut data, vec![event_receive])?;

    let flow = get_flow_by_id(&data.context.flow, &csmldata.bot.flows)?;
    check_for_hold(&mut data, flow)?;

    let res = interpret_step(&mut data, event.to_owned(), &csmldata);

    if let Ok(var) = env::var(DEBUG) {
        if var == "true" {
            let el = now.elapsed()?;
            println!("Total time Manager - {}.{}", el.as_secs(), el.as_millis());
        }
    }
    res
}

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
                println!("msg => {:?}", msg);
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
                let state_body = format_state_body(data, "hold", vec![("position", &state_hold)])?;
                set_state_items(&data, state_body)?;
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
                    println!("memories => {:?}", memories);
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

    let mem = format_memories(data, &memories)?;
    let format_msg = format_messages(data, &data.messages, interaction_order, "SEND")?;

    let now = SystemTime::now();
    // save in db
    add_messages_bulk(data, format_msg)?;
    add_memories(data, mem)?;

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

    *current_flow = get_flow_by_id(&nextflow, &csmldata.bot.flows)?;
    data.context.flow = nextflow;
    data.context.step = nextstep;

    update_conversation(
        &data,
        Some(current_flow.id.clone()),
        Some(data.context.step.clone()),
    )?;

    *interaction_order += 1;
    create_node(data)?;
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
    create_node(data)?;

    Ok(false)
}
