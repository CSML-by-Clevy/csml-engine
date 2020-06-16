use uuid::Uuid;

use crate::{Client, ManagerError};
use dynamodb::{
    apis::client::APIClient,
    models::{CreateInteractionBody, InlineObject2},
};

pub fn init_interaction(
    event: serde_json::Value,
    client: &Client,
    api_client: &APIClient,
) -> Result<String, ManagerError> {
    let inter = CreateInteractionBody::new(Uuid::new_v4().to_string(), event);
    let interaction = api_client.interactions_api().init_interaction(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        inter,
    )?;

    Ok(interaction.id)
}

pub fn update_interaction(
    interaction_id: &str,
    success: bool,
    client: &Client,
    api_client: &APIClient,
) -> Result<(), ManagerError> {
    let inl2 = InlineObject2::new(success);
    api_client.interactions_api().update_interaction(
        &interaction_id,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        inl2,
    )?;

    Ok(())
}
