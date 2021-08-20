use chrono::SecondsFormat;

use diesel::{RunQueryDsl, ExpressionMethods, QueryDsl};
use diesel::{insert_into};

use serde_json::Value;

use crate::{
    db_connectors::postgresql::get_db,
    encrypt::{decrypt_data, encrypt_data},
    EngineError, PostgresqlClient,
    ConversationInfo, Memory, Client
};

use super::{
    models,
    schema::states
};

use std::collections::HashMap;
use std::env;

pub fn delete_state_key(
    client: &Client,
    type_: &str,
    key: &str,
    db: &PostgresqlClient,
) -> Result<(), EngineError> {
    diesel::delete(states::table
        .filter(states::client_id.eq(42))
        // .filter(states::type_.eq(type_))
        // .filter(states::key.eq(key))
    ).execute(&db.client);

    Ok(())
}

pub fn get_state_key(
    client: &Client,
    type_: &str,
    key: &str,
    db: &PostgresqlClient,
) -> Result<Option<serde_json::Value>, EngineError> {
    let state: models::State = states::table
    .filter(states::client_id.eq(42))
    // .filter(states::bot_id.eq("Sean"))
    // .filter(states::channel_id.eq("Sean"))
    // .filter(states::user_id.eq("Sean"))
    .filter(states::type_.eq("hold"))
    .filter(states::key.eq("position"))
    .limit(1)
    .get_result(&db.client)
    .expect("Error getting memory"); 

    Ok(Some(decrypt_data(state.value)?))
}

pub fn get_current_state(
    client: &Client,
    db: &PostgresqlClient,
) -> Result<Option<serde_json::Value>, EngineError> {

    let current_state: models::State = states::table.filter(states::client_id.eq(42))
        // .filter(states::bot_id.eq("Sean"))
        // .filter(states::channel_id.eq("Sean"))
        // .filter(states::user_id.eq("Sean"))

        .filter(states::type_.eq("hold"))
        .filter(states::key.eq("position"))
        .limit(1)
        .get_result(&db.client)
        .expect("Error getting states"); 


    let current_state = serde_json::json!({
        "client": current_state.client_id,
        "type": current_state.type_,
        "value": decrypt_data(current_state.value)?,
        "created_at": current_state.created_at.to_string(),
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
            client_id: 42,
            type_,
            key,
            value,
        };

        new_states.push(mem);
    }

    diesel::insert_into(states::table)
    .values(&new_states)
    .execute(&db.client)
    .expect("Error creating memory");

    Ok(())
}

pub fn delete_user_state(
    client: &Client,
    db: &PostgresqlClient
) -> Result<(), EngineError> {
    diesel::delete(states::table
        .filter(states::client_id.eq(42))
    ).execute(&db.client);

    Ok(())
}
