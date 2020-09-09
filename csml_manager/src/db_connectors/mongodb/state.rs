use crate::{
    encrypt::{decrypt_data, encrypt_data},
    ManagerError,
};
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
    // let find_options = mongodb::options::FindOneOptions::builder().sort(doc! { "$natural": -1 }).build();
    let _doc_find = state.delete_one(filter, None)?;

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
    _client: &Client,
    state_data: Vec<Document>,
    db: &mongodb::Database
) -> Result<(), ManagerError> {

    if state_data.len() == 0 {
        return Ok(())
    }

    let state = db.collection("state");
    state.insert_many(state_data, None)?;

    Ok(())
}
