use crate::db_connectors::{conversations::*, interactions::*, memories::*};
use crate::{
    data::{ConversationInfo, CsmlRequest, Database, EngineError},
    utils::{get_default_flow, get_flow_by_id, search_flow},
    Context, CsmlBot, CsmlFlow, CsmlResult,
};

use csml_interpreter::{
    data::{
        context::{get_hashmap_from_json, get_hashmap_from_mem},
        ApiInfo, Client, Event,
    },
    load_components, validate_bot,
};
use curl::{
    easy::{Easy, List},
    Error as CurlError,
};
use std::collections::HashMap;

/**
 * Initialize a new ConversationInfo data, usually upon new chat request.
 * This will contain meaningful information about the request being processsed
 * and get regularly updated as the request progresses.
 *
 * This will hold references to:
 * - the bot's data,
 * - the current status of the request (steps, messages, variables, context...)
 * - the DB to use for data persistence
 * - the cached Curl connexion to the configured callback_url, if any
 *
 * This method takes care of the initialization of the data as well as setting up
 * some information in the database (conversation_id, metadata, state...).
 */
pub fn init_conversation_info<'a>(
    default_flow: String,
    event: &Event,
    request: &'a CsmlRequest,
    bot: &'a CsmlBot,
    mut db: Database,
) -> Result<ConversationInfo, EngineError> {
    // Create a new interaction. An interaction is basically each request,
    // initiated from the bot or the user.
    let interaction_id = init_interaction(request.payload.clone(), &request.client, &mut db)?;
    let mut context = init_context(default_flow, request.client.clone(), &bot.fn_endpoint);

    // Create and cache a curl agent to call the callback_url for every new message.
    // If no callback_url is set, no message will be sent as they are processed and
    // they will only be returned at the end of the fully-processed and successful request.
    let curl = match request.callback_url {
        Some(ref url) => {
            if let Ok(curl) = init_curl(url) {
                Some(curl)
            } else {
                return Err(EngineError::Manager(format!(
                    "not valid callback_url {}",
                    url
                )));
            }
        }
        None => None,
    };

    // Do we have a flow matching the request? If the user is requesting a flow in one way
    // or another, this takes precedence over any previously open conversation
    // and a new conversation is created with the new flow as a starting point.
    let flow_found = search_flow(event, &bot, &request.client, &mut db).ok();
    let conversation_id = get_or_create_conversation(
        &mut context,
        &bot,
        flow_found,
        &request.client,
        &mut db,
    )?;

    context.metadata = get_hashmap_from_json(&request.metadata, &context.flow);
    context.current = get_hashmap_from_mem(&internal_use_get_memories(&request.client, &mut db)?, &context.flow);

    let mut data = ConversationInfo {
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

    let flow = data.context.flow.to_owned();
    let step = data.context.step.to_owned();

    // Now that everything is correctly setup, update the conversation with wherever
    // we are now and continue with the rest of the request!
    println!("init.rs line=96 {:?}", data.context);
    update_conversation(&mut data, Some(flow), Some(step))?;

    Ok(data)
}

/**
 * Initialize the bot
 */
pub fn init_bot(bot: &mut CsmlBot) -> Result<(), EngineError> {
    // load native components into the bot
    bot.native_components = match load_components() {
        Ok(components) => Some(components),
        Err(err) => return Err(EngineError::Interpreter(err.format_error())),
    };

    match validate_bot(&bot) {
        CsmlResult {
            flows: Some(flows),
            errors: None,
            ..
        } => {
            bot.bot_ast = Some(base64::encode(bincode::serialize(&flows).unwrap()));
        }
        CsmlResult {
            errors: Some(errors),
            ..
        } => {
            return Err(EngineError::Interpreter(format!(
                "invalid bot {:?}",
                errors
            )))
        }
        _ => return Err(EngineError::Interpreter(format!("empty bot"))),
    }

    Ok(())
}

/**
 * Initialize the context object for incoming requests
 */
pub fn init_context(flow: String, client: Client, fn_endpoint: &Option<String>) -> Context {
    let api_info = match fn_endpoint {
        Some(value) => Some(ApiInfo {
            client,
            fn_endpoint: value.to_owned(),
        }),
        None => None,
    };

    Context {
        current: HashMap::new(),
        metadata: HashMap::new(),
        api_info,
        hold: None,
        step: "start".to_owned(),
        flow,
    }
}

/**
 * Initialize a curl agent for standardized post requests to a given url.
 * It should be cached whenever possible to reuse existing connections.
 */
pub fn init_curl(url: &str) -> Result<Easy, CurlError> {
    let mut easy = Easy::new();
    let mut list = List::new();
    easy.url(url)?;
    easy.post(true)?;

    list.append("Accept: application/json")?;
    list.append("Content-Type: application/json")?;
    easy.http_headers(list)?;
    Ok(easy)
}

/**
 * Retrieve the current conversation, or create one if none exists.
 */
fn get_or_create_conversation<'a>(
    context: &mut Context,
    bot: &'a CsmlBot,
    flow_found: Option<&'a CsmlFlow>,
    client: &Client,
    db: &mut Database,
) -> Result<String, EngineError> {
    match get_latest_open(client, db)? {
        Some(conversation) => {
            match flow_found {
                Some(flow) => {
                    context.step = "start".to_owned();
                    context.flow = flow.name.to_owned();
                }
                None => {
                    let flow = match get_flow_by_id(&conversation.flow_id, &bot.flows) {
                        Ok(flow) => flow,
                        Err(..) => {
                            // if flow id exist in db but not in bot close conversation
                            close_conversation(&conversation.id, &client, db)?;
                            // start new conversation at default flow
                            return create_new_conversation(
                                context, bot, flow_found, client, db,
                            );
                        }
                    };

                    context.step = conversation.step_id.to_owned();
                    context.flow = flow.name.to_owned();
                }
            };

            Ok(conversation.id)
        }
        None => create_new_conversation(context, bot, flow_found, client, db),
    }
}

/**
 * Create and save a new conversation in DB
 */
fn create_new_conversation<'a>(
    context: &mut Context,
    bot: &'a CsmlBot,
    flow_found: Option<&'a CsmlFlow>,
    client: &Client,
    db: &mut Database,
) -> Result<String, EngineError> {
    let flow = match flow_found {
        Some(flow) => flow,
        None => get_default_flow(bot)?,
    };
    context.step = "start".to_owned();
    context.flow = flow.name.to_owned();

    let conversation_id =
        create_conversation(&flow.id, &context.step, client, db)?;

    Ok(conversation_id)
}
