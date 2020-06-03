use crate::{
    encrypt::{decrypt_data, encrypt_data},
    ConversationInfo, ManagerError,
};
use bson::{doc, Bson, Document};
use csmlinterpreter::data::Client;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct State {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: bson::oid::ObjectId,
    pub client: Client,
    #[serde(rename = "type")]
    pub _type: String,
    pub value: String, // encrypted
    pub expires_at: Option<bson::UtcDateTime>,
    pub created_at: bson::UtcDateTime,
}

pub fn format_state_body(
    data: &mut ConversationInfo,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<Vec<Document>, ManagerError> {
    let client = bson::to_bson(&data.client)?;

    let value = keys_values.iter().fold(Ok(vec![]), |vec, (key, value)| {
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
    });

    value
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

// pub fn get_state_type(
//     client: &Client,
//     _type: &str,
//     db: &mongodb::Database,
// ) -> Result<mongodb::Cursor, Error> {
//     let state = db.collection("state");

//     let filter = doc! {
//         "client": bson::to_bson(client)?,
//         "type": _type,
//     };
//     let cursor = state.find(filter, None)?;

//     Ok(cursor)
// }

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
            let state: State = bson::from_bson(bson::Bson::Document(value))?;

            Ok(Some(decrypt_data(state.value)?))
        }
        None => Ok(None),
    }
}

pub fn set_state_items(data: &ConversationInfo, docs: Vec<Document>) -> Result<(), ManagerError> {
    let state = data.db.collection("state");

    state.insert_many(docs, None)?;

    Ok(())
}
