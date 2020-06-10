use serde_json::Value;
use uuid::Uuid;

use crate::{Client, ConversationInfo};
use dynamodb::{
    apis::{client::APIClient, Error},
    models::{
        CreateInteractionBody,
        InlineObject2,
        // InlineResponse2001,
        // InlineResponse2002,
        InteractionModel,
    },
};
// use serde_json::Value;
// use tokio_core::reactor::Core;

// pub fn get_interaction(
//     interaction_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str
// )

// pub fn get_interaction(core: &mut Core, api_client: &APIClient, client: &Client, interaction_id: &str) -> Result<InteractionModel, Error<Value>> {
//     let future = api_client.interactions_api().get_interaction(interaction_id, &client.bot_id, &client.user_id, &client.channel_id);

//     core.run(future)
// }

// pub fn get_interaction_status(
//     interaction_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str
// )

// pub fn get_interaction_status(core: &mut Core, api_client: &APIClient, client: &Client, interaction_id: &str) -> Result<InlineResponse2001, Error<Value>> {
//     let future = api_client.interactions_api().get_interaction_status(interaction_id, &client.bot_id, &client.user_id, &client.channel_id);

//     core.run(future)
// }

// pub fn get_lock_status(
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str
// )

// pub fn get_lock_status(core: &mut Core, api_client: &APIClient, client: &Client) -> Result<InlineResponse2002, Error<Value>> {
//     let future = api_client.interactions_api().get_lock_status(&client.bot_id, &client.user_id, &client.channel_id);

//     core.run(future)
// }

// pub fn init_interaction(
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     create_interaction_body: crate::models::CreateInteractionBody
// )

pub fn init_interaction(
    api_client: &APIClient,
    client: &Client,
    event: Value,
) -> Result<InteractionModel, Error> {
    let inter = CreateInteractionBody::new(Uuid::new_v4().to_string(), event);
    api_client.interactions_api().init_interaction(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        inter,
    )
}

// pub fn update_interaction(
//     interaction_id: &str,
//     bot_id: &str,
//     user_id: &str,
//     channel_id: &str,
//     inline_object2: crate::models::InlineObject2
// )
pub fn update_interaction(data: &mut ConversationInfo, success: bool) -> Result<(), Error> {
    let inl2 = InlineObject2::new(success);
    data.api_client.interactions_api().update_interaction(
        &data.interaction_id,
        &data.client.bot_id,
        &data.client.user_id,
        &data.client.channel_id,
        inl2,
    )
}
