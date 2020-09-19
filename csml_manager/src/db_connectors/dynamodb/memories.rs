use crate::{Client, ManagerError, ConversationInfo, encrypt::{encrypt_data, decrypt_data}};
use crate::db_connectors::dynamodb::{Memory, get_db};
use rusoto_dynamodb::*;
use csml_interpreter::data::Memories as InterpreterMemory;
use crate::data::DynamoDbClient;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

fn format_memories(
    data: &ConversationInfo,
    memories: &[InterpreterMemory],
    interaction_order: i32,
) -> Result<Vec<Memory>, ManagerError> {

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
    interaction_order: i32
) -> Result<(), ManagerError> {

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
        };

        request_items.insert(
            get_table_name()?,
            items_to_write,
        );

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

struct QueryResult {
    last_evaluated_key: Option<HashMap<String, AttributeValue>>,
    items: Vec<serde_json::Value>
}

fn query_memories(
    client: &Client,
    db: &mut DynamoDbClient,
    last_evaluated_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryResult, ManagerError> {

    let hash = Memory::get_hash(client);

    let expr_attr_names = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range_time")) // time index
    ].iter().cloned().collect();

    let expr_attr_values = [
        (String::from(":hashVal"), AttributeValue { s: Some(hash), ..Default::default() }),
        (String::from(":rangePrefix"), AttributeValue { s: Some(String::from("memory#")), ..Default::default() }),
    ].iter().cloned().collect();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name: Some(String::from("TimeIndex")),
        key_condition_expression: Some("#hashKey = :hashVal and begins_with(#rangeKey, :rangePrefix)".to_owned()),
        expression_attribute_names: Some(expr_attr_names),
        expression_attribute_values: Some(expr_attr_values),
        exclusive_start_key: last_evaluated_key,
        scan_index_forward: Some(true),
        select: Some(String::from("ALL_ATTRIBUTES")),
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = db.runtime.block_on(future)?;

    let mut items = vec![];
    match data.items {
        Some(val) => {
            for item in val.iter() {
                let mem: Memory = serde_dynamodb::from_hashmap(item.to_owned())?;

                let mut clean = serde_json::json!(mem);
                clean["value"] = decrypt_data(clean["value"].as_str().unwrap().to_string())?;
                items.push(clean);
            }
        },
        _ => (),
    };

    Ok(QueryResult {
        last_evaluated_key: data.last_evaluated_key,
        items,
    })

}

pub fn get_memories(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<serde_json::Value, ManagerError> {

    let mut memories = vec![];
    let mut last_evaluated_key = None;

    // recursively retrieve all memories from dynamodb
    loop {
        let tmp = query_memories(client, db, last_evaluated_key)?;
        let mut items = tmp.items.to_owned();
        memories.append(&mut items);
        if let None = tmp.last_evaluated_key {
            break;
        }
        last_evaluated_key = tmp.last_evaluated_key;
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
