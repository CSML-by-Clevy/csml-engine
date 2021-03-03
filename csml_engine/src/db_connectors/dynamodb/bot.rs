use crate::data::{DynamoDbClient, DynamoBot};
use crate::db_connectors::{
    dynamodb::{aws_s3, Bot, DynamoDbKey},
    BotVersion,
};
use crate::EngineError;
use csml_interpreter::data::{csml_flow::CsmlFlow};

use rusoto_dynamodb::*;

use crate::db_connectors::dynamodb::utils::*;

pub fn create_bot_version(
    bot_id: String,
    bot: String,
    flows: String,
    db: &mut DynamoDbClient,
) -> Result<String, EngineError> {
    let data: Bot = Bot::new(bot_id, bot);

    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&data)?,
        table_name: get_table_name()?,
        ..Default::default()
    };

    let client = db.client.to_owned();
    let future = client.put_item(input);
    db.runtime.block_on(future)?;

    let key = format!("bots/{}/versions/{}/flows.json", &data.id, &data.version_id);
    aws_s3::put_object(db, &key, flows)?;

    Ok(data.version_id.to_owned())
}

pub fn get_flows(key: &str, db: &mut DynamoDbClient) -> Result<Vec<CsmlFlow>, EngineError> {
    let object = aws_s3::get_object(db, key)?;
    let flows: Vec<CsmlFlow> = serde_json::from_str(&object).unwrap();

    Ok(flows)
}

