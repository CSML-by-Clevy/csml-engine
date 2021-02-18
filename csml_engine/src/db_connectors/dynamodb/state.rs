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
