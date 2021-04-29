use crate::data::{DynamoDbClient};
use crate::db_connectors::dynamodb::{get_db, Memory, DynamoDbKey};
use crate::{
    encrypt::{decrypt_data, encrypt_data},
    Client, ConversationInfo, EngineError,
};
use csml_interpreter::data::Memory as InterpreterMemory;
use rusoto_dynamodb::*;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

fn format_memories(
    data: &ConversationInfo,
    memories: &[InterpreterMemory],
    interaction_order: i32,
) -> Result<Vec<Memory>, EngineError> {
    let mut res = vec![];

    for (i, mem) in memories.iter().enumerate() {
        res.push(Memory::new(
            &data.client,
            &data.conversation_id,
            &data.interaction_id,
            interaction_order,
            i as i32,
            &data.context.flow,
            &data.context.step,
            &mem.key,
            Some(encrypt_data(&mem.value)?),
        ));
    }

    Ok(res)
}

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &[InterpreterMemory],
    interaction_order: i32,
) -> Result<(), EngineError> {
    if memories.len() == 0 {
        return Ok(());
    }

    let memories = format_memories(data, memories, interaction_order)?;

    // We can only use BatchWriteItem on up to 25 items at once,
    // so we need to split the memories to write into chunks of max
    // 25 items.
    for chunk in memories.chunks(25) {
        let mut request_items = HashMap::new();

        let mut items_to_write = vec![];
        for data in chunk {
            items_to_write.push(WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&data)?,
                    ..Default::default()
                }),
                ..Default::default()
            });
        }

        request_items.insert(get_table_name()?, items_to_write);

        let input = BatchWriteItemInput {
            request_items,
            ..Default::default()
        };

        let db = get_db(&mut data.db)?;
        let future = db.client.batch_write_item(input);

        db.runtime.block_on(future)?;
    }

    Ok(())
}

pub fn create_client_memory(
    client: &Client,
    key: String,
    value: serde_json::Value,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let memories = Memory::new(
        client,
        "_",
        "_",
        0,
        0,
        "_",
        "_",
        &key,
        Some(encrypt_data(&value)?),
    );

    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&memories)?,
        table_name: get_table_name()?,
        ..Default::default()
    };

    let client = db.client.to_owned();
    let future = client.put_item(input);

    db.runtime.block_on(future)?;

    Ok(())
}

fn query_memories(
    client: &Client,
    range: String,
    range_key: &str,
    index_name: Option<String>,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryOutput, EngineError> {
    let hash = Memory::get_hash(client);

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), range_key.to_owned()),
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
                s: Some(range),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name,
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

pub fn get_memories(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let mut memories = vec![];
    let mut last_evaluated_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_memories(
            client,
            String::from("memory#"),
            "range_time",
            Some("TimeIndex".to_owned()),
            db,
            25,
            last_evaluated_key
        )?;

        match data.items {
            Some(val) => {
                for item in val.iter() {
                    let mem: Memory = serde_dynamodb::from_hashmap(item.to_owned())?;

                    let mut json_value = serde_json::json!(mem);
                    json_value["value"] = decrypt_data(json_value["value"].as_str().unwrap().to_string())?;
                    memories.push(json_value);
                }

                if let None = data.last_evaluated_key {
                    break;
                }

                last_evaluated_key = data.last_evaluated_key;
            }
            _ => break,
        };
    }

    // format memories output
    let mut map = serde_json::Map::new();
    for mem in memories {
        let key = mem["key"].as_str().unwrap();
        if !map.contains_key(key) {
            map.insert(key.to_string(), mem["value"].clone());
        }
    }

    Ok(serde_json::json!(map))
}

fn get_memory_batches_to_delete(
    client: &Client,
    range: String,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let mut pagination_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_memories(client, range.clone(), "range", None, db, 25, pagination_key)?;

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
            let mem: Memory = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: Memory::get_hash(client),
                range: Memory::get_range(&mem.key, &mem.id),
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
            return Ok(())
        }
    }
}

pub fn delete_client_memories(client: &Client, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    get_memory_batches_to_delete(client, format!("memory#"), db)
}

pub fn delete_client_memory(
    client: &Client,
    key: &str,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    get_memory_batches_to_delete(client, format!("memory#{}", key), db)
}