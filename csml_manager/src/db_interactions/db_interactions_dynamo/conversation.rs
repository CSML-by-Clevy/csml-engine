use crate::{Client, ConversationInfo};
use dynamodb::{
    apis::{client::APIClient, Error},
    models::{
        inline_object::Status,
        // inline_object_1::Status,
        ConversationModel,
        CreateConversationBody,
        InlineObject,
        InlineObject1,
        InlineResponse200,
        UpdateConversationBody,
    },
};
use serde_json::Value;
// use tokio_core::reactor::Core;
use uuid::Uuid;

// pub fn create_conversation(
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     create_conversation_body: crate::models::CreateConversationBody
// )
pub fn create_conversation(
    api_client: &APIClient,
    client: &Client,
    metadata: Value,
) -> Result<ConversationModel, Error> {
    let ccb = CreateConversationBody::new(Uuid::new_v4().to_string(), metadata);
    api_client.conversations_api().create_conversation(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        ccb,
    )
}

// pub fn close_conversation(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     inline_object1: crate::models::InlineObject1
// )
pub fn close_conversation(data: &mut ConversationInfo, status: InlineObject1) -> Result<(), Error> {
    data.api_client.conversations_api().close_conversation(
        &data.conversation_id,
        &data.client.bot_id,
        &data.client.user_id,
        &data.client.channel_id,
        status,
    )
}

// pub fn close_all_conversations(
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     inline_object: crate::models::InlineObject
// )
pub fn close_all_conversations(
    bot_id: &str,
    user_id: &str,
    channel_id: &str,
    api_client: &APIClient,
) -> Result<(), Error> {
    api_client.conversations_api().close_all_conversations(
        bot_id,
        user_id,
        channel_id,
        InlineObject::new(Status::CLOSED),
    )
}

// pub fn get_latest_open(
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str
// )
pub fn get_latest_open(
    api_client: &APIClient,
    client: &Client,
) -> Result<InlineResponse200, Error> {
    api_client.conversations_api().get_latest_open(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )
}

// pub fn get_conversation(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str
// )

// pub fn get_conversation(core: &mut Core, api_client: &APIClient) {
//     let future = api_client.conversations_api().get_conversation("b541da74-dfdc-4ff0-891c-fe5747e87460", "alexis", "1", "1");

//     println!("{:?}", core.run(future).unwrap());
// }

// pub fn update_conversation(
//     conversation_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     update_conversation_body: crate::models::UpdateConversationBody
// )
pub fn update_conversation(
    api_client: &APIClient,
    client: &Client,
    conversation_id: &str,
    flow_id: Option<String>,
    step_id: Option<String>,
    last_interaction: Option<bool>,
) -> Result<(), Error> {
    let mut conv = UpdateConversationBody::new();
    conv.flow_id = flow_id;
    conv.step_id = step_id;
    conv.last_interaction = last_interaction;

    api_client.conversations_api().update_conversation(
        &conversation_id,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        conv,
    )
}
