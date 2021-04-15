use crate::db_connectors::dynamodb::{get_db, Message, DynamoDbClient, DynamoDbKey};
use crate::{encrypt::encrypt_data, ConversationInfo, EngineError, Client};
use rusoto_dynamodb::*;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<Vec<Message>, EngineError> {
    let mut res = vec![];

    for (i, message) in messages.iter().enumerate() {
        res.push(Message::new(
            &data.client,
            &data.conversation_id,
            &data.interaction_id,
            &data.context.flow,
            &data.context.step,
            direction,
            interaction_order,
            i as i32,
            &encrypt_data(&message["payload"])?,
            &message["payload"]["content_type"].to_string(),
        ));
    }

    Ok(res)
}

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
) -> Result<(), EngineError> {
    if messages.len() == 0 {
        return Ok(());
    }

    let messages = format_messages(data, messages, interaction_order, direction)?;

    // We can only use BatchWriteItem on up to 25 items at once,
    // so we need to split the messages to write into chunks of max
    // 25 items.
    for chunk in messages.chunks(25) {
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

        let db = get_db(&mut data.db)?;
        let future = db.client.batch_write_item(input);

        db.runtime.block_on(future)?;
    }

    Ok(())
}

fn query_messages(
    client: &Client,
    db: &mut DynamoDbClient,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<QueryOutput, EngineError> {
    let hash = Message::get_hash(client);

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
                s: Some(String::from("message#")),
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
        key_condition_expression: Some(
            "#hashKey = :hashVal and begins_with(#rangeKey, :rangePrefix)".to_owned(),
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

fn get_message_batches_to_delete(
    client: &Client,
    db: &mut DynamoDbClient,
) -> Result<Vec<Vec<WriteRequest>>, EngineError> {
    let mut batches = vec![];
    let mut pagination_key = None;

    // retrieve all memories from dynamodb
    loop {
        let data = query_messages(client, db, 25, pagination_key)?;

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
            let message: Message = serde_dynamodb::from_hashmap(item.to_owned())?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: Message::get_hash(client),
                range: Message::get_range(&message.conversation_id, &message.id),
            })?;

            write_requests.push(WriteRequest {
                delete_request: Some(DeleteRequest {key}),
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

pub fn delete_user_messages(client: &Client, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let batches = get_message_batches_to_delete(client, db)?;

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