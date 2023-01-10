use crate::db_connectors::dynamodb::{
    get_db, DynamoDbClient, DynamoDbKey, Message, MessageFromDateInfo, MessageKeys,
};
use crate::{data::EngineError, encrypt::encrypt_data, Client, ConversationInfo};
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use rusoto_dynamodb::*;
use std::collections::HashMap;

use crate::db_connectors::dynamodb::utils::*;

fn format_messages(
    data: &ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
    expires_at: Option<i64>,
) -> Result<Vec<Message>, EngineError> {
    let mut res = vec![];

    for (i, message) in messages.iter().enumerate() {
        res.push(Message::new(
            &data.client,
            &data.conversation_id,
            &data.context.flow,
            &data.context.step.get_step(),
            direction,
            interaction_order,
            i as i32,
            &encrypt_data(&message)?,
            &message["content_type"].to_string(),
            expires_at,
        ));
    }

    Ok(res)
}

pub fn write_messages_batch(
    messages: &[Message],
    db: &mut DynamoDbClient,
) -> Result<(), EngineError> {
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

        let future = db.client.batch_write_item(input);

        db.runtime.block_on(future)?;
    }

    Ok(())
}

pub fn add_messages_bulk(
    data: &mut ConversationInfo,
    messages: &[serde_json::Value],
    interaction_order: i32,
    direction: &str,
    expires_at: Option<i64>,
) -> Result<(), EngineError> {
    if messages.len() == 0 {
        return Ok(());
    }

    let messages = format_messages(data, messages, interaction_order, direction, expires_at)?;
    let db = get_db(&mut data.db)?;

    write_messages_batch(&messages, db)
}

fn query_messages(
    client: &Client,
    db: &mut DynamoDbClient,
    range: String,
    index_name: Option<String>,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
    expression_attribute_names: Option<HashMap<String, String>>,
    key_condition_expression: Option<String>,
    projection_expression: Option<String>,
) -> Result<QueryOutput, EngineError> {
    let hash = Message::get_hash(client);

    let expr_attr_values: HashMap<String, AttributeValue> = [
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
                s: Some(format!("{}", range)),
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
        key_condition_expression,
        expression_attribute_names,
        expression_attribute_values: Some(expr_attr_values),
        limit: Some(limit),
        exclusive_start_key: pagination_key,
        scan_index_forward: Some(false),
        select: Some(String::from("SPECIFIC_ATTRIBUTES")),
        projection_expression,
        ..Default::default()
    };

    let future = db.client.query(input);
    let data = match db.runtime.block_on(future) {
        Ok(data) => data,
        Err(e) => return Err(EngineError::Manager(format!("query_messages {:?}", e))),
    };

    Ok(data)
}

fn query_messages_from_date(
    db: &mut DynamoDbClient,
    range: String,
    index_name: Option<String>,
    limit: i64,
    pagination_key: Option<HashMap<String, AttributeValue>>,
    projection_expression: Option<String>,
    expression_attribute_names: Option<HashMap<String, String>>,
    from_date: i64,
    _to_date: Option<i64>,
) -> Result<QueryOutput, EngineError> {
    let from_date = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(from_date, 0).unwrap(), Utc);
    // let to_date = match to_date {
    //     Some(to_date) => {
    //         DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(to_date, 0), Utc)
    //     }
    //     None => chrono::Utc::now(),
    // };

    let expr_attr_values: HashMap<String, AttributeValue> = [
        (
            String::from(":hashVal"),
            AttributeValue {
                s: Some(range),
                ..Default::default()
            },
        ),
        (
            String::from(":fromDateTime"),
            AttributeValue {
                s: Some(from_date.to_rfc3339_opts(SecondsFormat::Millis, true)),
                ..Default::default()
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let key_condition_expression = "#hashKey = :hashVal and #rangeKey >= :fromDateTime".to_owned();

    let input = QueryInput {
        table_name: get_table_name()?,
        index_name,
        key_condition_expression: Some(key_condition_expression),
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
        Err(e) => return Err(EngineError::Manager(format!("query_messages {:?}", e))),
    };

    Ok(data)
}

pub fn get_client_messages(
    client: &Client,
    db: &mut DynamoDbClient,
    limit: Option<i64>,
    pagination_key: Option<HashMap<String, AttributeValue>>,
) -> Result<serde_json::Value, EngineError> {
    let limit = match limit {
        Some(limit) if limit >= 1 => limit,
        Some(_limit) => 20,
        None => 20,
    };

    let key_condition_expression =
        "#hashKey = :hashVal and begins_with(#rangeTimeKey, :rangePrefix)".to_owned();

    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
        (String::from("#rangeTimeKey"), String::from("range_time")),
    ]
    .iter()
    .cloned()
    .collect();

    let data = query_messages(
        client,
        db,
        String::from("message#"),
        Some(String::from("TimeIndex")),
        limit,
        pagination_key,
        Some(expr_attr_names),
        Some(key_condition_expression),
        Some(String::from("#rangeKey, #hashKey")),
    )?;

    // The query returns an array of items (max 10, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    // , "last_key": :
    let items = match data.items {
        None => return Ok(serde_json::json!({"messages": []})),
        Some(items) if items.len() == 0 => return Ok(serde_json::json!({"messages": []})),
        Some(items) => items.clone(),
    };

    let mut get_requests = vec![];

    for item in items {
        let message: MessageKeys = serde_dynamodb::from_hashmap(item)?;

        let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
            hash: message.hash,
            range: message.range,
        })?;

        get_requests.push(key);
    }

    let request_items = [(get_table_name()?, get_requests)]
        .iter()
        .cloned()
        .map(|(name, keys)| {
            let mut attval = KeysAndAttributes::default();

            attval.keys = keys;

            (name, attval)
        })
        .collect();

    let input = BatchGetItemInput {
        request_items,
        ..Default::default()
    };

    let messages = execute_messages_batch_get_query(db, input)?;

    match data.last_evaluated_key {
        Some(pagination_key) => {
            let pagination_key = base64::encode(serde_json::json!(pagination_key).to_string());

            Ok(serde_json::json!({"messages": messages, "pagination_key": pagination_key}))
        }
        None => Ok(serde_json::json!({ "messages": messages })),
    }
}

