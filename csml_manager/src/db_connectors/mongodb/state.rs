use crate::{
    encrypt::{decrypt_data, encrypt_data},
    ManagerError, ConversationInfo,
};
use crate::db_connectors::mongodb as mongodb_connector;
use bson::{doc, Bson, Document};
use csml_interpreter::data::Client;

pub fn format_state_data(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<Vec<Document>, ManagerError> {
    let client = bson::to_bson(client)?;

    keys_values.iter().fold(Ok(vec![]), |vec, (key, value)| {
        let time = Bson::UtcDatetime(chrono::Utc::now());

        let value = encrypt_data(value)?;
        let mut vec = vec?;

        vec.push(doc! {
            "client": client.clone(),
            "type": _type,
            "key": key,
            "value": value,
            "expires_at": Bson::Null,
            "created_at": time
        });
        Ok(vec)
    })
}

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &mongodb::Database,
) -> Result<(), ManagerError> {
    let state = db.collection("state");

    let filter = doc! {
        "client": bson::to_bson(client)?,
        "type": _type,
        "key": key,
    };
    state.delete_one(filter, None)?;

    Ok(())
}

pub fn get_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &mongodb::Database,
) -> Result<Option<serde_json::Value>, ManagerError> {
    let state = db.collection("state");

    let filter = doc! {
        "client": bson::to_bson(client)?,
        "type": _type,
        "key": key,
    };
    match state.find_one(filter, None)? {
        Some(value) => {
            let state: serde_json::Value = bson::from_bson(bson::Bson::Document(value))?;

            Ok(Some(decrypt_data(
                state["value"].as_str().unwrap().to_owned(),
            )?))
        }
        None => Ok(None),
    }
}

pub fn set_state_items(
    data: &ConversationInfo,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {

    if keys_values.len() == 0 {
        return Ok(())
    }

    let state_data = format_state_data(&data.client, _type, keys_values)?;
    let db = mongodb_connector::get_db(&data.db)?;
    let state = db.collection("state");
    state.insert_many(state_data, None)?;

    Ok(())
}
