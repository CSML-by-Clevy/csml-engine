use crate::{db_interactions::Conversation, Client, ManagerError};
use dynamodb::{
    apis::client::APIClient,
    models::{
        conversation_model::Status as C_Status, inline_object, inline_object_1::Status,
        ConversationModel, CreateConversationBody, InlineObject, InlineObject1,
        UpdateConversationBody,
    },
};
use serde_json::Value;
use uuid::Uuid;

fn get_status(status: &str) -> InlineObject1 {
    let value = match status {
        open if open == "OPEN" => Status::OPEN,
        close if close == "CLOSED" => Status::CLOSED,
        failed if failed == "FAILED" => Status::FAILED,
        expired if expired == "EXPIRED" => Status::EXPIRED,
        switched if switched == "SWITCHED" => Status::SWITCHED,
        _ => unreachable!(),
    };

    InlineObject1 { status: value }
}

fn status_to_str(status: &C_Status) -> String {
    match status {
        C_Status::OPEN => "OPEN".to_owned(),
        C_Status::CLOSED => "CLOSED".to_owned(),
        C_Status::FAILED => "FAILED".to_owned(),
        C_Status::EXPIRED => "EXPIRED".to_owned(),
        C_Status::SWITCHED => "SWITCHED".to_owned(),
    }
}

fn format_conversation_struct(model: ConversationModel) -> Result<Conversation, ManagerError> {
    let client = Client {
        bot_id: model.client.bot_id,
        channel_id: model.client.channel_id,
        user_id: model.client.user_id,
    };

    Ok(Conversation {
        id: model.id,
        client,
        flow_id: model.flow_id.unwrap(),
        step_id: model.step_id.unwrap(),
        metadata: model.metadata,
        status: status_to_str(&model.status),
        last_interaction_at: model.last_interaction_at,
        updated_at: model.updated_at.unwrap_or("".to_owned()),
        created_at: model.created_at.unwrap_or("".to_owned()),
    })
}

pub fn create_conversation(
    flow_id: &str,
    step_id: &str,
    client: &Client,
    metadata: Value,
    api_client: &APIClient,
) -> Result<String, ManagerError> {
    let ccb = CreateConversationBody::new(
        Uuid::new_v4().to_string(),
        metadata,
        Some(flow_id.to_owned()),
        Some(step_id.to_owned()),
    );
    let conversation_model = api_client.conversations_api().create_conversation(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        ccb,
    )?;

    Ok(conversation_model.id)
}

pub fn close_conversation(
    id: &String,
    client: &Client,
    status: &str,
    api_client: &APIClient,
) -> Result<(), ManagerError> {
    let status = get_status(status);

    api_client.conversations_api().close_conversation(
        &id,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        status,
    )?;

    Ok(())
}

pub fn close_all_conversations(
    client: &Client,
    api_client: &APIClient,
) -> Result<(), ManagerError> {
    api_client.conversations_api().close_all_conversations(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        InlineObject::new(inline_object::Status::CLOSED),
    )?;

    Ok(())
}

pub fn get_latest_open(
    client: &Client,
    api_client: &APIClient,
) -> Result<Option<Conversation>, ManagerError> {
    let object200 = api_client.conversations_api().get_latest_open(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )?;

    match object200.conversation {
        Some(conversation) => Ok(Some(format_conversation_struct(conversation)?)),
        None => Ok(None),
    }
}

pub fn update_conversation(
    conversation_id: &str,
    client: &Client,
    flow_id: Option<String>,
    step_id: Option<String>,
    api_client: &APIClient,
) -> Result<(), ManagerError> {
    let mut conv = UpdateConversationBody::new();
    conv.flow_id = flow_id;
    conv.step_id = step_id;
    conv.last_interaction = Some(false); // tmp

    api_client.conversations_api().update_conversation(
        &conversation_id,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        conv,
    )?;

    Ok(())
}
