use crate::db_connectors::{conversations::*, memories::*, state};
use crate::interpreter_actions::SwitchBot;
use crate::{
    data::{ConversationInfo, CsmlRequest, Database, EngineError},
    utils::{
        get_default_flow, get_flow_by_id, get_low_data_mode_value, get_ttl_duration_value,
        search_flow, send_msg_to_callback_url,
    },
    BotOpt, Context, CsmlBot, CsmlFlow, CsmlResult,
};

use csml_interpreter::data::context::ContextStepInfo;
use csml_interpreter::{
    data::{
        ast::Flow,
        context::{get_hashmap_from_json, get_hashmap_from_mem},
        ApiInfo, Client, Event, Message, PreviousBot,
    },
    load_components, search_for_modules, validate_bot,
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
pub fn init_conversation_info<'a, 'b>(
    default_flow: String,
    event: &Event,
    request: &'a CsmlRequest,
    bot: &'a CsmlBot,
    mut db: Database<'b>,
) -> Result<ConversationInfo<'b>, EngineError> {
    // Create a new interaction. An interaction is basically each request,
    // initiated from the bot or the user.

    let mut context = init_context(
        default_flow,
        request.client.clone(),
        &bot.apps_endpoint,
        &mut db,
    );
    let ttl = get_ttl_duration_value(Some(event));
    let low_data = get_low_data_mode_value(event);

    // Do we have a flow matching the request? If the user is requesting a flow in one way
    // or another, this takes precedence over any previously open conversation
    // and a new conversation is created with the new flow as a starting point.
    let flow_found = search_flow(event, bot, &request.client, &mut db).ok();
    let conversation_id = get_or_create_conversation(
        &mut context,
        bot,
        flow_found,
        &request.client,
        ttl,
        &mut db,
    )?;

    context.metadata = get_hashmap_from_json(&request.metadata, &context.flow);
    context.current = get_hashmap_from_mem(
        &internal_use_get_memories(&request.client, &mut db)?,
        &context.flow,
    );

    let mut data = ConversationInfo {
        conversation_id,
        context,
        metadata: request.metadata.clone(), // ??
        request_id: request.request_id.clone(),
        callback_url: request.callback_url.clone(),
        client: request.client.clone(),
        messages: vec![],
        ttl,
        low_data,
        db,
    };

    let flow = data.context.flow.to_owned();
    let step = data.context.step.to_owned();

    // Now that everything is correctly setup, update the conversation with wherever
    // we are now and continue with the rest of the request!
    update_conversation(&mut data, Some(flow), Some(step.get_step()))?;

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

    if let Err(err) = search_for_modules(bot) {
        return Err(EngineError::Interpreter(format!("{:?}", err)));
    }

    set_bot_ast(bot)
}

/**
 * Initialize bot ast
 */