pub fn get_client_messages_from_date(
    db: &mut DynamoDbClient,
    limit: Option<i64>,
    pagination_key: Option<HashMap<String, AttributeValue>>,
    from_date: i64,
    to_date: Option<i64>,
) -> Result<serde_json::Value, EngineError> {
    let mut messages = vec![];
    let limit = match limit {
        Some(limit) if limit >= 1 => limit,
        Some(_limit) => 20,
        None => 20,
    };

    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("class")),
        (String::from("#rangeKey"), String::from("created_at")), // time index
    ]
    .iter()
    .cloned()
    .collect();

    let data = query_messages_from_date(
        db,
        String::from("message"),
        Some(String::from("CreatedIndex")),
        limit,
        pagination_key,
        None,
        Some(expr_attr_names.clone()),
        from_date,
        to_date,
    )?;

    // The query returns an array of items (max 10, based on the limit param above).
    // If 0 item is returned it means that there is no open conversation, so simply return None
    // , "last_key": :
    let items = match data.items {
        None => return Ok(serde_json::json!({"messages": []})),
        Some(items) if items.len() == 0 => return Ok(serde_json::json!({"messages": []})),
        Some(items) => items.clone(),
    };

    let mut get_requests = vec![];

    for item in items {
        let message: MessageFromDateInfo = serde_dynamodb::from_hashmap(item)?;

        let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
            hash: message.hash,
            range: message.range,
        })?;

        get_requests.push(key);
    }

    let request_items = [(get_table_name()?, get_requests)]
        .iter()
        .cloned()
        .map(|(name, keys)| {
            let mut attval = KeysAndAttributes::default();

            attval.keys = keys;

            (name, attval)
        })
        .collect();

    let input = BatchGetItemInput {
        request_items,
        ..Default::default()
    };

    let mut get_messages = execute_messages_batch_get_query(db, input)?;
    messages.append(&mut get_messages);

    match data.last_evaluated_key {
        Some(pagination_key) => {
            let pagination_key = base64::encode(serde_json::json!(pagination_key).to_string());

            return Ok(serde_json::json!({"messages": messages, "pagination_key": pagination_key}));
        }
        None => return Ok(serde_json::json!({ "messages": messages })),
    }
}

pub fn delete_user_messages(client: &Client, db: &mut DynamoDbClient) -> Result<(), EngineError> {
    let mut pagination_key = None;

    let key_condition_expression =
        "#hashKey = :hashVal and begins_with(#rangeKey, :rangePrefix)".to_owned();

    let expr_attr_names: HashMap<String, String> = [
        (String::from("#hashKey"), String::from("hash")),
        (String::from("#rangeKey"), String::from("range")),
    ]
    .iter()
    .cloned()
    .collect();

    // retrieve all memories from dynamodb
    loop {
        let data = query_messages(
            client,
            db,
            String::from("message#"),
            None,
            25,
            pagination_key,
            Some(expr_attr_names.clone()),
            Some(key_condition_expression.clone()),
            Some("#hashKey, #rangeKey".to_owned()),
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
            let message: MessageKeys = serde_dynamodb::from_hashmap(item)?;

            let key = serde_dynamodb::to_hashmap(&DynamoDbKey {
                hash: message.hash,
                range: message.range,
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
