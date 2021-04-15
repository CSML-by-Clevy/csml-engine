use crate::data::DynamoDbClient;
use crate::db_connectors::dynamodb::{DynamoDbKey, State};
use crate::{
    encrypt::{decrypt_data, encrypt_data},
    Client, EngineError,
};
use rusoto_dynamodb::*;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let item_key = DynamoDbKey {
        hash: State::get_hash(client),
        range: State::get_range(_type, key),
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

pub fn get_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &mut DynamoDbClient,
) -> Result<Option<serde_json::Value>, EngineError> {
    let item_key = DynamoDbKey {
        hash: State::get_hash(client),
        range: State::get_range(_type, key),
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
            let state: State = serde_dynamodb::from_hashmap(val)?;

            let val = serde_json::json!(state);
            let value = decrypt_data(val["value"].as_str().unwrap().to_string())?;
            Ok(Some(value))
        }
        _ => Ok(None),
    }
}

fn format_state_data(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<Vec<State>, EngineError> {
    let mut vec = vec![];
    for (key, value) in keys_values.iter() {
        let encrypted_value = encrypt_data(value)?;
        vec.push(State::new(client, _type, *key, &encrypted_value));
    }
    Ok(vec)
}

pub fn set_state_items(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let states = format_state_data(&client, _type, keys_values)?;

    // We can only use BatchWriteItem on up to 25 items at once,
    // so we need to split the memories to write into chunks of max
    // 25 items.
    for chunk in states.chunks(25) {
        let mut request_items = HashMap::new();

        let mut items_to_write = vec![];
        for data in chunk {
            items_to_write.push(WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&data)?,
                }),
                ..Default::default()
            });
        }

        request_items.insert(get_table_name()?, items_to_write);

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        let future = db.client.batch_write_item(input);

        db.runtime.block_on(future)?;
    }

    Ok(())
}

fn query_states(
    client: &Client,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryOutput, EngineError> {
    let hash = State::get_hash(client);

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
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
                s: Some(String::from("state")),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        key_condition_expression: Some(
            "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_owned(),
        ),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(limit),
        exclusive_start_key: pagination_key,
        scan_index_forward: Some(false),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = db.runtime.block_on(future)?;

    Ok(data)
}

fn get_state_batches_to_delete(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Vec<Vec<WriteRequest>>, EngineError> {
    let mut batches = vec![];
    let mut pagination_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_states(client, db, 25, pagination_key)?;

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
            let state: State = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: State::get_hash(client),
                range: State::get_range(&state._type, &state.key),
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest { key }),
                put_request: None,
            });
        }

        batches.push(write_requests);

        pagination_key = match data.last_evaluated_key {
            Some(pagination_key) => Some(pagination_key),
            None => return Ok(batches),
        };
    }
}

pub fn delete_user_state(client: &Client, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let batches = get_state_batches_to_delete(client, db)?;

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