fn set_bot_ast(bot: &mut CsmlBot) -> Result<(), EngineError> {
    match validate_bot(bot) {
        CsmlResult {
            flows: Some(flows),
            extern_flows: Some(extern_flows),
            errors: None,
            ..
        } => {
            bot.bot_ast = Some(base64::encode(
                bincode::serialize(&(&flows, &extern_flows)).unwrap(),
            ));
        }
        CsmlResult {
            flows: Some(flows),
            extern_flows: None,
            errors: None,
            ..
        } => {
            let extern_flows: HashMap<String, Flow> = HashMap::new();

            bot.bot_ast = Some(base64::encode(
                bincode::serialize(&(&flows, &extern_flows)).unwrap(),
            ));
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
        _ => return Err(EngineError::Interpreter("empty bot".to_string())),
    }

    Ok(())
}

/**
 * Initialize the context object for incoming requests
 */
pub fn init_context(
    flow: String,
    client: Client,
    apps_endpoint: &Option<String>,
    db: &mut Database,
) -> Context {
    let previous_bot = get_previous_bot(&client, db);

    let api_info = apps_endpoint.as_ref().map(|value| ApiInfo {
            client,
            apps_endpoint: value.to_owned(),
        });

    Context {
        current: HashMap::new(),
        metadata: HashMap::new(),
        api_info,
        hold: None,
        step: ContextStepInfo::Normal("start".to_owned()),
        flow,
        previous_bot,
    }
}

fn get_previous_bot(client: &Client, db: &mut Database) -> Option<PreviousBot> {
    match state::get_state_key(client, "bot", "previous", db) {
        Ok(Some(bot)) => serde_json::from_value(bot).ok(),
        _ => None,
    }
}

/**
 * Retrieve the current conversation, or create one if none exists.
 */
fn get_or_create_conversation<'a>(
    context: &mut Context,
    bot: &'a CsmlBot,
    flow_found: Option<(&'a CsmlFlow, String)>,
    client: &Client,
    ttl: Option<chrono::Duration>,
    db: &mut Database,
) -> Result<String, EngineError> {
    match get_latest_open(client, db)? {
        Some(conversation) => {
            match flow_found {
                Some((flow, step)) => {
                    context.step = ContextStepInfo::UnknownFlow(step);
                    context.flow = flow.name.to_owned();
                }
                None => {
                    let flow = match get_flow_by_id(&conversation.flow_id, &bot.flows) {
                        Ok(flow) => flow,
                        Err(..) => {
                            // if flow id exist in db but not in bot close conversation
                            close_conversation(&conversation.id, client, db)?;
                            // start new conversation at default flow
                            return create_new_conversation(
                                context, bot, flow_found, client, ttl, db,
                            );
                        }
                    };

                    context.step = ContextStepInfo::UnknownFlow(conversation.step_id.to_owned());
                    context.flow = flow.name.to_owned();
                }
            };

            Ok(conversation.id)
        }
        None => create_new_conversation(context, bot, flow_found, client, ttl, db),
    }
}

/**
 * Create and save a new conversation in DB
 */
fn create_new_conversation<'a>(
    context: &mut Context,
    bot: &'a CsmlBot,
    flow_found: Option<(&'a CsmlFlow, String)>,
    client: &Client,
    ttl: Option<chrono::Duration>,
    db: &mut Database,
) -> Result<String, EngineError> {
    let (flow, step) = match flow_found {
        Some((flow, step)) => (flow, step),
        None => (get_default_flow(bot)?, "start".to_owned()),
    };

    let conversation_id = create_conversation(&flow.id, &step, client, ttl, db)?;

    context.step = ContextStepInfo::UnknownFlow(step);
    context.flow = flow.name.to_owned();

    Ok(conversation_id)
}

/**
 * Switch bot find next bot in DB and create new Client and new conversation
 */
pub fn switch_bot(
    data: &mut ConversationInfo,
    bot: &mut CsmlBot,
    next_bot: SwitchBot,
    bot_opt: &mut BotOpt,
    event: &mut Event,
) -> Result<(), EngineError> {
    // update data info with new bot |ex| client bot_id, create new conversation
    *bot_opt = match next_bot.version_id {
        Some(version_id) => BotOpt::Id {
            version_id,
            bot_id: next_bot.bot_id,
            apps_endpoint: bot.apps_endpoint.take(),
            multibot: bot.multibot.take(),
        },
        None => BotOpt::BotId {
            bot_id: next_bot.bot_id,
            apps_endpoint: bot.apps_endpoint.take(),
            multibot: bot.multibot.take(),
        },
    };

    let mut new_bot = bot_opt.search_bot(&mut data.db)?;
    new_bot.custom_components = bot.custom_components.take();
    new_bot.native_components = bot.native_components.take();

    *bot = new_bot;

    set_bot_ast(bot)?;

    data.context.step = ContextStepInfo::UnknownFlow(next_bot.step);
    data.context.flow = match next_bot.flow {
        Some(flow) => flow,
        None => bot.get_default_flow_name(),
    };

    // update client with the new bot id
    data.client.bot_id = bot.id.to_owned();

    let (flow, step) = match get_flow_by_id(&data.context.flow, &bot.flows) {
        Ok(flow) => (flow, data.context.step.clone()),
        Err(_) => {
            let error_message = format!(
                "flow: [{}] not found in bot: [{}], switching to start@default_flow",
                data.context.flow, bot.name
            );

            let message = Message {
                content_type: "error".to_owned(),
                content: serde_json::json!({"error": error_message}),
            };

            // save message
            data.messages.push(message.clone());
            // send message
            send_msg_to_callback_url(data, vec![message], 0, false);

            // setting default step && flow
            data.context.step = ContextStepInfo::Normal("start".to_owned());
            data.context.flow = bot.get_default_flow_name();

            (
                get_flow_by_id(&bot.default_flow, &bot.flows)?,
                ContextStepInfo::Normal("start".to_owned()),
            )
        }
    };

    // update event to flow trigger
    event.content_type = "flow_trigger".to_owned();
    event.content = serde_json::json!({
            "flow_id": flow.id,
            "step_id": step
        }
    );

    // create new conversation for the new client
    data.conversation_id = create_conversation(
        &flow.id,
        &step.get_step(),
        &data.client,
        data.ttl,
        &mut data.db,
    )?;

    // and get memories of the new bot form db,
    // clearing the permanent memories form scope of the previous bot
    data.context.current = get_hashmap_from_mem(
        &internal_use_get_memories(&data.client, &mut data.db)?,
        &data.context.flow,
    );

    Ok(())
}
