use crate::data::DynamoDbClient;
use csml_interpreter::data::{csml_bot::DynamoBot, csml_flow::CsmlFlow,};
use crate::db_connectors::dynamodb::{Bot, DynamoDbKey, Flows};
use crate::{CsmlBot, EngineError};

use std::collections::HashMap;
use rusoto_dynamodb::*;

use crate::db_connectors::dynamodb::utils::*;

pub fn create_bot_version(
    bot_id: String,
    bot: String,
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
    Ok(data.id.to_owned())
}

fn create_flows_baches(mut flows: Vec<CsmlFlow>,) -> Vec<String> {
    let mut flows_vec = vec!();
    let mut max_range = flows.len();

    while max_range > 0 {
        let range = match max_range {
            max_range if max_range > 10 => 9,
            max_range => max_range
        };

        let tmp: Vec<CsmlFlow> = flows.drain(0..range).collect();

        let serialize_flows = base64::encode(bincode::serialize(&tmp).unwrap());
        flows_vec.push(serialize_flows);
        max_range = max_range - range;

    }

    flows_vec
}

pub fn create_flows_batches(
    bot_id: String,
    id_bot: String,
    flows: Vec<CsmlFlow>,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let flows_vec = create_flows_baches(flows);

    for flows in flows_vec {
        let data: Flows = Flows::new(bot_id.clone(), id_bot.clone(), flows);
        let input = PutItemInput {
            item: serde_dynamodb::to_hashmap(&data)?,
            table_name: get_table_name()?,
            ..Default::default()
        };
        let client = db.client.to_owned();
        let future = client.put_item(input);

        db.runtime.block_on(future)?;
    }
    Ok(())
}

pub fn get_flows(
    bot_id: &str,
    id_bot: &str,
    db: &mut DynamoDbClient,
) -> Result<Vec<CsmlFlow>, EngineError> {
    let mut flows = vec![];
    let mut last_evaluated_key = None;

    // recursively retrieve all memories from dynamodb
    loop {
        let tmp = query_flows(bot_id, id_bot, db, last_evaluated_key)?;
        let mut items = tmp.items.to_owned();
        flows.append(&mut items);
        if let None = tmp.last_evaluated_key {
            break;
        }
        last_evaluated_key = tmp.last_evaluated_key;
    }

    Ok(flows)
}

struct QueryResult {
    last_evaluated_key: Option<HashMap<String, AttributeValue>>,
    items: Vec<CsmlFlow>,
}

fn query_flows(
    bot_id: &str,
    id_bot: &str,
    db: &mut DynamoDbClient,
    last_evaluated_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryResult, EngineError> {
    let hash = Flows::get_hash(&bot_id);

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
                s: Some(hash),
                ..Default::default()
            },
        ),
        (
            String::from(":rangePrefix"),
            AttributeValue {
                s: Some(format!("flows#{}", id_bot)),
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
        select: Some(String::from("ALL_ATTRIBUTES")),
        exclusive_start_key: last_evaluated_key,
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = db.runtime.block_on(future)?;

    let mut items = vec![];
    match data.items {
        Some(val) => {
            for item in val.iter() {
                let data: Flows = serde_dynamodb::from_hashmap(item.to_owned())?;

                let base64decoded = base64::decode(&data.flows).unwrap();
                let mut flows: Vec<CsmlFlow> = bincode::deserialize(&base64decoded[..]).unwrap();
                items.append(&mut flows);
            }
        }
        _ => (),
    };

    Ok(QueryResult {
        last_evaluated_key: data.last_evaluated_key,
        items,
    })
}

pub fn get_bot_versions(
    bot_id: &str,
    last_key: Option<String>,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, EngineError> {
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

    let last_evaluated_key = match last_key {
        Some(key) => serde_json::from_str(&key).unwrap(),
        None => None,
    };

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some(String::from("TimeIndex")),
        key_condition_expression: Some(key_cond_expr),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(1),
        select: Some(String::from("ALL_ATTRIBUTES")),
        exclusive_start_key: last_evaluated_key,
        ..Default::default()
    };

    let query = db.client.query(input);
    let data = db.runtime.block_on(query)?;

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

        let json = serde_json::json!({
            "id": data.id,
            "bot": csml_bot,
            "engine_version": data.engine_version,
            "created_at": data.created_at
        });

        bots.push(json);
    }

    let last_key = serde_json::json!(data.last_evaluated_key).to_string();

    Ok(serde_json::json!({"bots": bots, "last_key": last_key}))
}

pub fn get_bot_by_version_id(
    id: &str,
    bot_id: &str,
    db: &mut DynamoDbClient,
) -> Result<Option<CsmlBot>, EngineError> {
    let item_key = DynamoDbKey {
        hash: Bot::get_hash(bot_id),
        range: Bot::get_range(id),
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

            let flows = get_flows(&csml_bot.id, &bot.id, db)?;

            Ok(Some(csml_bot.to_bot(flows)))
        }
        _ => Ok(None),
    }
}

pub fn get_last_bot_version(
    bot_id: &str,
    db: &mut DynamoDbClient,
) -> Result<Option<CsmlBot>, EngineError> {
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

    let flows = get_flows(&csml_bot.id, &bot.id, db)?;

    Ok(Some(csml_bot.to_bot(flows)))
}
