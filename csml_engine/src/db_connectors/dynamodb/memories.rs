use crate::data::{DynamoDbClient};
use crate::db_connectors::dynamodb::{get_db, Memory, MemoryGetInfo, MemoryDeleteInfo, DynamoDbKey};
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
    memories: &HashMap<String, InterpreterMemory>,
) -> Result<Vec<Memory>, EngineError> {
    let mut res = vec![];

    for (_, mem) in memories.iter() {
        res.push(Memory::new(
            &data.client,
            &mem.key,
            Some(encrypt_data(&mem.value)?),
        ));
    }

    Ok(res)
}

pub fn add_memories(
    data: &mut ConversationInfo,
    memories: &HashMap<String, InterpreterMemory>,
) -> Result<(), EngineError> {
    if memories.len() == 0 {
        return Ok(());
    }

    let memories = format_memories(data, memories)?;

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
    index_name: Option<String>,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
    projection_expression: Option<String>,
    expression_attribute_names: Option<HashMap<String, String>>,
    expression_attribute_values: Option<HashMap<String, AttributeValue>>,
    filter_expression: Option<String>,
) -> Result<QueryOutput, EngineError> {
    let input = QueryInput {
        table_name: get_table_name()?,
        index_name,
        key_condition_expression: Some(
            "#hashKey = :hashVal AND begins_with(#rangeKey, :rangePrefix)".to_owned(),
        ),
        expression_attribute_names,
        expression_attribute_values,
        limit: Some(limit),
        exclusive_start_key: pagination_key,
        scan_index_forward: Some(false),
        projection_expression,
        filter_expression,
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = match db.runtime.block_on(future) {
        Ok(data) => data,
        Err(e) => {
            return Err(EngineError::Manager(format!("query_memories {:?}", e)))
        }
    };

    Ok(data)
}

fn get_all_memories(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Vec<serde_json::Value>, EngineError> {
    let mut memories = vec![];
    let mut last_evaluated_key = None;

    let expr_attr_names: HashMap<String, String> = [
        ("#hashKey".to_string(), String::from("hash")),
        ("#rangeKey".to_string(), "range_time".to_owned()),
        ("#key".to_string(), "key".to_string()),
        ("#value".to_string(), "value".to_string()),
        ("#created_at".to_string(), "created_at".to_string()),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values: HashMap<String, AttributeValue> = [
        (":hashVal".to_owned(), AttributeValue {
            s: Some(Memory::get_hash(client)),
            ..Default::default()
        }),
        (":rangePrefix".to_owned(), AttributeValue {
            s: Some(format!("memory#")),
            ..Default::default()
        }),
    ].iter().cloned().collect();

    // retrieve all memories from dynamodb
    loop {
        let data = query_memories(
            Some("TimeIndex".to_owned()),
            db,
            25,
            last_evaluated_key,
            Some("#key, #value, #created_at".to_owned()),
            Some(expr_attr_names.clone()),
            Some(expr_attr_values.clone()),
            None,
        )?;

        match data.items {
            Some(val) => {
                for item in val.iter() {
                    let mem: MemoryGetInfo = serde_dynamodb::from_hashmap(item.to_owned())?;

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

    Ok(memories)
}

pub fn internal_use_get_memories(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let memories = get_all_memories(client, db)?;

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

pub fn get_memories(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let memories = get_all_memories(client, db)?;

    // format memories output
    let mut map = serde_json::Map::new();
    let mut vec = vec![];
    for mem in memories {
        let key = mem["key"].as_str().unwrap();

        if !map.contains_key(key) {
            map.insert(key.to_string(), mem["value"].clone());

            vec.push(mem);
        }
    }

    Ok(serde_json::json!(vec))
}

pub fn get_memory(
    client: &Client,
    key: &str,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, EngineError> {
    let memories = get_all_memories(client, db)?;

    // format memories output
    let mut return_value  = serde_json::Value::Null;
    for mem in memories {
        let val = mem["key"].as_str().unwrap();

        if key == val  {
            return_value = mem;
            break
        }
    }

    Ok(return_value)
}

fn get_memory_batches_to_delete(
    client: &Client,
    db: &mut DynamoDbClient,
    expr_attr_names: Option<HashMap<String, String>>,
    expr_attr_values: Option<HashMap<String, AttributeValue>>,
    filter_expression: Option<String>,
) -> Result<(), EngineError> {
    let mut pagination_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_memories(
            None,
            db,
            25,
            pagination_key,
            Some("#rangeKey".to_owned()),
            expr_attr_names.clone(),
            expr_attr_values.clone(),
            filter_expression.clone(),
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
            let mem: MemoryDeleteInfo = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: Memory::get_hash(client),
                range: mem.range,
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
    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), "range".to_owned()),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values: HashMap<String, AttributeValue> = [
        (":hashVal".to_owned(), AttributeValue {
            s: Some(Memory::get_hash(client)),
            ..Default::default()
        }),
        (":rangePrefix".to_owned(), AttributeValue {
            s: Some(format!("memory#")),
            ..Default::default()
        }),
    ].iter().cloned().collect();

    get_memory_batches_to_delete(
        client,
        db,
        Some(expr_attr_names),
        Some(expr_attr_values),
        None,
    )
}

pub fn delete_client_memory(
    client: &Client,
    key: &str,
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), "range".to_owned()),
        (String::from("#key"), "key".to_owned()),
    ]
    .iter()
    .cloned()
    .collect();

    let expr_attr_values: HashMap<String, AttributeValue> = [
        (":hashVal".to_owned(), AttributeValue {
            s: Some(Memory::get_hash(client)),
            ..Default::default()
        }),
        (":rangePrefix".to_owned(), AttributeValue {
            s: Some(format!("memory#{}", key)),
            ..Default::default()
        }),
        (":key".to_owned(), AttributeValue {
            s: Some(key.to_owned()),
            ..Default::default()
        })
    ].iter().cloned().collect();

    let filter_expr = Some(format!("#key = :key"));

    get_memory_batches_to_delete(
        client,
        db,
        Some(expr_attr_names),
        Some(expr_attr_values),
        filter_expr,
    )
}