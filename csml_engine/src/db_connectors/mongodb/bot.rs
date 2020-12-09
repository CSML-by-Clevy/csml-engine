use crate::{db_connectors::DbBot, encrypt::encrypt_data, Client, EngineError, CsmlBot, SerializeCsmlBot};
use csml_interpreter::data::ast::Flow;
use bson::{doc, Bson};
use mongodb::options::Hint;
use chrono::{SecondsFormat, DateTime, Utc};
use std::collections::HashMap;

fn format_bot_struct(
    bot: bson::ordered::OrderedDocument,
) -> Result<DbBot, EngineError> {
    Ok(DbBot {
        id: bot.get_object_id("_id").unwrap().to_hex(), // to_hex bson::oid::ObjectId
        // user_id: bot.get_str("user_id").unwrap().to_owned(),
        bot_id: bot.get_str("bot_id").unwrap().to_owned(),
        build_nbr: bot.get_i32("build_nbr").unwrap(),
        bot: bot.get_str("bot").unwrap().to_owned(),
        // ast: bot.get_str("ast").unwrap().to_owned(),
        engine_version: bot.get_str("engine_version").unwrap().to_owned(),
        created_at: bot
            .get_utc_datetime("created_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    })
}

pub fn create_bot_state(
    bot_id: String,
    bot: String,
    db: &mongodb::Database,
) -> Result<String, EngineError> {
    let collection = db.collection("ast");
    let time = Bson::UtcDatetime(chrono::Utc::now());

    let bot = doc! {
        "bot_id": bot_id,
        "bot": bot,
        "build_nbr": 0,
        "engine_version": env!("CARGO_PKG_VERSION").to_owned(),
        "created_at": &time
    };

    let inserted = collection.insert_one(bot.clone(), None)?;

    let id = inserted.inserted_id.as_object_id().unwrap();

    Ok(id.to_hex())
}

pub fn get_bot_list(
    bot_id: &str,
    db: &mongodb::Database,
) -> Result<Vec<serde_json::Value> , EngineError> {
    let collection = db.collection("ast"); // bot_version

    let filter = doc! {
        "bot_id": bot_id,
        // "_id": {"$gt": bson::oid::ObjectId::with_string("5fd0ea4200aca41f005c82af").unwrap() }
    };

    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "$natural": -1, })
        .batch_size(10)
        .limit(10)
        .build();

    let cursor = collection.find(filter, find_options)?; // list 
    let mut bots =  vec!();

    for doc in cursor {

        match doc {
            Ok(bot) => {
                let bot = format_bot_struct(bot)?;

                let base64decoded = base64::decode(&bot.bot).unwrap();
                let csml_bot: SerializeCsmlBot = bincode::deserialize(&base64decoded[..]).unwrap();


                let json = serde_json::json!({
                    "id": bot.id,
                    "bot": csml_bot.to_bot(),
                    "engine_version": bot.engine_version,
                    "created_at": bot.created_at
                });

                bots.push(json);
            }
            Err(_) => (),
        };
    }

    Ok(bots)
}

pub fn get_bot_by_id(
    id: &str,
    db: &mongodb::Database,
) -> Result<Option<CsmlBot>, EngineError> {
    let collection = db.collection("ast");

    let filter = doc! {
        "_id": bson::oid::ObjectId::with_string(id).unwrap()
    };

    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1, })
        .build();

    let result = collection.find_one(filter, find_options)?;

    match result {
        Some(bot) => {
            let bot = format_bot_struct(bot)?;

            let base64decoded = base64::decode(&bot.bot).unwrap();
            let csml_bot: SerializeCsmlBot = bincode::deserialize(&base64decoded[..]).unwrap();

            Ok(Some(csml_bot.to_bot()))
        }
        None => Ok(None),
    }
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &mongodb::Database,
) -> Result<Option<CsmlBot>, EngineError> {
    let collection = db.collection("ast");

    let filter = doc! {
        "bot_id": bot_id,
    };

    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1,  })
        .build();

    let result = collection.find_one(filter, find_options)?;

    match result {
        Some(bot) => {
            let bot = format_bot_struct(bot)?;

            let base64decoded = base64::decode(&bot.bot).unwrap();
            let csml_bot: SerializeCsmlBot = bincode::deserialize(&base64decoded[..]).unwrap();

            Ok(Some(csml_bot.to_bot()))
        }
        None => Ok(None),
    }
}