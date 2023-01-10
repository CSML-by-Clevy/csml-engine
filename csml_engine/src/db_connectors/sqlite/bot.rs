use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{BotVersion, EngineError, SerializeCsmlBot, SqliteClient};

use super::{models, pagination::*, schema::cmsl_bot_versions};

use std::env;

pub fn create_bot_version(
    bot_id: String,
    bot: String,
    db: &mut SqliteClient,
) -> Result<String, EngineError> {
    let id = models::UUID::new_v4();

    let newbot = models::NewBot {
        id,
        bot_id: &bot_id,
        bot: &bot,
        engine_version: env!("CARGO_PKG_VERSION"),
    };

    diesel::insert_into(cmsl_bot_versions::table)
        .values(&newbot)
        .execute(db.client.as_mut())?;

    Ok(id.to_string())
}

pub fn get_bot_versions(
    bot_id: &str,
    limit: Option<i64>,
    pagination_key: Option<String>,
    db: &mut SqliteClient,
) -> Result<serde_json::Value, EngineError> {
    let pagination_key = match pagination_key {
        Some(paginate) => paginate.parse::<i64>().unwrap_or(1),
        None => 1,
    };

    let mut query = cmsl_bot_versions::table
        .order_by(cmsl_bot_versions::updated_at.desc())
        .filter(cmsl_bot_versions::bot_id.eq(bot_id))
        .paginate(pagination_key);

    let limit_per_page = match limit {
        Some(limit) => std::cmp::min(limit, 25),
        None => 25,
    };
    query = query.per_page(limit_per_page);

    let (bot_versions, total_pages) = query.load_and_count_pages::<models::Bot>(db.client.as_mut())?;

    let mut bots = vec![];
    for bot_version in bot_versions {
        let csml_bot: SerializeCsmlBot = serde_json::from_str(&bot_version.bot).unwrap();

        let mut json = serde_json::json!({
            "version_id": bot_version.id.get_uuid(),
            "id": csml_bot.id,
            "name": csml_bot.name,
            "default_flow": csml_bot.default_flow,
            "engine_version": bot_version.engine_version,
            "created_at": bot_version.created_at.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
        });

        if let Some(custom_components) = csml_bot.custom_components {
            json["custom_components"] = serde_json::json!(custom_components);
        }

        bots.push(json);
    }

    match pagination_key < total_pages {
        true => {
            let pagination_key = (pagination_key + 1).to_string();
            Ok(serde_json::json!({"bots": bots, "pagination_key": pagination_key}))
        }
        false => Ok(serde_json::json!({ "bots": bots })),
    }
}

pub fn get_bot_by_version_id(
    id: &str,
    db: &mut SqliteClient,
) -> Result<Option<BotVersion>, EngineError> {
    let version_id = models::UUID::parse_str(id).unwrap();

    let result: Result<models::Bot, diesel::result::Error> = cmsl_bot_versions::table
        .filter(cmsl_bot_versions::id.eq(&version_id))
        .first::<models::Bot>(db.client.as_mut());

    match result {
        Ok(bot) => {
            let csml_bot: SerializeCsmlBot = serde_json::from_str(&bot.bot).unwrap();

            Ok(Some(BotVersion {
                bot: csml_bot.to_bot(),
                version_id: bot.id.to_string(),
                engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            }))
        }
        Err(..) => Ok(None),
    }
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &mut SqliteClient,
) -> Result<Option<BotVersion>, EngineError> {
    let result: Result<models::Bot, diesel::result::Error> = cmsl_bot_versions::table
        .filter(cmsl_bot_versions::bot_id.eq(&bot_id))
        .order_by(cmsl_bot_versions::created_at.desc())
        .get_result(db.client.as_mut());

    match result {
        Ok(bot) => {
            let csml_bot: SerializeCsmlBot = serde_json::from_str(&bot.bot).unwrap();

            Ok(Some(BotVersion {
                bot: csml_bot.to_bot(),
                version_id: bot.id.to_string(),
                engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            }))
        }
        Err(..) => Ok(None),
    }
}

pub fn delete_bot_version(version_id: &str, db: &mut SqliteClient) -> Result<(), EngineError> {
    let id = match models::UUID::parse_str(version_id) {
        Ok(id) => id,
        Err(..) => return Ok(()),
    };

    diesel::delete(cmsl_bot_versions::table.filter(cmsl_bot_versions::id.eq(id)))
        .execute(db.client.as_mut())
        .ok();

    Ok(())
}

pub fn delete_bot_versions(bot_id: &str, db: &mut SqliteClient) -> Result<(), EngineError> {
    diesel::delete(cmsl_bot_versions::table.filter(cmsl_bot_versions::bot_id.eq(bot_id)))
        .execute(db.client.as_mut())
        .ok();

    Ok(())
}
