use crate::{ConversationInfo, ManagerError};
use csml_interpreter::data::Client;
use http_db::{
    apis::client::APIClient,
    models::CreateStateBody,
};

pub fn format_state_data(
    data: &mut ConversationInfo,
    _type: &str,
    interaction_order: i32,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Vec<CreateStateBody> {
    keys_values
        .iter()
        .enumerate()
        .fold(vec![], |mut vec, (mem_order, (key, value))| {
            let id = uuid::Uuid::new_v4().to_string();
            vec.push(CreateStateBody::new(
                id,
                data.interaction_id.clone(),
                data.conversation_id.clone(),
                mem_order as i32,
                interaction_order,
                data.context.flow.to_owned(),
                data.context.step.to_owned(),
                _type.to_owned(),
                key.to_string(),
                (*value).to_owned(),
                true,
                None,
            ));
            vec
        })
}

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    api_client: &APIClient,
) -> Result<(), ManagerError> {
    api_client.state_api().delete_state_key(
        _type,
        key,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )?;

    Ok(())
}

pub fn get_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    api_client: &APIClient,
) -> Result<Option<serde_json::Value>, ManagerError> {
    let state = api_client.state_api().get_state_key(
        _type,
        key,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )?;

    Ok(Some(state.value))
}

pub fn set_state_items(
    client: &Client,
    state_data: Vec<CreateStateBody>,
    db: &APIClient,
) -> Result<(), ManagerError> {
    db.state_api().set_state_items(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        state_data,
    )?;

    Ok(())
}