fn query_bot_version(
    bot_id: &str,
    limit: i64,
    pagination_key: Option<String>,
    db: &mut DynamoDbClient,
) -> Result<QueryOutput, EngineError> {
    let hash = Bot::get_hash(bot_id);
    let key_cond_expr = "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_string();
    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range_time")), // time index
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(hash.to_string()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangePrefix"),
            AttributeValue {
                s: Some(String::from("bot#")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let last_evaluated_key = match pagination_key {
        Some(key) => {
            let base64decoded = match base64::decode(&key) {
                Ok(base64decoded) => base64decoded,
                Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
            };

            match serde_json::from_slice(&base64decoded) {
                Ok(key) => Some(key),
                Err(_) => return Err(EngineError::Manager(format!("Invalid pagination_key"))),
            }
        }
        None => None,
    };

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some(String::from("TimeIndex")),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(limit),
        select: Some(String::from("ALL_ATTRIBUTES")),
        scan_index_forward: Some(false),
        exclusive_start_key: last_evaluated_key,
        ..Default::default()
    };

    let query = db.client.query(input);
    let data = db.runtime.block_on(query)?;

    Ok(data)
}

pub fn get_bot_versions(
    bot_id: &str,
    limit: Option<i64>,
    pagination_key: Option<String>,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let limit = match limit {
        Some(limit) if limit >= 1 => limit,
        Some(_limit) => 20,
        None => 20,
    };

    let data = query_bot_version(bot_id, limit, pagination_key, db)?;
    // The query returns an array of items (max 10, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    // , "last_key": :
    let items = match data.items {
        None => return Ok(serde_json::json!({"bots": []})),
        Some(items) if items.len() == 0 => return Ok(serde_json::json!({"bots": []})),
        Some(items) => items.clone(),
    };

    let mut bots = vec![];

    for item in items.iter() {
        let data: Bot = serde_dynamodb::from_hashmap(item.to_owned())?;

        let base64decoded = base64::decode(&data.bot).unwrap();
        let csml_bot: DynamoBot = bincode::deserialize(&base64decoded[..]).unwrap();

        let mut json = serde_json::json!({
            "version_id": data.version_id,
            "id": data.id,
            "name": csml_bot.name,
            "default_flow": csml_bot.default_flow,
            "engine_version": data.engine_version,
            "created_at": data.created_at
        });

        if let Some(custom_components) = csml_bot.custom_components {
            json["custom_components"] = serde_json::json!(custom_components);
        }

        bots.push(json);
    }

    match data.last_evaluated_key {
        Some(pagination_key) => {
            let pagination_key = base64::encode(serde_json::json!(pagination_key).to_string());

            Ok(serde_json::json!({"bots": bots, "pagination_key": pagination_key}))
        }
        None => Ok(serde_json::json!({ "bots": bots })),
    }
}

pub fn get_bot_by_version_id(
    version_id: &str,
    bot_id: &str,
    db: &mut DynamoDbClient,
) -> Result<Option<BotVersion>, EngineError> {
    let item_key = DynamoDbKey {
        hash: Bot::get_hash(bot_id),
        range: Bot::get_range(version_id),
    };

    let input = GetItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&item_key)?,
        ..Default::default()
    };

    let future = db.client.get_item(input);
    let res = db.runtime.block_on(future)?;

    match res.item {
        Some(val) => {
            let bot: Bot = serde_dynamodb::from_hashmap(val)?;
            let base64decoded = base64::decode(&bot.bot).unwrap();
            let csml_bot: DynamoBot = bincode::deserialize(&base64decoded[..]).unwrap();

            let key = format!("bots/{}/versions/{}/flows.json", bot_id, version_id);
            let flows = get_flows(&key, db)?;

            Ok(Some(BotVersion {
                bot: csml_bot.to_bot(flows)?,
                version_id: bot.version_id,
                engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            }))
        }
        _ => Ok(None),
    }
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &mut DynamoDbClient,
) -> Result<Option<BotVersion>, EngineError> {
    let hash = Bot::get_hash(bot_id);

    let key_cond_expr = "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_string();
    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range_time")), // time index
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(hash.to_string()),
                ..Default::default()
            },
        ),
        (
            String::from(":rangePrefix"),
            AttributeValue {
                s: Some(String::from("bot#")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some(String::from("TimeIndex")),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        scan_index_forward: Some(false),
        limit: Some(1),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let query = db.client.query(input);
    let data = db.runtime.block_on(query)?;

    // The query returns an array of items (max 1, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    let item = match data.items {
        None => return Ok(None),
        Some(items) if items.len() == 0 => return Ok(None),
        Some(items) => items[0].clone(),
    };

    let bot: Bot = serde_dynamodb::from_hashmap(item)?;
    let base64decoded = base64::decode(&bot.bot).unwrap();
    let csml_bot: DynamoBot = bincode::deserialize(&base64decoded[..]).unwrap();

    let key = format!("bots/{}/versions/{}/flows.json", bot_id, bot.version_id);
    let flows = get_flows(&key, db)?;

    Ok(Some(BotVersion {
        bot: csml_bot.to_bot(flows)?,
        version_id: bot.version_id,
        engine_version: env!("CARGO_PKG_VERSION").to_owned(),
    }))
}

pub fn delete_bot_version(
    bot_id: &str,
    version_id: &str,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let key = format!("bots/{}/versions/{}/flows.json", bot_id, version_id);
    aws_s3::delete_object(db, &key)?;

    let item_key = DynamoDbKey {
        hash: Bot::get_hash(bot_id),
        range: Bot::get_range(version_id),
    };

    let input = DeleteItemInput {
        table_name: get_table_name()?,
        key: serde_dynamodb::to_hashmap(&item_key)?,
        ..Default::default()
    };

    let future = db.client.delete_item(input);
    db.runtime.block_on(future)?;

    Ok(())
}

fn get_bot_version_batches_and_delete_flows(
    bot_id: &str,
    db: &mut DynamoDbClient,
) -> Result<Vec<Vec<WriteRequest>>, EngineError> {
    let mut batches = vec![];
    let mut pagination_key = None;

    loop {
        // 25 is the Maximum operations in a single request for BatchWriteItemInput
        let data = query_bot_version(bot_id, 25, pagination_key, db)?;

        // The query returns an array of items (max 10, based on the limit param above).
        // If 0 item is returned it means that there is no open conversation, so simply return None
        // , "last_key": :
        let items = match data.items {
            None => return Ok(batches),
            Some(items) if items.len() == 0 => return Ok(batches),
            Some(items) => items.clone(),
        };

        let mut write_requests = vec![];
        for item in items {
            let data: Bot = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = format!("bots/{}/versions/{}/flows.json", bot_id, data.version_id);
            aws_s3::delete_object(db, &key)?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: Bot::get_hash(bot_id),
                range: Bot::get_range(&data.version_id),
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest { key }),
                put_request: None,
            });
        }
        batches.push(write_requests);

        pagination_key = match data.last_evaluated_key {
            Some(pagination_key) => Some(base64::encode(
                serde_json::json!(pagination_key).to_string(),
            )),
            None => return Ok(batches),
        };
    }
}

pub fn delete_bot_versions(bot_id: &str, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let batches = get_bot_version_batches_and_delete_flows(bot_id, db)?;

    for write_requests in batches {
        let request_items = [(get_table_name()?, write_requests)]
            .iter()
            .cloned()
            .collect();

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        let future = db.client.batch_write_item(input);
        db.runtime.block_on(future)?;
    }
    Ok(())
}
