use crate::db_interactions::{conversation::*, interactions::*, memories::*};
use crate::{
    data::{ConversationInfo, CsmlData, ManagerError},
    tools::{get_default_flow, get_flow_by_id, search_flow},
    ContextJson, CsmlBot, CsmlFlow,
};

use csmlinterpreter::data::{ApiInfo, Client, Event};
use curl::{
    easy::{Easy, List},
    Error as CurlError,
};
use std::env;

pub fn init_db() -> Result<mongodb::Database, ManagerError> {
    let uri = match env::var("MONGODB_URI") {
        Ok(var) => var,
        _ => panic!("error no MONGODB_URI en env"),
    };

    let client = mongodb::Client::with_uri_str(&uri)?;

    Ok(client.database("csml"))
}

pub fn init_conversation_info<'a>(
    default_flow: String,
    event: &Event,
    csmldata: &'a CsmlData,
) -> Result<ConversationInfo, ManagerError> {
    let db = init_db()?;

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

pub fn init_context(flow: String, client: Client, fn_endpoint: &Option<String>) -> ContextJson {
    let api_info = match fn_endpoint {
        Some(value) => Some(ApiInfo {
            client,
            fn_endpoint: value.to_owned(),
        }),
        None => None,
    };

    ContextJson {
        current: serde_json::json!({}),
        metadata: serde_json::json!({}),
        api_info,
        hold: None,
        step: "start".to_owned(),
        flow,
    }
}

pub fn init_curl(callback_url: &str) -> Result<Easy, CurlError> {
    let mut easy = Easy::new();
    let mut list = List::new();
    easy.url(callback_url)?;
    easy.post(true)?;

    list.append("Accept: application/json")?;
    list.append("Content-Type: application/json")?;
    easy.http_headers(list)?;
    Ok(easy)
}

fn get_conversation<'a>(
    context: &mut ContextJson,
    bot: &'a CsmlBot,
    flow_found: Option<&'a CsmlFlow>,
    metadata: serde_json::Value,
    client: &Client,
    db: &mongodb::Database,
) -> Result<bson::Bson, ManagerError> {
    match get_latest_open(client, db)? {
        Some(conversation) => {
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
                        Err(..) => {
                            // if flow id exist in db but not in bot close conversation
                            close_conversation(
                                &bson::Bson::ObjectId(conversation.id),
                                &client,
                                &db,
                            )?;
                            // and start new conversation at default flow
                            return create_new_conversation(
                                context, bot, flow_found, client, metadata, db,
                            );
                        }
                    };

                    context.step = conversation.step_id.to_owned();
                    context.flow = flow.name.to_owned();
                }
            };

            Ok(bson::Bson::ObjectId(conversation.id))
        }
        None => create_new_conversation(context, bot, flow_found, client, metadata, db),
    }
}

fn create_new_conversation<'a>(
    context: &mut ContextJson,
    bot: &'a CsmlBot,
    flow_found: Option<&'a CsmlFlow>,
    client: &Client,
    metadata: serde_json::Value,
    db: &mongodb::Database,
) -> Result<bson::Bson, ManagerError> {
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
