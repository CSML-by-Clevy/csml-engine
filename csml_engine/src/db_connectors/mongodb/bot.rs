use crate::{db_connectors::DbBot, encrypt::encrypt_data, Client, EngineError};
use csml_interpreter::data::ast::Flow;
use bson::{doc, Bson};
use chrono::SecondsFormat;
use std::collections::HashMap;

fn format_bot_struct(
    conversation: bson::ordered::OrderedDocument,
) -> Result<DbBot, EngineError> {
    Ok(DbBot {
        id: conversation.get_object_id("_id").unwrap().to_hex(), // to_hex bson::oid::ObjectId
        bot_id: conversation.get_str("bot_id").unwrap().to_owned(),
        build_nbr: conversation.get_i32("build_nbr").unwrap(),
        bot: conversation.get_str("bot").unwrap().to_owned(),
        ast: conversation.get_str("ast").unwrap().to_owned(),
        engine_version: conversation.get_str("engine_version").unwrap().to_owned(),
        updated_at: conversation
            .get_utc_datetime("updated_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
        created_at: conversation
            .get_utc_datetime("created_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    })
}

pub fn save_bot_state(
    bot_id: String,
    bot: String,
    ast: String,
    db: &mongodb::Database,
) -> Result<String, EngineError> {
    let collection = db.collection("ast");
    let time = Bson::UtcDatetime(chrono::Utc::now());

    let bot = doc! {
        "bot_id": bot_id,
        "build_nbr": 0,
        "bot": bot,
        "ast": ast,
        "engine_version": "1.3",
        "updated_at": &time,
        "created_at": &time
    };

    let inserted = collection.insert_one(bot.clone(), None)?;

    let id = inserted.inserted_id.as_object_id().unwrap();

    Ok(id.to_hex())
}

pub fn get_bot_ast(
    id: &str,
    db: &mongodb::Database,
) -> Result<Option<HashMap<String, Flow>>, EngineError> {
    let collection = db.collection("ast");

    let filter = doc! {
        "_id": bson::oid::ObjectId::with_string(id).unwrap()
    };

    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();
    let result = collection.find_one(filter, find_options)?;

    match result {
        Some(conv) => {
            let conversation = format_bot_struct(conv)?;

            let base64decoded = base64::decode(&conversation.ast).unwrap();
            let csml_bot = bincode::deserialize(&base64decoded[..]).unwrap();

            Ok(Some(csml_bot))
        }
        None => Ok(None),
    }
}