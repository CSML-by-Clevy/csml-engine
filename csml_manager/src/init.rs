use crate::db_interactions::{conversation::*, init_db, interactions::*, memories::*};
use crate::{
    data::{CsmlRequest, ConversationInfo, Database, ManagerError},
    tools::{get_default_flow, get_flow_by_id, search_flow},
    ContextJson, CsmlBot, CsmlFlow,
};

use csml_interpreter::data::{ApiInfo, Client, Event};
use curl::{
    easy::{Easy, List},
    Error as CurlError,
};

pub fn init_conversation_info<'a>(
    default_flow: String,
    event: &Event,
    request: &'a CsmlRequest,
    bot: &'a CsmlBot,
) -> Result<ConversationInfo, ManagerError> {
    let db = init_db()?;

    let interaction_id = init_interaction(request.payload.clone(), &request.client, &db)?;
    let mut context = init_context(
        default_flow,
        request.client.clone(),
        &bot.fn_endpoint,
    );

    let curl = match request.callback_url {
        Some(ref url) => {
            if let Ok(curl) = init_curl(url) {
                Some(curl)
            } else {
                return Err(ManagerError::Manager(format!(
                    "not valid callback_url {}",
                    url
                )));
            }
        }
        None => None,
    };

    let flow_found = search_flow(event, &bot, &request.client, &db).ok();
    let conversation_id = get_conversation(
        &mut context,
        &bot,
        flow_found,
        event.metadata.clone(),
        &request.client,
        &db,
    )?;

    get_memories(
        &request.client,
        // &conversation_id,
        &mut context,
        &request.metadata,
        &db,
    )?;

    let data = ConversationInfo {
        conversation_id,
        interaction_id,
        context,
        metadata: request.metadata.clone(), // ??
        request_id: request.request_id.clone(),
        curl,
        client: request.client.clone(),
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
    db: &Database,
) -> Result<String, ManagerError> {
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
                            close_conversation(&conversation.id, &client, &db)?;
                            // start new conversation at default flow
                            return create_new_conversation(
                                context, bot, flow_found, client, metadata, db,
                            );
                        }
                    };

                    context.step = conversation.step_id.to_owned();
                    context.flow = flow.name.to_owned();
                }
            };

            Ok(conversation.id)
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
    db: &Database,
) -> Result<String, ManagerError> {
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
