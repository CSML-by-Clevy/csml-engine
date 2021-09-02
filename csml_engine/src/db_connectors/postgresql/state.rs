use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};

use crate::{
    encrypt::{decrypt_data, encrypt_data},
    EngineError, PostgresqlClient,
    Client
};

use super::{
    models,
    schema::csml_states
};

pub fn delete_state_key(
    client: &Client,
    type_: &str,
    key: &str,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::delete(csml_states::table
        .filter(csml_states::bot_id.eq(&client.bot_id))
        .filter(csml_states::channel_id.eq(&client.channel_id))
        .filter(csml_states::user_id.eq(&client.user_id))
        .filter(csml_states::type_.eq(type_))
        .filter(csml_states::key.eq(key))
    ).execute(&db.client)?;

    Ok(())
}

pub fn get_state_key(
    client: &Client,
    type_: &str,
    key: &str,
    db: &PostgresqlClient,
) -> Result<Option<serde_json::Value>, EngineError> {
    let state: Result<models::State, diesel::result::Error> = csml_states::table
    .filter(csml_states::bot_id.eq(&client.bot_id))
    .filter(csml_states::channel_id.eq(&client.channel_id))
    .filter(csml_states::user_id.eq(&client.user_id))

    .filter(csml_states::type_.eq(type_))
    .filter(csml_states::key.eq(key))

    .get_result(&db.client);

    match state {
        Ok(state) => {
            let value = decrypt_data(state.value)?;
            Ok(Some(value))
        },
        Err(_err) => {
            Ok(None)
        }
    }
}

pub fn get_current_state(
    client: &Client,
    db: &PostgresqlClient,
) -> Result<Option<serde_json::Value>, EngineError> {

    let current_state: models::State = csml_states::table
        .filter(csml_states::bot_id.eq(&client.bot_id))
        .filter(csml_states::channel_id.eq(&client.channel_id))
        .filter(csml_states::user_id.eq(&client.user_id))

        .filter(csml_states::type_.eq("hold"))
        .filter(csml_states::key.eq("position"))

        .get_result(&db.client)?;

    let current_state = serde_json::json!({
        "client": {
            "bot_id": current_state.bot_id,
            "channel_id": current_state.channel_id,
            "user_id": current_state.user_id
        },
        "type": current_state.type_,
        "value": decrypt_data(current_state.value)?,
        "created_at": current_state.created_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
    });

    Ok(Some(current_state))
}

pub fn set_state_items(
    client: &Client,
    type_: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {
    if keys_values.len() == 0 {
        return Ok(());
    }

    let mut new_states = vec!();
    for (key, value) in keys_values.iter() {

        let value = encrypt_data(value)?;

        let mem = models::NewState {
            id: uuid::Uuid::new_v4(),

            bot_id: &client.bot_id,
            channel_id: &client.channel_id,
            user_id: &client.user_id,
            type_,
            key,
            value,
        };

        new_states.push(mem);
    }

    diesel::insert_into(csml_states::table)
    .values(&new_states)
    .execute(&db.client)?;

    Ok(())
}

pub fn delete_user_state(
    client: &Client,
    db: &PostgresqlClient
) -> Result<(), EngineError> {
    diesel::delete(csml_states::table
        .filter(csml_states::bot_id.eq(&client.bot_id))
        .filter(csml_states::channel_id.eq(&client.channel_id))
        .filter(csml_states::user_id.eq(&client.user_id))
    ).execute(&db.client).ok();

    Ok(())
}

pub fn delete_all_bot_data(
    bot_id: &str,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::delete(
        csml_states::table
        .filter(csml_states::bot_id.eq(bot_id))
    ).execute(&db.client).ok();

    Ok(())
}