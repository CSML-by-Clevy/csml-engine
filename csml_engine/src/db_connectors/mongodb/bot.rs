use crate::{
    data::{CsmlBotBincode, MongoDbClient, SerializeCsmlBot},
    db_connectors::{BotVersion, DbBot},
    EngineError,
};
use bson::{doc, Document};
use chrono::SecondsFormat;

fn format_bot_struct(bot: bson::document::Document) -> Result<DbBot, EngineError> {
    Ok(DbBot {
        id: bot.get_object_id("_id").unwrap().to_hex(),
        bot_id: bot.get_str("bot_id").unwrap().to_owned(),
        bot: bot.get_str("bot").unwrap().to_owned(),
        engine_version: bot.get_str("engine_version").unwrap().to_owned(),
        created_at: bot
            .get_datetime("created_at")
            .unwrap()
            .to_chrono()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
    })
}

pub fn create_bot_version(
    bot_id: String,
    bot: String,
    db: &MongoDbClient,
) -> Result<String, EngineError> {
    let collection = db.client.collection::<Document>("bot");
    let time = bson::DateTime::from_chrono(chrono::Utc::now());

    let bot = doc! {
        "bot_id": bot_id,
        "bot": bot,
        "engine_version": env!("CARGO_PKG_VERSION").to_owned(),
        "created_at": &time
    };

    let inserted = collection.insert_one(bot.clone(), None)?;

    let id = inserted.inserted_id.as_object_id().unwrap();

    Ok(id.to_hex())
}

pub fn get_bot_versions(
    bot_id: &str,
    limit: Option<i64>,
    pagination_key: Option<String>,
    db: &MongoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let collection = db.client.collection::<Document>("bot");

    let limit = match limit {
        Some(limit) if limit >= 1 => limit + 1,
        Some(_limit) => 21,
        None => 21,
    };

    let filter = match pagination_key {
        Some(key) => {
            doc! {
                "bot_id": bot_id,
                "_id": {"$gt": bson::oid::ObjectId::parse_str(&key).unwrap() }
            }
        }
        None => doc! {"bot_id": bot_id },
    };

    let find_options = mongodb::options::FindOptions::builder()
        .sort(doc! { "$natural": -1, })
        .batch_size(30)
        .limit(limit)
        .build();

    let cursor = collection.find(filter, find_options)?;
    let mut bots = vec![];

    for doc in cursor {
        match doc {
            Ok(bot_doc) => {
                let bot_version = format_bot_struct(bot_doc)?;

                let csml_bot: SerializeCsmlBot = match base64::decode(&bot_version.bot) {
                    Ok(base64decoded) => {
                        match bincode::deserialize::<CsmlBotBincode>(&base64decoded[..]) {
                            Ok(bot) => bot.to_bot(),
                            Err(_) => serde_json::from_str(&bot_version.bot).unwrap(),
                        }
                    }
                    Err(_) => serde_json::from_str(&bot_version.bot).unwrap(),
                };

                let mut json = serde_json::json!({
                    "version_id": bot_version.id,
                    "id": csml_bot.id,
                    "name": csml_bot.name,
                    "default_flow": csml_bot.default_flow,
                    "engine_version": bot_version.engine_version,
                    "created_at": bot_version.created_at
                });

                if let Some(custom_components) = csml_bot.custom_components {
                    json["custom_components"] = serde_json::json!(custom_components);
                }

                bots.push(json);
            }
            Err(_) => (),
        };
    }

    match bots.len() == limit as usize {
        true => {
            bots.pop();
            match bots.last() {
                Some(last) => {
                    let pagination_key = base64::encode(last["version_id"].clone().to_string());

                    Ok(serde_json::json!({"bots": bots, "pagination_key": pagination_key}))
                }
                None => Ok(serde_json::json!({ "bots": bots })),
            }
        }
        false => Ok(serde_json::json!({ "bots": bots })),
    }
}

pub fn get_bot_by_version_id(
    id: &str,
    db: &MongoDbClient,
) -> Result<Option<BotVersion>, EngineError> {
    let collection = db.client.collection::<Document>("bot");

    let filter = doc! {
        "_id": bson::oid::ObjectId::parse_str(id).unwrap()
    };

    let find_options = mongodb::options::FindOneOptions::builder()
        .sort(doc! { "$natural": -1, })
        .build();

    let result = collection.find_one(filter, find_options)?;

    match result {
        Some(bot) => {
            let bot = format_bot_struct(bot)?;

            let csml_bot: SerializeCsmlBot = match base64::decode(&bot.bot) {
                Ok(base64decoded) => {
                    match bincode::deserialize::<CsmlBotBincode>(&base64decoded[..]) {
                        Ok(bot) => bot.to_bot(),
                        Err(_) => serde_json::from_str(&bot.bot).unwrap(),
                    }
                }
                Err(_) => serde_json::from_str(&bot.bot).unwrap(),
            };

            Ok(Some(BotVersion {
                bot: csml_bot.to_bot(),
                version_id: bot.id,
                engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            }))
        }
        None => Ok(None),
    }
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &MongoDbClient,
) -> Result<Option<BotVersion>, EngineError> {
    let collection = db.client.collection::<Document>("bot");

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

            let csml_bot: SerializeCsmlBot = match base64::decode(&bot.bot) {
                Ok(base64decoded) => {
                    match bincode::deserialize::<CsmlBotBincode>(&base64decoded[..]) {
                        Ok(bot) => bot.to_bot(),
                        Err(_) => serde_json::from_str(&bot.bot).unwrap(),
                    }
                }
                Err(_) => serde_json::from_str(&bot.bot).unwrap(),
            };

            Ok(Some(BotVersion {
                bot: csml_bot.to_bot(),
                version_id: bot.id,
                engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            }))
        }
        None => Ok(None),
    }
}

pub fn delete_bot_version(version_id: &str, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection::<Document>("bot");

    let filter = doc! {
        "_id": bson::oid::ObjectId::parse_str(version_id).unwrap()
    };

    collection.delete_one(filter, None)?;

    Ok(())
}

pub fn delete_bot_versions(bot_id: &str, db: &MongoDbClient) -> Result<(), EngineError> {
    let collection = db.client.collection::<Document>("bot");

    let filter = doc! {
        "bot_id": bot_id,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}

pub fn delete_all_bot_data(
    bot_id: &str,
    class: &str,
    db: &MongoDbClient,
) -> Result<(), EngineError> {
    let collection = db.client.collection::<Document>(class);

    let filter = doc! {
        "client.bot_id": bot_id,
    };

    collection.delete_many(filter, None)?;

    Ok(())
}
