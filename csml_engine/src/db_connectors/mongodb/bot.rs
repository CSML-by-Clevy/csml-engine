use crate::{db_connectors::DbBot, encrypt::encrypt_data, Client, EngineError, CsmlBot, SerializeCsmlBot};
use csml_interpreter::data::ast::Flow;
use bson::{doc, Bson};
use chrono::SecondsFormat;
use std::collections::HashMap;

fn format_bot_struct(
    bot: bson::ordered::OrderedDocument,
) -> Result<DbBot, EngineError> {
    Ok(DbBot {
        id: bot.get_object_id("_id").unwrap().to_hex(), // to_hex bson::oid::ObjectId
        bot_id: bot.get_str("bot_id").unwrap().to_owned(),
        build_nbr: bot.get_i32("build_nbr").unwrap(),
        bot: bot.get_str("bot").unwrap().to_owned(),
        ast: bot.get_str("ast").unwrap().to_owned(),
        engine_version: bot.get_str("engine_version").unwrap().to_owned(),
        updated_at: bot
            .get_utc_datetime("updated_at")
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
        created_at: bot
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
) -> Result<Option<CsmlBot>, EngineError> { //HashMap<String, Flow>
    let collection = db.collection("ast");

    // "_id": bson::oid::ObjectId::with_string(id).unwrap()
    let filter = doc! {
        "bot_id": id
    };

    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1 })
        .build();

    let result = collection.find(filter, find_options)?;

    for doc in cursor {
        println!("{}", doc?)
    }
    panic!("");

    // match result {
    //     Some(bot) => {
    //         let bot = format_bot_struct(bot)?;

    //         let base64decoded = base64::decode(&bot.bot).unwrap();
    //         let csml_bot: SerializeCsmlBot = bincode::deserialize(&base64decoded[..]).unwrap();

    //         Ok(Some(csml_bot.to_bot()))
    //     }
    //     None => Ok(None),
    // }
}