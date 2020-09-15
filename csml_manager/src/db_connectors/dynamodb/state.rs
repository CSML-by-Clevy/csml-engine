use crate::{ConversationInfo, ManagerError, Client, encrypt::{encrypt_data, decrypt_data}};
use crate::db_connectors::dynamodb::{DynamoDbKey, State, get_db};
use crate::data::DynamoDbClient;
use std::collections::HashMap;
use rusoto_dynamodb::*;

use crate::db_connectors::dynamodb::utils::*;

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &DynamoDbClient,
) -> Result<(), ManagerError> {

    let item_key = DynamoDbKey {
        hash: State::get_hash(client),
        range: State::get_range(_type, key),
    };

    let input = DeleteItemInput {
        key: to_attribute_value_map(&item_key)?,
        ..Default::default()
    };

    let mut runtime = db.get_runtime()?;
    let future = db.client.delete_item(input);
    runtime.block_on(future)?;

    Ok(())
}

pub fn get_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &DynamoDbClient,
) -> Result<Option<serde_json::Value>, ManagerError> {

    let item_key = DynamoDbKey {
        hash: State::get_hash(client),
        range: State::get_range(_type, key),
    };

    let input = GetItemInput {
        key: to_attribute_value_map(&item_key)?,
        ..Default::default()
    };

    let mut runtime = db.get_runtime()?;
    let future = db.client.get_item(input);
    let res = runtime.block_on(future)?;

    match res.item {
        Some(val) => {
            let mut val = from_attribute_value_map(&val)?;
            val["value"] = decrypt_data(val["value"].to_string())?;
            Ok(Some(val))
        },
        _ => Ok(None),
    }

}


fn format_state_data(
    data: &mut ConversationInfo,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<Vec<State>, ManagerError> {
    let mut vec = vec![];
    for (key, value) in keys_values.iter() {
        let encrypted_value = encrypt_data(value)?;
        vec.push(State::new(
            &data.client,
            _type,
            *key,
            &encrypted_value,
        ));
    }
    Ok(vec)
}

pub fn set_state_items(
    data: &mut ConversationInfo,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {

    let states = format_state_data(data, _type, keys_values)?;

    // We can only use BatchWriteItem on up to 25 items at once,
    // so we need to split the memories to write into chunks of max
    // 25 items.
    for chunk in states.chunks(25) {

        let mut request_items = HashMap::new();

        let mut items_to_write = vec![];
        for data in chunk {
            items_to_write.push(WriteRequest {
                put_request: Some(PutRequest {
                    item: to_attribute_value_map(&data)?,
                }),
                ..Default::default()
            });
        };

        request_items.insert(
            "PutRequest".to_owned(),
            items_to_write,
        );

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        let db = get_db(&data.db)?;
        let mut runtime = db.get_runtime()?;
        let future = db.client.batch_write_item(input);

        runtime.block_on(future)?;
    }

    Ok(())
}
