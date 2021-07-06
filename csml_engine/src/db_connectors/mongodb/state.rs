use crate::{
    encrypt::{decrypt_data, encrypt_data},
    EngineError, MongoDbClient,
};
use bson::{doc, Bson, Document};
use csml_interpreter::data::Client;

pub fn format_state_data(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<Vec<Document>, EngineError> {
    let client = bson::to_bson(client)?;

    keys_values.iter().fold(Ok(vec![]), |vec, (key, value)| {
        let time = Bson::DateTime(chrono::Utc::now());

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
    db: &MongoDbClient,
) -> Result<(), EngineError> {
    let state = db.client.collection("state");

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
    db: &MongoDbClient,
) -> Result<Option<serde_json::Value>, EngineError> {
    let state = db.client.collection("state");

    let filter = doc! {
        "client": bson::to_bson(client)?,
        "type": _type,
        "key": key,
    };

    match state.find_one(filter, None)? {
        Some(value) => {
            let state: serde_json::Value = bson::from_bson(bson::Bson::Document(value))?;
            let val = state["value"].as_str().unwrap().to_owned();
            Ok(Some(decrypt_data(val)?))
        }
        None => Ok(None),
    }
}

pub fn get_current_state(
    client: &Client,
    db: &MongoDbClient,
) -> Result<Option<serde_json::Value>, EngineError> {
    let state = db.client.collection("state");

    let filter = doc! {
        "client": bson::to_bson(client)?,
        "type": "hold",
        "key": "position",
    };

    match state.find_one(filter, None)? {
        Some(doc) => {
            let state: serde_json::Value = bson::from_bson(bson::Bson::Document(doc))?;
            let value = state["value"].as_str().unwrap().to_owned();

            let current_state = serde_json::json!({
                "client": state["client"],
                "type": state["type"],
                "value": decrypt_data(value)?,
                "created_at": state["created_at"],
            });

            Ok(Some(current_state))
        }
        None => Ok(None),
    }
}

pub fn set_state_items(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
    db: &MongoDbClient,
) -> Result<(), EngineError> {
    if keys_values.len() == 0 {
        return Ok(());
    }

    let state_data = format_state_data(client, _type, keys_values)?;
    let state = db.client.collection("state");
    state.insert_many(state_data, None)?;

    Ok(())
}

pub fn delete_user_state(client: &Client, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection("state");

    let filter = doc! {
        "client": bson::to_bson(&client)?,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}
