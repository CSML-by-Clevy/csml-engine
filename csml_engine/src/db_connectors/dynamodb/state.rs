use crate::data::DynamoDbClient;
use crate::db_connectors::dynamodb::{DynamoDbKey, StatDeleteInfo, State};
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

pub fn get_current_state(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Option<serde_json::Value>, EngineError> {
    let item_key = DynamoDbKey {
        hash: State::get_hash(client),
        range: State::get_range("hold", "position"),
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
            let dynamo_state: State = serde_dynamodb::from_hashmap(val)?;

            let mut state = serde_json::json!(dynamo_state);
            state["value"] = decrypt_data(state["value"].as_str().unwrap().to_string())?;

            let current_state = serde_json::json!({
                "client": state["client"],
                "type": state["type"],
                "value": state["value"],
                "created_at": state["created_at"],
            });

            Ok(Some(current_state))
        }
        _ => Ok(None),
    }
}

fn format_state_data(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
    expires_at: Option<i64>,
) -> Result<Vec<State>, EngineError> {
    let mut vec = vec![];
    for (key, value) in keys_values.iter() {
        let encrypted_value = encrypt_data(value)?;
        vec.push(State::new(
            client,
            _type,
            *key,
            &encrypted_value,
            expires_at,
        ));
    }
    Ok(vec)
}

pub fn set_state_items(
    client: &Client,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
    expires_at: Option<i64>,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let states = format_state_data(&client, _type, keys_values, expires_at)?;

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
    projection_expression: Option<String>,
    expression_attribute_names: Option<HashMap<String, String>>,
) -> Result<QueryOutput, EngineError> {
    let hash = State::get_hash(client);

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
        expression_attribute_names,
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(limit),
        exclusive_start_key: pagination_key,
        scan_index_forward: Some(false),
        projection_expression,
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = match db.runtime.block_on(future) {
        Ok(data) => data,
        Err(e) => return Err(EngineError::Manager(format!("query_states {:?}", e))),
    };

    Ok(data)
}

pub fn delete_user_state(client: &Client, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let mut pagination_key = None;
    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
        (String::from("#type"), String::from("type")),
        (String::from("#key"), String::from("key")),
    ]
    .iter()
    .cloned()
    .collect();

    // retrieve all memories from dynamodb
    loop {
        let data = query_states(
            client,
            db,
            25,
            pagination_key,
            Some("#type, #key".to_owned()),
            Some(expr_attr_names.clone()),
        )?;

        // The query returns an array of items (max 10, based on the limit param above).
        // If 0 item is returned it means that there is no open conversation, so simply return None
        // , "last_key": :
        let items = match data.items {
            None => return Ok(()),
            Some(items) if items.len() == 0 => return Ok(()),
            Some(items) => items.clone(),
        };

        let mut write_requests = vec![];

        for item in items {
            let state: StatDeleteInfo = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: State::get_hash(client),
                range: State::get_range(&state._type, &state.key),
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest { key }),
                put_request: None,
            });
        }

        let request_items = [(get_table_name()?, write_requests)]
            .iter()
            .cloned()
            .collect();

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        execute_batch_write_query(db, input)?;

        pagination_key = data.last_evaluated_key;
        if let None = &pagination_key {
            return Ok(());
        }
    }
}
