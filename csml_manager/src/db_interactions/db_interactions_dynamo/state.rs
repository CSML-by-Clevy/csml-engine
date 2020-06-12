use crate::{ConversationInfo, Memories};
use csmlinterpreter::data::Client;
use dynamodb::{
    apis::{client::APIClient, Error},
    models::{CreateStateBody, StateModel},
};

pub fn format_memories(
    data: &mut ConversationInfo,
    memories: &[Memories],
    interaction_order: i32,
) -> Vec<CreateStateBody> {
    let vec = memories
        .iter()
        .fold(vec![], |mut vec: Vec<(&str, &serde_json::Value)>, var| {
            vec.push((&var.key, &var.value));
            vec
        });
    format_state_body(data, "remember", interaction_order, vec)
}

pub fn format_state_body(
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
                data.flow_info.flow.id.to_owned(),
                data.flow_info.step_id.to_owned(),
                _type.to_owned(),
                key.to_string(),
                (*value).to_owned(),
                true,
                None,
            ));
            vec
        })
}

// pub fn delete_state_full(api_client: &APIClient, client: &Client) -> Result<(), Error> {
//     api_client
//         .state_api()
//         .delete_state_full(&client.bot_id, &client.user_id, &client.channel_id)
// }

// pub fn delete_state_type(api_client: &APIClient, client: &Client, _type: &str) -> Result<(), Error> {
//     api_client
//     .state_api()
//     .delete_state_type(_type, &client.bot_id, &client.user_id, &client.channel_id)
// }

pub fn delete_state_key(
    api_client: &APIClient,
    client: &Client,
    _type: &str,
    key: &str,
) -> Result<(), Error> {
    api_client.state_api().delete_state_key(
        _type,
        key,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )
}

// pub fn get_state_full(api_client: &APIClient, client: &Client) -> Result<Vec<StateModel>, Error> {
//     api_client
//     .state_api()
//     .get_state_full(&client.bot_id, &client.user_id, &client.channel_id)
// }

pub fn get_state_type(
    api_client: &APIClient,
    client: &Client,
    _type: &str,
) -> Result<Vec<StateModel>, Error> {
    api_client.state_api().get_state_type(
        _type,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )
}

pub fn get_state_key(
    api_client: &APIClient,
    client: &Client,
    _type: &str,
    key: &str,
) -> Result<StateModel, Error> {
    api_client.state_api().get_state_key(
        _type,
        key,
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
    )
}

pub fn set_state_items(
    api_client: &APIClient,
    client: &Client,
    create_state_body: Vec<CreateStateBody>,
) -> Result<(), Error> {
    api_client.state_api().set_state_items(
        &client.bot_id,
        &client.user_id,
        &client.channel_id,
        create_state_body,
    )
}
